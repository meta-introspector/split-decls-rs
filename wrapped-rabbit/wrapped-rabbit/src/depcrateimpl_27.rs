// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl StreamCipherCore for RabbitCore { # [inline (always)] fn remaining_blocks (& self) -> Option < usize > { None } fn process_with_backend (& mut self , f : impl StreamCipherClosure < BlockSize = Self :: BlockSize >) { f . call (& mut Backend (& mut self . state)) ; } }
};
}
