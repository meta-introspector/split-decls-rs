// Generated macro for edge_cases (macro)
macro_rules! Depcrateedge_cases {
() => {
// Module: crate
// Provides: {"edge_cases"}
// Dependencies: {}
macro_rules ! edge_cases { ($ I : ident , $ case : ident , $ inner : block) => { for i0 in 0 ..$ I :: FUZZ_NUM { let mask_lo = (!$ I :: Unsigned :: ZERO) . wrapping_shr ($ I :: FUZZ_LENGTHS [i0] as u32) ; for i1 in i0 .. I :: FUZZ_NUM { let mask_hi = (!$ I :: Unsigned :: ZERO) . wrapping_shl ($ I :: FUZZ_LENGTHS [i1 - i0] as u32) ; let $ case = I :: from_unsigned (mask_lo & mask_hi) ; $ inner } } } ; }
};
}
