macro_rules! deps {
    () => {
        LateContext!();
        BuiltinNoMangleGeneric!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl InvalidNoMangleItems { fn check_no_mangle_on_generic_fn (& self , cx : & LateContext < '_ > , attr_span : Span , def_id : LocalDefId ,) { let generics = cx . tcx . generics_of (def_id) ; if generics . requires_monomorphization (cx . tcx) { cx . emit_span_lint (NO_MANGLE_GENERIC_ITEMS , cx . tcx . def_span (def_id) , BuiltinNoMangleGeneric { suggestion : attr_span } ,) ; } } }
    };
}

impl_49!();