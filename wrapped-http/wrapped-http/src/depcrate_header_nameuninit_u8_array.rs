// Generated macro for uninit_u8_array (function)
macro_rules! Depcrate_header_nameuninit_u8_array {
() => {
// Module: crate::header::name
// Provides: {"uninit_u8_array"}
// Dependencies: {}
fn uninit_u8_array () -> [MaybeUninit < u8 > ; SCRATCH_BUF_SIZE] { let arr = MaybeUninit :: < [MaybeUninit < u8 > ; SCRATCH_BUF_SIZE] > :: uninit () ; unsafe { arr . assume_init () } }
};
}
