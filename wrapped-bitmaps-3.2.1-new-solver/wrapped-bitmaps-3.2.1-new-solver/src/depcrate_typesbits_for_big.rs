// Generated macro for bits_for_big (macro)
macro_rules! Depcrate_typesbits_for_big {
() => {
// Module: crate::types
// Provides: {"bits_for_big"}
// Dependencies: {}
macro_rules ! bits_for_big { ($ num : expr , $ words : expr) => { impl Bits for BitsImpl <$ num > { const VALUE : usize = $ num ; type Store = [u128 ; $ words] ; } } ; }
};
}
