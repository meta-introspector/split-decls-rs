// Generated macro for BORSH_FIELD_PARSE_MAP (static)
macro_rules! Depcrate_internals_attributes_fieldBORSH_FIELD_PARSE_MAP {
() => {
// Module: crate::internals::attributes::field
// Provides: {"BORSH_FIELD_PARSE_MAP"}
// Dependencies: {}
static BORSH_FIELD_PARSE_MAP : Lazy < BTreeMap < Symbol , Box < ParseFn > > > = Lazy :: new (| | { let mut m = BTreeMap :: new () ; let f_bounds : Box < ParseFn > = Box :: new (| _attr_name , _meta_item_name , meta | { let map_result = meta_get_by_symbol_keys (BOUND , meta , & BOUNDS_FIELD_PARSE_MAP) ? ; let bounds_attributes : bounds :: Bounds = map_result . into () ; Ok (Variants :: Bounds (bounds_attributes)) }) ; let f_serialize_with : Box < ParseFn > = Box :: new (| attr_name , meta_item_name , meta | { parse_lit_into :: < syn :: ExprPath > (attr_name , meta_item_name , meta) . map (Variants :: SerializeWith) }) ; let f_deserialize_with : Box < ParseFn > = Box :: new (| attr_name , meta_item_name , meta | { parse_lit_into :: < syn :: ExprPath > (attr_name , meta_item_name , meta) . map (Variants :: DeserializeWith) }) ; # [cfg (feature = "schema")] let f_schema : Box < ParseFn > = Box :: new (| _attr_name , _meta_item_name , meta | { let map_result = meta_get_by_symbol_keys (SCHEMA , meta , & SCHEMA_FIELD_PARSE_MAP) ? ; let schema_attributes : schema :: Attributes = map_result . into () ; Ok (Variants :: Schema (schema_attributes)) }) ; let f_skip : Box < ParseFn > = Box :: new (| _attr_name , _meta_item_name , _meta | Ok (Variants :: Skip (()))) ; m . insert (BOUND , f_bounds) ; m . insert (SERIALIZE_WITH , f_serialize_with) ; m . insert (DESERIALIZE_WITH , f_deserialize_with) ; m . insert (SKIP , f_skip) ; # [cfg (feature = "schema")] m . insert (SCHEMA , f_schema) ; m }) ;
};
}
