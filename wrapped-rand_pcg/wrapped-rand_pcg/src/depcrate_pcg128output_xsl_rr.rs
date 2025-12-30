// Generated macro for output_xsl_rr (function)
macro_rules! Depcrate_pcg128output_xsl_rr {
() => {
// Module: crate::pcg128
// Provides: {"output_xsl_rr"}
// Dependencies: {}
# [inline (always)] fn output_xsl_rr (state : u128) -> u64 { const XSHIFT : u32 = 64 ; const ROTATE : u32 = 122 ; let rot = (state >> ROTATE) as u32 ; let xsl = ((state >> XSHIFT) as u64) ^ (state as u64) ; xsl . rotate_right (rot) }
};
}
