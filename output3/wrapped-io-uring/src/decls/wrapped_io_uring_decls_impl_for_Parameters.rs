use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Parameters {
    /// Whether a kernel thread is performing queue polling. Enabled with [`Builder::setup_sqpoll`].
    pub fn is_setup_sqpoll(&self) -> bool {
        self.0.flags & sys::IORING_SETUP_SQPOLL != 0
    }
    /// Whether waiting for completion events is done with a busy loop instead of using IRQs.
    /// Enabled with [`Builder::setup_iopoll`].
    pub fn is_setup_iopoll(&self) -> bool {
        self.0.flags & sys::IORING_SETUP_IOPOLL != 0
    }
    /// Whether the single issuer hint is enabled. Enabled with [`Builder::setup_single_issuer`].
    pub fn is_setup_single_issuer(&self) -> bool {
        self.0.flags & sys::IORING_SETUP_SINGLE_ISSUER != 0
    }
    /// If this flag is set, the SQ and CQ rings were mapped with a single `mmap(2)` call. This
    /// means that only two syscalls were used instead of three.
    pub fn is_feature_single_mmap(&self) -> bool {
        self.0.features & sys::IORING_FEAT_SINGLE_MMAP != 0
    }
    /// If this flag is set, io_uring supports never dropping completion events. If a completion
    /// event occurs and the CQ ring is full, the kernel stores the event internally until such a
    /// time that the CQ ring has room for more entries.
    pub fn is_feature_nodrop(&self) -> bool {
        self.0.features & sys::IORING_FEAT_NODROP != 0
    }
    /// If this flag is set, applications can be certain that any data for async offload has been
    /// consumed when the kernel has consumed the SQE.
    pub fn is_feature_submit_stable(&self) -> bool {
        self.0.features & sys::IORING_FEAT_SUBMIT_STABLE != 0
    }
    /// If this flag is set, applications can specify offset == -1 with [`Readv`](opcode::Readv),
    /// [`Writev`](opcode::Writev), [`ReadFixed`](opcode::ReadFixed),
    /// [`WriteFixed`](opcode::WriteFixed), [`Read`](opcode::Read) and [`Write`](opcode::Write),
    /// which behaves exactly like setting offset == -1 in `preadv2(2)` and `pwritev2(2)`: it’ll use
    /// (and update) the current file position.
    ///
    /// This obviously comes with the caveat that if the application has multiple reads or writes in flight,
    /// then the end result will not be as expected.
    /// This is similar to threads sharing a file descriptor and doing IO using the current file position.
    pub fn is_feature_rw_cur_pos(&self) -> bool {
        self.0.features & sys::IORING_FEAT_RW_CUR_POS != 0
    }
    /// If this flag is set, then io_uring guarantees that both sync and async execution of
    /// a request assumes the credentials of the task that called [`Submitter::enter`] to queue the requests.
    /// If this flag isn’t set, then requests are issued with the credentials of the task that originally registered the io_uring.
    /// If only one task is using a ring, then this flag doesn’t matter as the credentials will always be the same.
    ///
    /// Note that this is the default behavior, tasks can still register different personalities
    /// through [`Submitter::register_personality`].
    pub fn is_feature_cur_personality(&self) -> bool {
        self.0.features & sys::IORING_FEAT_CUR_PERSONALITY != 0
    }
    /// Whether async pollable I/O is fast.
    ///
    /// See [the commit message that introduced
    /// it](https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/commit/?id=d7718a9d25a61442da8ee8aeeff6a0097f0ccfd6)
    /// for more details.
    ///
    /// If this flag is set, then io_uring supports using an internal poll mechanism to drive
    /// data/space readiness. This means that requests that cannot read or write data to a file no
    /// longer need to be punted to an async thread for handling, instead they will begin operation
    /// when the file is ready. This is similar to doing poll + read/write in userspace, but
    /// eliminates the need to do so. If this flag is set, requests waiting on space/data consume a
    /// lot less resources doing so as they are not blocking a thread. Available since kernel 5.7.
    pub fn is_feature_fast_poll(&self) -> bool {
        self.0.features & sys::IORING_FEAT_FAST_POLL != 0
    }
    /// Whether poll events are stored using 32 bits instead of 16. This allows the user to use
    /// `EPOLLEXCLUSIVE`.
    ///
    /// If this flag is set, the IORING_OP_POLL_ADD command accepts the full 32-bit range of epoll
    /// based flags. Most notably EPOLLEXCLUSIVE which allows exclusive (waking single waiters)
    /// behavior. Available since kernel 5.9.
    pub fn is_feature_poll_32bits(&self) -> bool {
        self.0.features & sys::IORING_FEAT_POLL_32BITS != 0
    }
    /// If this flag is set, the IORING_SETUP_SQPOLL feature no longer requires the use of fixed
    /// files. Any normal file descriptor can be used for IO commands without needing registration.
    /// Available since kernel 5.11.
    pub fn is_feature_sqpoll_nonfixed(&self) -> bool {
        self.0.features & sys::IORING_FEAT_SQPOLL_NONFIXED != 0
    }
    /// If this flag is set, then the io_uring_enter(2) system call supports passing in an extended
    /// argument instead of just the sigset_t of earlier kernels. This extended argument is of type
    /// struct io_uring_getevents_arg and allows the caller to pass in both a sigset_t and a
    /// timeout argument for waiting on events. The struct layout is as follows:
    ///
    /// // struct io_uring_getevents_arg {
    /// //     __u64 sigmask;
    /// //     __u32 sigmask_sz;
    /// //     __u32 pad;
    /// //     __u64 ts;
    /// // };
    ///
    /// and a pointer to this struct must be passed in if IORING_ENTER_EXT_ARG is set in the flags
    /// for the enter system call. Available since kernel 5.11.
    pub fn is_feature_ext_arg(&self) -> bool {
        self.0.features & sys::IORING_FEAT_EXT_ARG != 0
    }
    /// If this flag is set, io_uring is using native workers for its async helpers. Previous
    /// kernels used kernel threads that assumed the identity of the original io_uring owning task,
    /// but later kernels will actively create what looks more like regular process threads
    /// instead. Available since kernel 5.12.
    pub fn is_feature_native_workers(&self) -> bool {
        self.0.features & sys::IORING_FEAT_NATIVE_WORKERS != 0
    }
    /// Whether the kernel supports tagging resources.
    ///
    /// If this flag is set, then io_uring supports a variety of features related to fixed files
    /// and buffers. In particular, it indicates that registered buffers can be updated in-place,
    /// whereas before the full set would have to be unregistered first. Available since kernel
    /// 5.13.
    pub fn is_feature_resource_tagging(&self) -> bool {
        self.0.features & sys::IORING_FEAT_RSRC_TAGS != 0
    }
    /// Whether the kernel supports `IOSQE_CQE_SKIP_SUCCESS`.
    ///
    /// This feature allows skipping the generation of a CQE if a SQE executes normally. Available
    /// since kernel 5.17.
    pub fn is_feature_skip_cqe_on_success(&self) -> bool {
        self.0.features & sys::IORING_FEAT_CQE_SKIP != 0
    }
    /// Whether the kernel supports deferred file assignment.
    ///
    /// If this flag is set, then io_uring supports sane assignment of files for SQEs that have
    /// dependencies. For example, if a chain of SQEs are submitted with IOSQE_IO_LINK, then
    /// kernels without this flag will prepare the file for each link upfront. If a previous link
    /// opens a file with a known index, eg if direct descriptors are used with open or accept,
    /// then file assignment needs to happen post execution of that SQE. If this flag is set, then
    /// the kernel will defer file assignment until execution of a given request is started.
    /// Available since kernel 5.17.
    pub fn is_feature_linked_file(&self) -> bool {
        self.0.features & sys::IORING_FEAT_LINKED_FILE != 0
    }
    /// Whether the kernel supports `IORING_RECVSEND_BUNDLE`.
    ///
    /// This feature allows sending and recieving multiple buffers as a single bundle. Available
    /// since kernel 6.10.
    pub fn is_feature_recvsend_bundle(&self) -> bool {
        self.0.features & sys::IORING_FEAT_RECVSEND_BUNDLE != 0
    }
    /// If this flag is set, applications can use the
    /// [`SubmitArgs::min_wait_usec`](types::SubmitArgs::min_wait_usec) method to specify a timeout
    /// after which the kernel will return as soon as a single completion is received instead of
    /// waiting for the minimum specified by the application. Available since kernel 6.12.
    pub fn is_feature_min_timeout(&self) -> bool {
        self.0.features & sys::IORING_FEAT_MIN_TIMEOUT != 0
    }
    /// The number of submission queue entries allocated.
    pub fn sq_entries(&self) -> u32 {
        self.0.sq_entries
    }
    /// The idle time of the SQ poll thread in milliseconds.
    pub fn sq_thread_idle(&self) -> u32 {
        self.0.sq_thread_idle
    }
    /// The number of completion queue entries allocated.
    pub fn cq_entries(&self) -> u32 {
        self.0.cq_entries
    }
}
