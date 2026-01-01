# AST Trace: ../rust/compiler/rustc_thread_pool/src/job.rs

Generated 22 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
use std::any::Any;
use std::cell::UnsafeCell;
use std::mem;
use std::sync::Arc;

use crossbeam_deque::{Injector, Steal};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use crate::latch::Latch;
use crate::tlv::Tlv;
use crate::{tlv, unwind};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
pub(super) enum JobResult<T> {
    None,
    Ok(T),
    Panic(Box<dyn Any + Send>),
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=11 | LINES=12

```rust
/// A `Job` is used to advertise work for other threads that they may
/// want to steal. In accordance with time honored tradition, jobs are
/// arranged in a deque, so that thieves can take from the top of the
/// deque while the main worker manages the bottom of the deque. This
/// deque is managed by the `thread_pool` module.
pub(super) trait Job {
    /// Unsafe: this may be called from a different thread than the one
    /// which scheduled the job, so the implementer must ensure the
    /// appropriate traits are met, whether `Send`, `Sync`, or both.
    unsafe fn execute(this: *const ());
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(super) struct JobRefId {
    pointer: usize,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=11

```rust
/// Effectively a Job trait object. Each JobRef **must** be executed
/// exactly once, or else data may leak.
///
/// Internally, we store the job's data in a `*const ()` pointer. The
/// true type is something like `*const StackJob<...>`, but we hide
/// it. We also carry the "execute fn" from the `Job` trait.
pub(super) struct JobRef {
    pointer: *const (),
    execute_fn: unsafe fn(*const ()),
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=2

```rust
unsafe impl Send for JobRef {}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1

```rust
unsafe impl Sync for JobRef {}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=21 | LINES=22

```rust
impl JobRef {
    /// Unsafe: caller asserts that `data` will remain valid until the
    /// job is executed.
    pub(super) unsafe fn new<T>(data: *const T) -> JobRef
    where
        T: Job,
    {
        // erase types:
        JobRef { pointer: data as *const (), execute_fn: <T as Job>::execute }
    }

    #[inline]
    pub(super) fn id(&self) -> JobRefId {
        JobRefId { pointer: self.pointer.expose_provenance() }
    }

    #[inline]
    pub(super) unsafe fn execute(self) {
        unsafe { (self.execute_fn)(self.pointer) }
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=16

```rust
/// A job that will be owned by a stack slot. This means that when it
/// executes it need not free any heap data, the cleanup occurs when
/// the stack frame is later popped. The function parameter indicates
/// `true` if the job was stolen -- executed on a different thread.
pub(super) struct StackJob<L, F, R>
where
    L: Latch + Sync,
    F: FnOnce(bool) -> R + Send,
    R: Send,
{
    pub(super) latch: L,
    func: UnsafeCell<Option<F>>,
    result: UnsafeCell<JobResult<R>>,
    tlv: Tlv,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=34 | LINES=35

```rust
impl<L, F, R> StackJob<L, F, R>
where
    L: Latch + Sync,
    F: FnOnce(bool) -> R + Send,
    R: Send,
{
    pub(super) fn new(tlv: Tlv, func: F, latch: L) -> StackJob<L, F, R> {
        StackJob {
            latch,
            func: UnsafeCell::new(Some(func)),
            result: UnsafeCell::new(JobResult::None),
            tlv,
        }
    }

    pub(super) unsafe fn as_job_ref(&self) -> JobRef {
        unsafe { JobRef::new(self) }
    }

    pub(super) unsafe fn run_inline(&self, stolen: bool) {
        unsafe {
            let func = (*self.func.get()).take().unwrap();
            *(self.result.get()) = match unwind::halt_unwinding(|| func(stolen)) {
                Ok(x) => JobResult::Ok(x),
                Err(x) => JobResult::Panic(x),
            };
            Latch::set(&self.latch);
        }
    }

    pub(super) unsafe fn into_result(self) -> R {
        self.result.into_inner().into_return_value()
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=30 | LINES=21

```rust
impl<L, F, R> Job for StackJob<L, F, R>
where
    L: Latch + Sync,
    F: FnOnce(bool) -> R + Send,
    R: Send,
{
    unsafe fn execute(this: *const ()) {
        let this = unsafe { &*(this as *const Self) };
        tlv::set(this.tlv);
        let abort = unwind::AbortIfPanic;
        let func = unsafe { (*this.func.get()).take().unwrap() };
        unsafe {
            (*this.result.get()) = JobResult::call(func);
        }
        unsafe {
            Latch::set(&this.latch);
        }
        mem::forget(abort);
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=14

```rust
/// Represents a job stored in the heap. Used to implement
/// `scope`. Unlike `StackJob`, when executed, `HeapJob` simply
/// invokes a closure, which then triggers the appropriate logic to
/// signal that the job executed.
///
/// (Probably `StackJob` should be refactored in a similar fashion.)
pub(super) struct HeapJob<BODY>
where
    BODY: FnOnce(JobRefId) + Send,
{
    job: BODY,
    tlv: Tlv,
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=21 | LINES=24

```rust
impl<BODY> HeapJob<BODY>
where
    BODY: FnOnce(JobRefId) + Send,
{
    pub(super) fn new(tlv: Tlv, job: BODY) -> Box<Self> {
        Box::new(HeapJob { job, tlv })
    }

    /// Creates a `JobRef` from this job -- note that this hides all
    /// lifetimes, so it is up to you to ensure that this JobRef
    /// doesn't outlive any data that it closes over.
    pub(super) unsafe fn into_job_ref(self: Box<Self>) -> JobRef {
        unsafe { JobRef::new(Box::into_raw(self)) }
    }

    /// Creates a static `JobRef` from this job.
    pub(super) fn into_static_job_ref(self: Box<Self>) -> JobRef
    where
        BODY: 'static,
    {
        unsafe { self.into_job_ref() }
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=15 | LINES=12

```rust
impl<BODY> Job for HeapJob<BODY>
where
    BODY: FnOnce(JobRefId) + Send,
{
    unsafe fn execute(this: *const ()) {
        let pointer = this.expose_provenance();
        let this = unsafe { Box::from_raw(this as *mut Self) };
        tlv::set(this.tlv);
        (this.job)(JobRefId { pointer });
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
/// Represents a job stored in an `Arc` -- like `HeapJob`, but may
/// be turned into multiple `JobRef`s and called multiple times.
pub(super) struct ArcJob<BODY>
where
    BODY: Fn(JobRefId) + Send + Sync,
{
    job: BODY,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=21 | LINES=24

```rust
impl<BODY> ArcJob<BODY>
where
    BODY: Fn(JobRefId) + Send + Sync,
{
    pub(super) fn new(job: BODY) -> Arc<Self> {
        Arc::new(ArcJob { job })
    }

    /// Creates a `JobRef` from this job -- note that this hides all
    /// lifetimes, so it is up to you to ensure that this JobRef
    /// doesn't outlive any data that it closes over.
    pub(super) unsafe fn as_job_ref(this: &Arc<Self>) -> JobRef {
        unsafe { JobRef::new(Arc::into_raw(Arc::clone(this))) }
    }

    /// Creates a static `JobRef` from this job.
    pub(super) fn as_static_job_ref(this: &Arc<Self>) -> JobRef
    where
        BODY: 'static,
    {
        unsafe { Self::as_job_ref(this) }
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=15 | LINES=11

```rust
impl<BODY> Job for ArcJob<BODY>
where
    BODY: Fn(JobRefId) + Send + Sync,
{
    unsafe fn execute(this: *const ()) {
        let pointer = this.expose_provenance();
        let this = unsafe { Arc::from_raw(this as *mut Self) };
        (this.job)(JobRefId { pointer });
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=call | COMPLEXITY=17 | LINES=21

```rust
impl<T> JobResult<T> {
    fn call(func: impl FnOnce(bool) -> T) -> Self {
        match unwind::halt_unwinding(|| func(true)) {
            Ok(x) => JobResult::Ok(x),
            Err(x) => JobResult::Panic(x),
        }
    }

    /// Convert the `JobResult` for a job that has finished (and hence
    /// its JobResult is populated) into its return value.
    ///
    /// NB. This will panic if the job panicked.
    pub(super) fn into_return_value(self) -> T {
        match self {
            JobResult::None => unreachable!(),
            JobResult::Ok(x) => x,
            JobResult::Panic(x) => unwind::resume_unwinding(x),
        }
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
/// Indirect queue to provide FIFO job priority.
pub(super) struct JobFifo {
    inner: Injector<JobRef>,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=15 | LINES=14

```rust
impl JobFifo {
    pub(super) fn new() -> Self {
        JobFifo { inner: Injector::new() }
    }

    pub(super) unsafe fn push(&self, job_ref: JobRef) -> JobRef {
        // A little indirection ensures that spawns are always prioritized in FIFO order. The
        // jobs in a thread's deque may be popped from the back (LIFO) or stolen from the front
        // (FIFO), but either way they will end up popping from the front of this queue.
        self.inner.push(job_ref);
        unsafe { JobRef::new(self) }
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=27 | LINES=14

```rust
impl Job for JobFifo {
    unsafe fn execute(this: *const ()) {
        // We "execute" a queue by executing its first job, FIFO.
        let this = unsafe { &*(this as *const Self) };
        loop {
            match this.inner.steal() {
                Steal::Success(job_ref) => break unsafe { job_ref.execute() },
                Steal::Empty => panic!("FIFO is empty"),
                Steal::Retry => {}
            }
        }
    }
}
```

---
*Generated by AST tracing system*
