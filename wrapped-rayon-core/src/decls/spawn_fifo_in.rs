macro_rules! deps {
    () => {
        AbortIfPanic!();
        Registry!();
    };
}

macro_rules! spawn_fifo_in {
    () => {
        deps!();
        # [doc = " Spawns an asynchronous FIFO job in `registry.`"] # [doc = ""] # [doc = " Unsafe because `registry` must not yet have terminated."] pub (super) unsafe fn spawn_fifo_in < F > (func : F , registry : & Arc < Registry >) where F : FnOnce () + Send + 'static , { unsafe { let abort_guard = unwind :: AbortIfPanic ; let job_ref = spawn_job (func , registry) ; match registry . current_thread () { Some (worker) => worker . push_fifo (job_ref) , None => registry . inject (job_ref) , } mem :: forget (abort_guard) ; } }
    };
}

spawn_fifo_in!();