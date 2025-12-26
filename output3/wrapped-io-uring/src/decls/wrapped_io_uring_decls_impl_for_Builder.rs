use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl<S: squeue::EntryMarker, C: cqueue::EntryMarker> Builder<S, C> {
    /// Do not make this io_uring instance accessible by child processes after a fork.
    pub fn dontfork(&mut self) -> &mut Self {
        self.dontfork = true;
        self
    }
    /// Perform busy-waiting for I/O completion events, as opposed to getting notifications via an
    /// asynchronous IRQ (Interrupt Request). This will reduce latency, but increases CPU usage.
    ///
    /// This is only usable on file systems that support polling and files opened with `O_DIRECT`.
    pub fn setup_iopoll(&mut self) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_IOPOLL;
        self
    }
    /// Use a kernel thread to perform submission queue polling. This allows your application to
    /// issue I/O without ever context switching into the kernel, however it does use up a lot more
    /// CPU. You should use it when you are expecting very large amounts of I/O.
    ///
    /// After `idle` milliseconds, the kernel thread will go to sleep and you will have to wake it up
    /// again with a system call (this is handled by [`Submitter::submit`] and
    /// [`Submitter::submit_and_wait`] automatically).
    ///
    /// Before version 5.11 of the Linux kernel, to successfully use this feature, the application
    /// must register a set of files to be used for IO through io_uring_register(2) using the
    /// IORING_REGISTER_FILES opcode. Failure to do so will result in submitted IO being errored
    /// with EBADF. The presence of this feature can be detected by the IORING_FEAT_SQPOLL_NONFIXED
    /// feature flag. In version 5.11 and later, it is no longer necessary to register files to use
    /// this feature. 5.11 also allows using this as non-root, if the user has the CAP_SYS_NICE
    /// capability. In 5.13 this requirement was also relaxed, and no special privileges are needed
    /// for SQPOLL in newer kernels. Certain stable kernels older than 5.13 may also support
    /// unprivileged SQPOLL.
    pub fn setup_sqpoll(&mut self, idle: u32) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_SQPOLL;
        self.params.sq_thread_idle = idle;
        self
    }
    /// Bind the kernel's poll thread to the specified cpu. This flag is only meaningful when
    /// [`Builder::setup_sqpoll`] is enabled.
    pub fn setup_sqpoll_cpu(&mut self, cpu: u32) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_SQ_AFF;
        self.params.sq_thread_cpu = cpu;
        self
    }
    /// Create the completion queue with the specified number of entries. The value must be greater
    /// than `entries`, and may be rounded up to the next power-of-two.
    pub fn setup_cqsize(&mut self, entries: u32) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_CQSIZE;
        self.params.cq_entries = entries;
        self
    }
    /// Clamp the sizes of the submission queue and completion queue at their maximum values instead
    /// of returning an error when you attempt to resize them beyond their maximum values.
    pub fn setup_clamp(&mut self) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_CLAMP;
        self
    }
    /// Share the asynchronous worker thread backend of this io_uring with the specified io_uring
    /// file descriptor instead of creating a new thread pool.
    pub fn setup_attach_wq(&mut self, fd: RawFd) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_ATTACH_WQ;
        self.params.wq_fd = fd as _;
        self
    }
    /// Start the io_uring instance with all its rings disabled. This allows you to register
    /// restrictions, buffers and files before the kernel starts processing submission queue
    /// events. You are only able to [register restrictions](Submitter::register_restrictions) when
    /// the rings are disabled due to concurrency issues. You can enable the rings with
    /// [`Submitter::register_enable_rings`]. Available since 5.10.
    pub fn setup_r_disabled(&mut self) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_R_DISABLED;
        self
    }
    /// Normally io_uring stops submitting a batch of request, if one of these requests results in
    /// an error. This can cause submission of less than what is expected, if a request ends in
    /// error while being submitted. If the ring is created with this flag, io_uring_enter(2) will
    /// continue submitting requests even if it encounters an error submitting a request. CQEs are
    /// still posted for errored request regardless of whether or not this flag is set at ring
    /// creation time, the only difference is if the submit sequence is halted or continued when an
    /// error is observed. Available since 5.18.
    pub fn setup_submit_all(&mut self) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_SUBMIT_ALL;
        self
    }
    /// By default, io_uring will interrupt a task running in userspace when a completion event
    /// comes in. This is to ensure that completions run in a timely manner. For a lot of use
    /// cases, this is overkill and can cause reduced performance from both the inter-processor
    /// interrupt used to do this, the kernel/user transition, the needless interruption of the
    /// tasks userspace activities, and reduced batching if completions come in at a rapid rate.
    /// Most applications don't need the forceful interruption, as the events are processed at any
    /// kernel/user transition. The exception are setups where the application uses multiple
    /// threads operating on the same ring, where the application waiting on completions isn't the
    /// one that submitted them. For most other use cases, setting this flag will improve
    /// performance. Available since 5.19.
    pub fn setup_coop_taskrun(&mut self) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_COOP_TASKRUN;
        self
    }
    /// Used in conjunction with IORING_SETUP_COOP_TASKRUN, this provides a flag,
    /// IORING_SQ_TASKRUN, which is set in the SQ ring flags whenever completions are pending that
    /// should be processed. As an example, liburing will check for this flag even when doing
    /// io_uring_peek_cqe(3) and enter the kernel to process them, and applications can do the
    /// same. This makes IORING_SETUP_TASKRUN_FLAG safe to use even when applications rely on a
    /// peek style operation on the CQ ring to see if anything might be pending to reap. Available
    /// since 5.19.
    pub fn setup_taskrun_flag(&mut self) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_TASKRUN_FLAG;
        self
    }
    /// By default, io_uring will process all outstanding work at the end of any system call or
    /// thread interrupt. This can delay the application from making other progress. Setting this
    /// flag will hint to io_uring that it should defer work until an io_uring_enter(2) call with
    /// the IORING_ENTER_GETEVENTS flag set. This allows the application to request work to run
    /// just just before it wants to process completions. This flag requires the
    /// IORING_SETUP_SINGLE_ISSUER flag to be set, and also enforces that the call to
    /// io_uring_enter(2) is called from the same thread that submitted requests. Note that if this
    /// flag is set then it is the application's responsibility to periodically trigger work (for
    /// example via any of the CQE waiting functions) or else completions may not be delivered.
    /// Available since 6.1.
    pub fn setup_defer_taskrun(&mut self) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_DEFER_TASKRUN;
        self
    }
    /// Hint the kernel that a single task will submit requests. Used for optimizations. This is
    /// enforced by the kernel, and request that don't respect that will fail with -EEXIST.
    /// If [`Builder::setup_sqpoll`] is enabled, the polling task is doing the submissions and multiple
    /// userspace tasks can call [`Submitter::enter`] and higher level APIs. Available since 6.0.
    pub fn setup_single_issuer(&mut self) -> &mut Self {
        self.params.flags |= sys::IORING_SETUP_SINGLE_ISSUER;
        self
    }
    /// Build an [IoUring], with the specified number of entries in the submission queue and
    /// completion queue unless [`setup_cqsize`](Self::setup_cqsize) has been called.
    pub fn build(&self, entries: u32) -> io::Result<IoUring<S, C>> {
        let ring = IoUring::with_params(entries, self.params)?;
        if self.dontfork {
            ring.memory.sq_mmap.dontfork()?;
            ring.memory.sqe_mmap.dontfork()?;
            if let Some(cq_mmap) = ring.memory.cq_mmap.as_ref() {
                cq_mmap.dontfork()?;
            }
        }
        Ok(ring)
    }
}
