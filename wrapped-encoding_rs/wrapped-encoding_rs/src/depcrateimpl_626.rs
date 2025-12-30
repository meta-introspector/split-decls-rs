// Generated macro for impl_626 (impl)
macro_rules! Depcrateimpl_626 {
() => {
// Module: crate
// Provides: {"impl_626"}
// Dependencies: {}
impl EncoderResult { fn unmappable_from_bmp (bmp : u16) -> EncoderResult { EncoderResult :: Unmappable (:: core :: char :: from_u32 (u32 :: from (bmp)) . unwrap ()) } }
};
}
