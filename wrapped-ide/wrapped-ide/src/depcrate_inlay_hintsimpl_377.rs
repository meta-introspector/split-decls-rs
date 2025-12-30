// Generated macro for impl_377 (impl)
macro_rules! Depcrate_inlay_hintsimpl_377 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_377"}
// Dependencies: {}
impl std :: hash :: Hash for InlayHintLabelPart { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . text . hash (state) ; self . linked_location . is_some () . hash (state) ; self . tooltip . is_some () . hash (state) ; } }
};
}
