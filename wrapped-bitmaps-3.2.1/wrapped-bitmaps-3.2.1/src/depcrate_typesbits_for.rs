// Generated macro for bits_for (macro)
macro_rules! Depcrate_typesbits_for {
() => {
// Module: crate::types
// Provides: {"bits_for"}
// Dependencies: {}
macro_rules ! bits_for { ($ num : expr , $ result : ty) => { impl Bits for BitsImpl <$ num > { const VALUE : usize = $ num ; type Store = $ result ; } } ; }
};
}
