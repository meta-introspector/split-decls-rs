// Generated macro for impl_167 (impl)
macro_rules! Depcrate_codegen_from_meta_implimpl_167 {
() => {
// Module: crate::codegen::from_meta_impl
// Provides: {"impl_167"}
// Dependencies: {}
impl < 'a > OuterFromImpl < 'a > for ParseImpl < 'a > { fn trait_path (& self) -> syn :: Path { path ! (:: darling :: export :: syn :: parse :: Parse) } fn base (& 'a self) -> & 'a TraitImpl < 'a > { & self . 0 . base } fn trait_bound (& self) -> syn :: Path { self . 0 . trait_path () } }
};
}
