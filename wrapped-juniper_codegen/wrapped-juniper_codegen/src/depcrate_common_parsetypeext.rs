// Generated macro for TypeExt (trait)
macro_rules! Depcrate_common_parseTypeExt {
() => {
// Module: crate::common::parse
// Provides: {"TypeExt"}
// Dependencies: {}
# [doc = " Extension of [`syn::Type`] providing common function widely used by this crate for parsing."] pub (crate) trait TypeExt { # [doc = " Retrieves the innermost non-parenthesized [`syn::Type`] from the given"] # [doc = " one (unwraps nested [`syn::TypeParen`]s asap)."] # [must_use] fn unparenthesized (& self) -> & Self ; # [doc = " Retrieves the inner [`syn::Type`] from the given reference type, or just"] # [doc = " returns \"as is\" if the type is not a reference."] # [doc = ""] # [doc = " Also, makes the type [`TypeExt::unparenthesized`], if possible."] # [must_use] fn unreferenced (& self) -> & Self ; # [doc = " Iterates mutably over all the lifetime parameters of this [`syn::Type`]"] # [doc = " with the given `func`tion."] fn lifetimes_iter_mut < F : FnMut (& mut syn :: Lifetime) > (& mut self , func : & mut F) ; # [doc = " Anonymizes all the lifetime parameters of this [`syn::Type`] (except"] # [doc = " the `'static` ones), making it suitable for using in contexts with"] # [doc = " inferring."] fn lifetimes_anonymized (& mut self) ; # [doc = " Returns the topmost [`syn::Ident`] of this [`syn::TypePath`], if any."] # [must_use] fn topmost_ident (& self) -> Option < & syn :: Ident > ; }
};
}
