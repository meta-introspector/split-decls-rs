// Generated macro for keep_inhabited_variant (function)
macro_rules! Depcrate_derivekeep_inhabited_variant {
() => {
// Module: crate::derive
// Provides: {"keep_inhabited_variant"}
// Dependencies: {}
# [doc = " Filters out uninhabited and variants that we've been ordered to skip."] fn keep_inhabited_variant (ctx : Ctx , _self : & Ident , variant : Variant ,) -> DeriveResult < Option < (u32 , Ident , Vec < Field > , ParsedAttributes) > > { let attrs = attr :: parse_attributes (ctx , & variant . attrs) ? ; let fields = fields_to_vec (variant . fields) ; if attrs . skip { ensure_has_only_skip_attr (ctx , & attrs , error :: ENUM_VARIANT) ; fields . into_iter () . try_for_each (| field | { let f_attrs = attr :: parse_attributes (ctx , & field . attrs) ? ; error :: if_skip_present (ctx , & f_attrs , error :: ENUM_VARIANT_FIELD) ; ensure_has_only_skip_attr (ctx , & f_attrs , error :: ENUM_VARIANT_FIELD) ; Ok (()) }) ? ; return Ok (None) ; } if (& * fields) . is_uninhabited () { return Ok (None) ; } let weight = attrs . weight . unwrap_or (1) ; Ok (Some ((weight , variant . ident , fields , attrs))) }
};
}
