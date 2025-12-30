// Generated macro for TraitDef (struct)
macro_rules! Depcrate_deriving_genericTraitDef {
() => {
// Module: crate::deriving::generic
// Provides: {"TraitDef"}
// Dependencies: {}
pub (crate) struct TraitDef < 'a > { # [doc = " The span for the current #[derive(Foo)] header."] pub span : Span , # [doc = " Path of the trait, including any type parameters"] pub path : Path , # [doc = " Whether to skip adding the current trait as a bound to the type parameters of the type."] pub skip_path_as_bound : bool , # [doc = " Whether `Copy` is needed as an additional bound on type parameters in a packed struct."] pub needs_copy_as_bound_if_packed : bool , # [doc = " Additional bounds required of any type parameters of the type,"] # [doc = " other than the current trait"] pub additional_bounds : Vec < Ty > , # [doc = " Can this trait be derived for unions?"] pub supports_unions : bool , pub methods : Vec < MethodDef < 'a > > , pub associated_types : Vec < (Ident , Ty) > , pub is_const : bool , pub is_staged_api_crate : bool , }
};
}
