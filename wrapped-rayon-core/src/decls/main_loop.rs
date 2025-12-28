macro_rules! deps {
    () => {
        Latch!();
        AbortIfPanic!();
        ThreadBuilder!();
        WorkerThread!();
    };
}

macro_rules! main_loop {
    () => {
        deps!();
        unsafe fn main_loop (thread : ThreadBuilder) { unsafe { let worker_thread = & WorkerThread :: from (thread) ; WorkerThread :: set_current (worker_thread) ; let registry = & * worker_thread . registry ; let index = worker_thread . index ; Latch :: set (& registry . thread_infos [index] . primed) ; let abort_guard = unwind :: AbortIfPanic ; if let Some (ref handler) = registry . start_handler { registry . catch_unwind (| | handler (index)) ; } worker_thread . wait_until_out_of_work () ; mem :: forget (abort_guard) ; if let Some (ref handler) = registry . exit_handler { registry . catch_unwind (| | handler (index)) ; } } }
    };
}

main_loop!()