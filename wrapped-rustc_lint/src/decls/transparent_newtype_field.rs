macro_rules! transparent_newtype_field {
    () => {
        # [doc = " `repr(transparent)` structs can have a single non-1-ZST field, this function returns that"] # [doc = " field."] pub (crate) fn transparent_newtype_field < 'a , 'tcx > (tcx : TyCtxt < 'tcx > , variant : & 'a ty :: VariantDef ,) -> Option < & 'a ty :: FieldDef > { let typing_env = ty :: TypingEnv :: non_body_analysis (tcx , variant . def_id) ; variant . fields . iter () . find (| field | { let field_ty = tcx . type_of (field . did) . instantiate_identity () ; let is_1zst = tcx . layout_of (typing_env . as_query_input (field_ty)) . is_ok_and (| layout | layout . is_1zst ()) ; ! is_1zst }) }
    };
}

transparent_newtype_field!()