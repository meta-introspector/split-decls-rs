// Generated macro for GenericsExt (trait)
macro_rules! Depcrate_common_parseGenericsExt {
() => {
// Module: crate::common::parse
// Provides: {"GenericsExt"}
// Dependencies: {}
# [doc = " Extension of [`syn::Generics`] providing common function widely used by this crate for parsing."] pub (crate) trait GenericsExt { # [doc = " Moves all trait and lifetime bounds of these [`syn::Generics`] to its [`syn::WhereClause`]."] fn move_bounds_to_where_clause (& mut self) ; # [doc = " Replaces generic parameters in the given [`syn::Type`] with default"] # [doc = " ones, provided by these [`syn::Generics`]."] fn replace_type_with_defaults (& self , ty : & mut syn :: Type) ; # [doc = " Replaces generic parameters in the given [`syn::TypePath`] with default"] # [doc = " ones, provided by these [`syn::Generics`]."] fn replace_type_path_with_defaults (& self , ty : & mut syn :: TypePath) ; }
};
}
