// Generated macro for impl_378 (impl)
macro_rules! Depcrate_scalar_valueimpl_378 {
() => {
// Module: crate::scalar_value
// Provides: {"impl_378"}
// Dependencies: {}
impl Attr { # [doc = " Tries to merge two [`Attr`]s into a single one, reporting about"] # [doc = " duplicates, if any."] fn try_merge (self , mut another : Self) -> syn :: Result < Self > { Ok (Self { from_displayable : try_merge_opt ! (from_displayable : self , another) , from_displayable_non_static : try_merge_opt ! (from_displayable_non_static : self , another) , }) } # [doc = " Parses [`Attr`] from the given multiple `name`d [`syn::Attribute`]s"] # [doc = " placed on a enum variant."] fn from_attrs (name : & str , attrs : & [syn :: Attribute]) -> syn :: Result < Self > { filter_attrs (name , attrs) . map (| attr | attr . parse_args ()) . try_fold (Self :: default () , | prev , curr | prev . try_merge (curr ?)) } }
};
}
