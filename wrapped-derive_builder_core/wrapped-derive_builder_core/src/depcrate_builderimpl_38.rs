// Generated macro for impl_38 (impl)
macro_rules! Depcrate_builderimpl_38 {
() => {
// Module: crate::builder
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a > Builder < 'a > { # [doc = " Add `Clone` trait bound to generic types for non-owned builders."] # [doc = " This enables target types to declare generics without requiring a"] # [doc = " `Clone` impl. This is the same as how the built-in derives for"] # [doc = " `Clone`, `Default`, `PartialEq`, and other traits work."] fn compute_impl_bounds (& self) -> syn :: Generics { if let Some (type_gen) = self . generics { let mut generics = type_gen . clone () ; if ! self . pattern . requires_clone () || type_gen . type_params () . next () . is_none () { return generics ; } let crate_root = self . crate_root ; let clone_bound = TypeParamBound :: Trait (TraitBound { paren_token : None , modifier : TraitBoundModifier :: None , lifetimes : None , path : syn :: parse_quote ! (# crate_root :: export :: core :: clone :: Clone) , }) ; for typ in generics . type_params_mut () { typ . bounds . push (clone_bound . clone ()) ; } generics } else { Default :: default () } } }
};
}
