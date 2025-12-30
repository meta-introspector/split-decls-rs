// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl StreamCipherCore for Hc256Core { # [inline (always)] fn remaining_blocks (& self) -> Option < usize > { None } fn process_with_backend (& mut self , f : impl StreamCipherClosure < BlockSize = Self :: BlockSize >) { f . call (& mut Backend (self)) ; } }
};
}
