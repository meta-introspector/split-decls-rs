// Generated macro for SCHEMA_FIELD_PARSE_MAP (static)
macro_rules! Depcrate_internals_attributes_field_schemaSCHEMA_FIELD_PARSE_MAP {
() => {
// Module: crate::internals::attributes::field::schema
// Provides: {"SCHEMA_FIELD_PARSE_MAP"}
// Dependencies: {}
pub static SCHEMA_FIELD_PARSE_MAP : Lazy < BTreeMap < Symbol , Box < ParseFn > > > = Lazy :: new (| | { let mut m = BTreeMap :: new () ; let f_params : Box < ParseFn > = Box :: new (| attr_name , meta_item_name , meta | { parse_lit_into_vec :: < ParameterOverride > (attr_name , meta_item_name , meta) . map (Variants :: Params) }) ; let f_with_funcs : Box < ParseFn > = Box :: new (| _attr_name , _meta_item_name , meta | { let map_result = meta_get_by_symbol_keys (WITH_FUNCS , meta , & WITH_FUNCS_FIELD_PARSE_MAP) ? ; let with_funcs : WithFuncs = map_result . into () ; if (with_funcs . declaration . is_some () && with_funcs . definitions . is_none ()) || (with_funcs . declaration . is_none () && with_funcs . definitions . is_some ()) { return Err (syn :: Error :: new_spanned (& meta . path , format ! ("both `{}` and `{}` have to be specified at the same time" , DECLARATION . 1 , DEFINITIONS . 1 ,) ,)) ; } Ok (Variants :: WithFuncs (with_funcs)) }) ; m . insert (PARAMS , f_params) ; m . insert (WITH_FUNCS , f_with_funcs) ; m }) ;
};
}
