// Generated macro for impl_501 (impl)
macro_rules! Depcrate_rt_tokioimpl_501 {
() => {
// Module: crate::rt::tokio
// Provides: {"impl_501"}
// Dependencies: {}
impl Timer for TokioTimer { fn sleep (& self , duration : Duration) -> Pin < Box < dyn Sleep > > { Box :: pin (TokioSleep { inner : tokio :: time :: sleep (duration) , }) } fn sleep_until (& self , deadline : Instant) -> Pin < Box < dyn Sleep > > { Box :: pin (TokioSleep { inner : tokio :: time :: sleep_until (deadline . into ()) , }) } fn reset (& self , sleep : & mut Pin < Box < dyn Sleep > > , new_deadline : Instant) { if let Some (sleep) = sleep . as_mut () . downcast_mut_pin :: < TokioSleep > () { sleep . reset (new_deadline) } } fn now (& self) -> Instant { tokio :: time :: Instant :: now () . into () } }
};
}
