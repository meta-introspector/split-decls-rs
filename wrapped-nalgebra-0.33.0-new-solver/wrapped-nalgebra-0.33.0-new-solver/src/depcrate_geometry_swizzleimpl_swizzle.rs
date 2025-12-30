// Generated macro for impl_swizzle (macro)
macro_rules! Depcrate_geometry_swizzleimpl_swizzle {
() => {
// Module: crate::geometry::swizzle
// Provides: {"impl_swizzle"}
// Dependencies: {}
macro_rules ! impl_swizzle { ($ (where $ BaseDim : ident : $ ($ name : ident () -> $ Result : ident [$ ($ i : expr) ,+]) ,+ ;) *) => { $ ($ (# [doc = " Builds a new point from components of `self`."] # [inline] # [must_use] pub fn $ name (& self) -> $ Result < T > where < Const < D > as ToTypenum >:: Typenum : Cmp < typenum ::$ BaseDim , Output = Greater > { $ Result :: new ($ (self [$ i] . clone ()) ,*) }) *) * } }
};
}
