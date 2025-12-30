// Generated macro for impl_78 (impl)
macro_rules! Depcrate_semaphoreimpl_78 {
() => {
// Module: crate::semaphore
// Provides: {"impl_78"}
// Dependencies: {}
impl Semaphore { # [doc = " Construct a new semaphore."] # [doc = ""] # [doc = " This wraps"] # [doc = " [`CreateSemaphoreW`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-createsemaphorew)."] pub fn new () -> io :: Result < Self > { let handle = nonnull_handle_result (unsafe { CreateSemaphoreW (ptr :: null_mut () , 0 , 1 , ptr :: null_mut ()) }) ? ; let handle = unsafe { Handle :: from_raw (handle) } ; Ok (Self (handle)) } # [doc = " Release a permit on the semaphore."] # [doc = ""] # [doc = " This wraps"] # [doc = " [`ReleaseSemaphore`](https://docs.microsoft.com/en-us/windows/win32/api/synchapi/nf-synchapi-releasesemaphore)."] pub fn release (& self) -> io :: Result < () > { result (unsafe { ReleaseSemaphore (* self . 0 , 1 , ptr :: null_mut ()) }) } # [doc = " Access the underlying handle to the semaphore."] pub fn handle (& self) -> & Handle { & self . 0 } }
};
}
