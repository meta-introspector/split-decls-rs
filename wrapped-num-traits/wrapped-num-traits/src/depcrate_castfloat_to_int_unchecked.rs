// Generated macro for float_to_int_unchecked (macro)
macro_rules! Depcrate_castfloat_to_int_unchecked {
() => {
// Module: crate::cast
// Provides: {"float_to_int_unchecked"}
// Dependencies: {}
macro_rules ! float_to_int_unchecked { ($ float : expr => $ int : ty) => { unsafe { $ float . to_int_unchecked ::<$ int > () } } ; }
};
}
