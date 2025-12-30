// Generated macro for write_negative_assume_odd (function)
macro_rules! Depcrate_limbwrite_negative_assume_odd {
() => {
// Module: crate::limb
// Provides: {"write_negative_assume_odd"}
// Dependencies: {}
pub (crate) fn write_negative_assume_odd < 'r > (r : & mut Cursor < 'r , Limb > , a : & [Limb] ,) -> Result < & 'r mut [Limb] , LenMismatchError > { let r = r . write_iter (a . iter () . map (| & a | ! a)) . src_empty () ? . into_written () ; let Some (least_significant_limb) = r . get_mut (0) else { return Err (LenMismatchError :: new (a . len ())) ; } ; * least_significant_limb |= 1 ; Ok (r) }
};
}
