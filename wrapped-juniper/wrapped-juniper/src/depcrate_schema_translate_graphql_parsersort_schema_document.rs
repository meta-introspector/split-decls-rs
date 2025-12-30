// Generated macro for sort_schema_document (function)
macro_rules! Depcrate_schema_translate_graphql_parsersort_schema_document {
() => {
// Module: crate::schema::translate::graphql_parser
// Provides: {"sort_schema_document"}
// Dependencies: {}
# [doc = " Sorts the provided [`schema::Document`] in the \"type-then-name\" manner."] pub (crate) fn sort_schema_document < 'a , T > (document : & mut schema :: Document < 'a , T >) where T : schema :: Text < 'a > , { document . definitions . sort_by (move | a , b | { let type_cmp = sort_value :: by_type (a) . cmp (& sort_value :: by_type (b)) ; let name_cmp = sort_value :: by_is_directive (a) . cmp (& sort_value :: by_is_directive (b)) . then (sort_value :: by_name (a) . cmp (& sort_value :: by_name (b))) . then (sort_value :: by_directive (a) . cmp (& sort_value :: by_directive (b))) ; type_cmp . then (name_cmp) }) }
};
}
