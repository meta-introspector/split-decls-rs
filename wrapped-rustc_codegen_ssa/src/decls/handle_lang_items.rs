macro_rules! deps {
    () => {
        InterestingAttributeDiagnosticSpans!();
    };
}

macro_rules! handle_lang_items {
    () => {
        deps!();
        fn handle_lang_items (tcx : TyCtxt < '_ > , did : LocalDefId , interesting_spans : & InterestingAttributeDiagnosticSpans , attrs : & [Attribute] , codegen_fn_attrs : & mut CodegenFnAttrs ,) { let lang_item = lang_items :: extract (attrs) . and_then (| (name , _) | LangItem :: from_name (name)) ; if let Some (lang_item) = lang_item && let Some (link_name) = lang_item . link_name () { codegen_fn_attrs . flags |= CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL ; codegen_fn_attrs . symbol_name = Some (link_name) ; } if codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL) && codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: NO_MANGLE) { let mut err = tcx . dcx () . struct_span_err (interesting_spans . no_mangle . unwrap_or_default () , "`#[no_mangle]` cannot be used on internal language items" ,) . with_note ("Rustc requires this item to have a specific mangled name.") . with_span_label (tcx . def_span (did) , "should be the internal language item") ; if let Some (lang_item) = lang_item && let Some (link_name) = lang_item . link_name () { err = err . with_note ("If you are trying to prevent mangling to ease debugging, many") . with_note (format ! ("debuggers support a command such as `rbreak {link_name}` to")) . with_note (format ! ("match `.*{link_name}.*` instead of `break {link_name}` on a specific name")) } err . emit () ; } }
    };
}

handle_lang_items!();