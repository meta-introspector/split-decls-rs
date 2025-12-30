// Generated macro for impl_357 (impl)
macro_rules! Depcrate_inlay_hintsimpl_357 {
() => {
// Module: crate::inlay_hints
// Provides: {"impl_357"}
// Dependencies: {}
impl std :: hash :: Hash for InlayHint { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . range . hash (state) ; self . position . hash (state) ; self . pad_left . hash (state) ; self . pad_right . hash (state) ; self . kind . hash (state) ; self . label . hash (state) ; mem :: discriminant (& self . text_edit) . hash (state) ; } }
};
}
