// Generated macro for implementation (module)
macro_rules! Depcrate_platform_uniximplementation {
() => {
// Module: crate::platform::unix
// Provides: {"implementation"}
// Dependencies: {}
# [cfg (target_vendor = "apple")] mod implementation { use dispatch2 :: { DispatchRetained , DispatchSemaphore , DispatchTime } ; static mut SEMAPHORE : Option < DispatchRetained < DispatchSemaphore > > = None ; pub unsafe fn sem_init () { SEMAPHORE = Some (DispatchSemaphore :: new (0)) ; } # [allow (static_mut_refs)] pub unsafe fn sem_post () { SEMAPHORE . as_deref () . unwrap () . signal () ; } # [allow (static_mut_refs)] pub unsafe fn sem_wait_forever () { SEMAPHORE . as_deref () . unwrap () . wait (DispatchTime :: FOREVER) ; } }
};
}
