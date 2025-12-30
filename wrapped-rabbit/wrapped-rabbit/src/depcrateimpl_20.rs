// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl StreamCipherCore for RabbitKeyOnlyCore { # [inline (always)] fn remaining_blocks (& self) -> Option < usize > { None } fn process_with_backend (& mut self , f : impl StreamCipherClosure < BlockSize = Self :: BlockSize >) { f . call (& mut Backend (& mut self . state)) ; } }
};
}
