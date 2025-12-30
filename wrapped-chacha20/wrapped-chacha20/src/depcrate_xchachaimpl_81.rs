// Generated macro for impl_81 (impl)
macro_rules! Depcrate_xchachaimpl_81 {
() => {
// Module: crate::xchacha
// Provides: {"impl_81"}
// Dependencies: {}
impl < R : Rounds > StreamCipherCore for XChaChaCore < R > { # [inline (always)] fn remaining_blocks (& self) -> Option < usize > { self . 0 . remaining_blocks () } # [inline (always)] fn process_with_backend (& mut self , f : impl StreamCipherClosure < BlockSize = Self :: BlockSize >) { self . 0 . process_with_backend (f) ; } }
};
}
