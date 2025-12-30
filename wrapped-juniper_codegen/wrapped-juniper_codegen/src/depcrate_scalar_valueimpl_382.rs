// Generated macro for impl_382 (impl)
macro_rules! Depcrate_scalar_valueimpl_382 {
() => {
// Module: crate::scalar_value
// Provides: {"impl_382"}
// Dependencies: {}
impl VariantAttr { # [doc = " Tries to merge two [`VariantAttr`]s into a single one, reporting about"] # [doc = " duplicates, if any."] fn try_merge (mut self , mut another : Self) -> syn :: Result < Self > { let dup = another . 0 . iter () . find (| m | self . 0 . contains (m)) ; if let Some (dup) = dup { Err (err :: dup_arg (dup . span_ident ())) } else { self . 0 . append (& mut another . 0) ; Ok (self) } } # [doc = " Parses [`VariantAttr`] from the given multiple `name`d"] # [doc = " [`syn::Attribute`]s placed on a enum variant."] fn from_attrs (name : & str , attrs : & [syn :: Attribute]) -> syn :: Result < Self > { filter_attrs (name , attrs) . map (| attr | attr . parse_args ()) . try_fold (Self :: default () , | prev , curr | prev . try_merge (curr ?)) } }
};
}
