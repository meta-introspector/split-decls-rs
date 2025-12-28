macro_rules! maybe_check_static_with_link_section {
    () => {
        pub (super) fn maybe_check_static_with_link_section (tcx : TyCtxt < '_ > , id : LocalDefId) { if ! tcx . sess . target . is_like_wasm { return ; } let Some (link_section) = tcx . codegen_fn_attrs (id) . link_section else { return ; } ; if let Ok (alloc) = tcx . eval_static_initializer (id . to_def_id ()) && ! alloc . inner () . provenance () . ptrs () . is_empty () && ! link_section . as_str () . starts_with (".init_array") { let msg = "statics with a custom `#[link_section]` must be a \
                        simple list of bytes on the wasm target with no \
                        extra levels of indirection such as references" ; tcx . dcx () . span_err (tcx . def_span (id) , msg) ; } }
    };
}

maybe_check_static_with_link_section!()