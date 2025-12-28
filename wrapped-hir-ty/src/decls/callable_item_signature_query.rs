macro_rules! deps {
    () => {
        HirDatabase!();
        PolyFnSig!();
        EarlyBinder!();
    };
}

macro_rules! callable_item_signature_query {
    () => {
        deps!();
        # [doc = " Build the signature of a callable item (function, struct or enum variant)."] pub (crate) fn callable_item_signature_query < 'db > (db : & 'db dyn HirDatabase , def : CallableDefId ,) -> EarlyBinder < 'db , PolyFnSig < 'db > > { match def { CallableDefId :: FunctionId (f) => fn_sig_for_fn (db , f) , CallableDefId :: StructId (s) => fn_sig_for_struct_constructor (db , s) , CallableDefId :: EnumVariantId (e) => fn_sig_for_enum_variant_constructor (db , e) , } }
    };
}

callable_item_signature_query!();