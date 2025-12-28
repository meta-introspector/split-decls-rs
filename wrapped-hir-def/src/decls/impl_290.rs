macro_rules! deps {
    () => {
        ExpressionStoreSourceMap!();
        ExpressionOnlySourceMap!();
        ExpressionOnlyStore!();
        FormatTemplate!();
        ExpressionStore!();
        ExpressionStoreBuilder!();
    };
}

macro_rules! impl_290 {
    () => {
        deps!();
        impl ExpressionStoreBuilder { pub fn finish (self) -> (ExpressionStore , ExpressionStoreSourceMap) { let Self { block_scopes , mut exprs , mut labels , mut pats , mut bindings , mut binding_owners , mut ident_hygiene , mut types , mut lifetimes , mut expr_map , mut expr_map_back , mut pat_map , mut pat_map_back , mut label_map , mut label_map_back , mut types_map_back , mut types_map , mut lifetime_map_back , mut lifetime_map , mut binding_definitions , mut field_map_back , mut pat_field_map_back , mut template_map , mut expansions , diagnostics , } = self ; exprs . shrink_to_fit () ; labels . shrink_to_fit () ; pats . shrink_to_fit () ; bindings . shrink_to_fit () ; binding_owners . shrink_to_fit () ; ident_hygiene . shrink_to_fit () ; types . shrink_to_fit () ; lifetimes . shrink_to_fit () ; expr_map . shrink_to_fit () ; expr_map_back . shrink_to_fit () ; pat_map . shrink_to_fit () ; pat_map_back . shrink_to_fit () ; label_map . shrink_to_fit () ; label_map_back . shrink_to_fit () ; types_map_back . shrink_to_fit () ; types_map . shrink_to_fit () ; lifetime_map_back . shrink_to_fit () ; lifetime_map . shrink_to_fit () ; binding_definitions . shrink_to_fit () ; field_map_back . shrink_to_fit () ; pat_field_map_back . shrink_to_fit () ; if let Some (template_map) = & mut template_map { let FormatTemplate { format_args_to_captures , asm_to_captures , implicit_capture_to_source , } = & mut * * template_map ; format_args_to_captures . shrink_to_fit () ; asm_to_captures . shrink_to_fit () ; implicit_capture_to_source . shrink_to_fit () ; } expansions . shrink_to_fit () ; let has_exprs = ! exprs . is_empty () || ! labels . is_empty () || ! pats . is_empty () || ! bindings . is_empty () ; let store = { let expr_only = if has_exprs { Some (Box :: new (ExpressionOnlyStore { exprs , pats , bindings , labels , binding_owners , block_scopes : block_scopes . into_boxed_slice () , ident_hygiene , })) } else { None } ; ExpressionStore { expr_only , types , lifetimes } } ; let source_map = { let expr_only = if has_exprs || ! expansions . is_empty () || ! diagnostics . is_empty () { Some (Box :: new (ExpressionOnlySourceMap { expr_map , expr_map_back , pat_map , pat_map_back , label_map , label_map_back , binding_definitions , field_map_back , pat_field_map_back , template_map , expansions , diagnostics : ThinVec :: from_iter (diagnostics) , })) } else { None } ; ExpressionStoreSourceMap { expr_only , types_map_back , types_map , lifetime_map_back , lifetime_map , } } ; (store , source_map) } }
    };
}

impl_290!();