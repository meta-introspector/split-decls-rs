// Generated macro for make_drop_impl (function)
macro_rules! Depcrate_pin_project_derivemake_drop_impl {
() => {
// Module: crate::pin_project::derive
// Provides: {"make_drop_impl"}
// Dependencies: {}
# [allow (clippy :: doc_overindented_list_items)] # [doc = " Creates `Drop` implementation for the original type."] # [doc = ""] # [doc = " The kind of `Drop` impl generated depends on `pinned_drop` field:"] # [doc = " - `Some` - implements `Drop` via `PinnedDrop` impl."] # [doc = " - `None` - generates code that ensures that `Drop` trait is not implemented,"] # [doc = "            instead of generating `Drop` impl."] fn make_drop_impl (cx : & Context < '_ >) -> TokenStream { let ident = cx . orig . ident ; let (impl_generics , ty_generics , where_clause) = cx . orig . generics . split_for_impl () ; if let Some (span) = cx . pinned_drop { let unsafety = < Token ! [unsafe] > :: default () ; quote_spanned ! { span => impl # impl_generics _pin_project :: __private :: Drop for # ident # ty_generics # where_clause { # [allow (clippy :: missing_inline_in_public_items)] fn drop (& mut self) { # unsafety { let __pinned_self = _pin_project :: __private :: Pin :: new_unchecked (self) ; _pin_project :: __private :: PinnedDrop :: drop (__pinned_self) ; } } } } } else { let trait_ident = format_ident ! ("{}MustNotImplDrop" , ident) ; quote ! { trait # trait_ident { } # [allow (clippy :: drop_bounds , drop_bounds)] impl < T : _pin_project :: __private :: Drop > # trait_ident for T { } impl # impl_generics # trait_ident for # ident # ty_generics # where_clause { } # [doc (hidden)] impl # impl_generics _pin_project :: __private :: PinnedDrop for # ident # ty_generics # where_clause { unsafe fn drop (self : _pin_project :: __private :: Pin <& mut Self >) { } } } } }
};
}
