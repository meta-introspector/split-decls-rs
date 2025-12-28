macro_rules! has_non_default_type_params {
    () => {
        fn has_non_default_type_params (db : & dyn HirDatabase , generic_def : GenericDefId) -> bool { let params = db . generic_params (generic_def) ; let defaults = db . generic_defaults (generic_def) ; params . iter_type_or_consts () . filter (| (_ , param) | matches ! (param , TypeOrConstParamData :: TypeParamData (_))) . map (| (local_id , _) | TypeOrConstParamId { parent : generic_def , local_id }) . any (| param | { let Some (param) = hir_ty :: param_idx (db , param) else { return false ; } ; defaults . get (param) . is_none () }) }
    };
}

has_non_default_type_params!()