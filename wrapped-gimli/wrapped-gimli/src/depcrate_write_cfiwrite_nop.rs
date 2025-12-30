// Generated macro for write_nop (function)
macro_rules! Depcrate_write_cfiwrite_nop {
() => {
// Module: crate::write::cfi
// Provides: {"write_nop"}
// Dependencies: {}
fn write_nop < W : Writer > (w : & mut W , len : usize , align : u8) -> Result < () > { debug_assert_eq ! (align & (align - 1) , 0) ; let tail_len = (! len + 1) & (align as usize - 1) ; for _ in 0 .. tail_len { w . write_u8 (constants :: DW_CFA_nop . 0) ? ; } Ok (()) }
};
}
