// Generated macro for impl_500 (impl)
macro_rules! Depcrate_threadimpl_500 {
() => {
// Module: crate::thread
// Provides: {"impl_500"}
// Dependencies: {}
impl < T > JoinHandle < T > { # [doc = " Waits for the associated thread to finish."] # [track_caller] pub fn join (self) -> std :: thread :: Result < T > { self . notify . wait (location ! ()) ; self . result . lock () . unwrap () . take () . unwrap () } # [doc = " Gets a handle to the underlying [`Thread`]"] pub fn thread (& self) -> & Thread { & self . thread } }
};
}
