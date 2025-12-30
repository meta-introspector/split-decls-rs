// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < KeySize > StreamCipherCore for Rc4Core < KeySize > { # [inline (always)] fn remaining_blocks (& self) -> Option < usize > { None } fn process_with_backend (& mut self , f : impl StreamCipherClosure < BlockSize = Self :: BlockSize >) { f . call (& mut Backend (& mut self . state)) ; } }
};
}
