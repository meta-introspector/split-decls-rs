// Generated macro for build_default_ctor_args (function)
macro_rules! Depcrate_item_typebuild_default_ctor_args {
() => {
// Module: crate::item_type
// Provides: {"build_default_ctor_args"}
// Dependencies: {}
fn build_default_ctor_args (fields_source : & Fields , fields : & [FieldEntry] , use_bounds : bool , wcb : & mut WhereClauseBuilder ,) -> Result < TokenStream > { let kind = DeriveItemKind :: Default ; let trait_ = kind . to_path () ; let mut ctor_args = Vec :: new () ; for field in fields { let value = field . hattrs . default_value (& field . field . ty) ; if field . hattrs . push_bounds_to (use_bounds , kind , wcb) && value . is_none () { wcb . push_bounds_for_field (field . field) } let value = if let Some (value) = value { value } else { let field_ty = & field . field . ty ; quote ! (<# field_ty as # trait_ >:: default ()) } ; ctor_args . push (value) ; } Ok (build_ctor_args (fields_source , & ctor_args)) }
};
}
