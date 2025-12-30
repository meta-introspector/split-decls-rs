// Generated macro for BOUNDS_FIELD_PARSE_MAP (static)
macro_rules! Depcrate_internals_attributes_field_boundsBOUNDS_FIELD_PARSE_MAP {
() => {
// Module: crate::internals::attributes::field::bounds
// Provides: {"BOUNDS_FIELD_PARSE_MAP"}
// Dependencies: {}
pub static BOUNDS_FIELD_PARSE_MAP : Lazy < BTreeMap < Symbol , Box < ParseFn > > > = Lazy :: new (| | { let mut m = BTreeMap :: new () ; let f_serialize : Box < ParseFn > = Box :: new (| attr_name , meta_item_name , meta | { parse_lit_into_vec :: < WherePredicate > (attr_name , meta_item_name , meta) . map (Variants :: Serialize) }) ; let f_deserialize : Box < ParseFn > = Box :: new (| attr_name , meta_item_name , meta | { parse_lit_into_vec :: < WherePredicate > (attr_name , meta_item_name , meta) . map (Variants :: Deserialize) }) ; m . insert (SERIALIZE , f_serialize) ; m . insert (DESERIALIZE , f_deserialize) ; m }) ;
};
}
