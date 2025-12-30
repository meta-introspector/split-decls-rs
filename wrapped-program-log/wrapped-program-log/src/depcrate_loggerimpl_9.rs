// Generated macro for impl_9 (impl)
macro_rules! Depcrate_loggerimpl_9 {
() => {
// Module: crate::logger
// Provides: {"impl_9"}
// Dependencies: {}
impl < const BUFFER : usize > Deref for Logger < BUFFER > { type Target = [u8] ; fn deref (& self) -> & Self :: Target { unsafe { from_raw_parts (self . buffer . as_ptr () as * const _ , self . len) } } }
};
}
