macro_rules! deps {
    () => {
        JobId!();
        FnContext!();
        StackJob!();
        SpinLatch!();
    };
}

macro_rules! join_context {
    () => {
        deps!();
        # [doc = " Identical to `join`, except that the closures have a parameter"] # [doc = " that provides context for the way the closure has been called,"] # [doc = " especially indicating whether they're executing on a different"] # [doc = " thread than where `join_context` was called.  This will occur if"] # [doc = " the second job is stolen by a different thread, or if"] # [doc = " `join_context` was called from outside the thread pool to begin"] # [doc = " with."] pub fn join_context < A , B , RA , RB > (oper_a : A , oper_b : B) -> (RA , RB) where A : FnOnce (FnContext) -> RA + Send , B : FnOnce (FnContext) -> RB + Send , RA : Send , RB : Send , { # [inline] fn call_a < R > (f : impl FnOnce (FnContext) -> R , injected : bool) -> impl FnOnce () -> R { move | | f (FnContext :: new (injected)) } # [inline] fn call_b < R > (f : impl FnOnce (FnContext) -> R) -> impl FnOnce (bool) -> R { move | migrated | f (FnContext :: new (migrated)) } registry :: in_worker (| worker_thread , injected | unsafe { let job_b = StackJob :: new (call_b (oper_b) , SpinLatch :: new (worker_thread)) ; let job_b_ref = job_b . as_job_ref () ; let job_b_id : JobId = job_b_ref . id () ; worker_thread . push (job_b_ref) ; let status_a = unwind :: halt_unwinding (call_a (oper_a , injected)) ; let result_a = match status_a { Ok (v) => v , Err (err) => join_recover_from_panic (worker_thread , & job_b . latch , err) , } ; while ! job_b . latch . probe () { let Some (job) = worker_thread . take_local_job () else { worker_thread . wait_until (& job_b . latch) ; debug_assert ! (job_b . latch . probe ()) ; break ; } ; let job_id = job . id () ; if job_b_id == job_id { let result_b = job_b . run_inline (injected) ; return (result_a , result_b) ; } worker_thread . execute (job) ; } (result_a , job_b . into_result ()) }) }
    };
}

join_context!()