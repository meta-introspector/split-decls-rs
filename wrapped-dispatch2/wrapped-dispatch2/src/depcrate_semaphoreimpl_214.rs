// Generated macro for impl_214 (impl)
macro_rules! Depcrate_semaphoreimpl_214 {
() => {
// Module: crate::semaphore
// Provides: {"impl_214"}
// Dependencies: {}
impl DispatchSemaphore { # [doc = " Attempt to acquire the [`DispatchSemaphore`] and return a [`DispatchSemaphoreGuard`]."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Return [WaitError::TimeOverflow] if the passed ``timeout`` is too big."] # [doc = ""] # [doc = " Return [WaitError::Timeout] in case of timeout."] pub fn try_acquire (& self , timeout : DispatchTime) -> Result < DispatchSemaphoreGuard , WaitError > { let result = Self :: wait (self , timeout) ; match result { 0 => Ok (DispatchSemaphoreGuard (self . retain ())) , _ => Err (WaitError :: Timeout) , } } }
};
}
