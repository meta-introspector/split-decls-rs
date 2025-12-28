macro_rules! deps {
    () => {
        InvalidLiteralValue!();
        OutOfRangeInteger!();
        UnexpectedParameterName!();
        ExpectedNameValuePair!();
    };
}

macro_rules! parse_patchable_function_entry {
    () => {
        deps!();
        fn parse_patchable_function_entry (tcx : TyCtxt < '_ > , attr : & Attribute ,) -> Option < PatchableFunctionEntry > { attr . meta_item_list () . and_then (| l | { let mut prefix = None ; let mut entry = None ; for item in l { let Some (meta_item) = item . meta_item () else { tcx . dcx () . emit_err (errors :: ExpectedNameValuePair { span : item . span () }) ; continue ; } ; let Some (name_value_lit) = meta_item . name_value_literal () else { tcx . dcx () . emit_err (errors :: ExpectedNameValuePair { span : item . span () }) ; continue ; } ; let attrib_to_write = match meta_item . name () { Some (sym :: prefix_nops) => & mut prefix , Some (sym :: entry_nops) => & mut entry , _ => { tcx . dcx () . emit_err (errors :: UnexpectedParameterName { span : item . span () , prefix_nops : sym :: prefix_nops , entry_nops : sym :: entry_nops , }) ; continue ; } } ; let rustc_ast :: LitKind :: Int (val , _) = name_value_lit . kind else { tcx . dcx () . emit_err (errors :: InvalidLiteralValue { span : name_value_lit . span }) ; continue ; } ; let Ok (val) = val . get () . try_into () else { tcx . dcx () . emit_err (errors :: OutOfRangeInteger { span : name_value_lit . span }) ; continue ; } ; * attrib_to_write = Some (val) ; } if let (None , None) = (prefix , entry) { tcx . dcx () . span_err (attr . span () , "must specify at least one parameter") ; } Some (PatchableFunctionEntry :: from_prefix_and_entry (prefix . unwrap_or (0) , entry . unwrap_or (0))) }) }
    };
}

parse_patchable_function_entry!()