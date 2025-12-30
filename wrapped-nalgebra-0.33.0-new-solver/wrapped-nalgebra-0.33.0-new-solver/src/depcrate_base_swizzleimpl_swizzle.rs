// Generated macro for impl_swizzle (macro)
macro_rules! Depcrate_base_swizzleimpl_swizzle {
() => {
// Module: crate::base::swizzle
// Provides: {"impl_swizzle"}
// Dependencies: {}
macro_rules ! impl_swizzle { ($ (where $ BaseDim : ident : $ ($ name : ident () -> $ Result : ident [$ ($ i : expr) ,+]) ,+ ;) *) => { $ ($ (# [doc = " Builds a new vector from components of `self`."] # [inline] # [must_use] pub fn $ name (& self) -> $ Result < T > where D :: Typenum : Cmp < typenum ::$ BaseDim , Output = Greater > { $ Result :: new ($ (self [$ i] . clone ()) ,*) }) *) * } }
};
}
