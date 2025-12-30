// Generated macro for build_deref_for_struct (function)
macro_rules! Depcrate_item_typebuild_deref_for_struct {
() => {
// Module: crate::item_type
// Provides: {"build_deref_for_struct"}
// Dependencies: {}
fn build_deref_for_struct (item : & ItemStruct , e : & DeriveEntry , fields : & [FieldEntry] ,) -> Result < TokenStream > { let kind = e . kind ; let (impl_g , type_g , _) = item . generics . split_for_impl () ; let this_ty_ident = & item . ident ; let this_ty : Type = parse_quote ! (# this_ty_ident # type_g) ; let trait_ = kind . to_path () ; let mut wcb = WhereClauseBuilder :: new (& item . generics) ; e . push_bounds_to (& mut wcb) ; if fields . len () != 1 { bail ! (Span :: call_site () , "`#[deirve_ex({})]` supports only single field struct." , kind) ; } let target_ty = & fields [0] . field . ty ; let member = fields [0] . member () ; let content = match kind { DeriveItemKind :: Deref => { quote ! { type Target = # target_ty ; fn deref (& self) -> & # target_ty { & self .# member } } } DeriveItemKind :: DerefMut => { quote ! { fn deref_mut (& mut self) -> & mut # target_ty { & mut self .# member } } } _ => unreachable ! () , } ; let wheres = wcb . build (| ty | quote ! (# ty : # trait_)) ; Ok (quote ! { # [automatically_derived] impl # impl_g # trait_ for # this_ty # wheres { # content } }) }
};
}
