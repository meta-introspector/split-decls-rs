// Generated macro for emit_if_duplicated (function)
macro_rules! Depcrate_attrs_duplicated_attributesemit_if_duplicated {
() => {
// Module: crate::attrs::duplicated_attributes
// Provides: {"emit_if_duplicated"}
// Dependencies: {}
fn emit_if_duplicated (cx : & EarlyContext < '_ > , attr : & MetaItem , attr_paths : & mut FxHashMap < String , Span > , complete_path : String ,) { match attr_paths . entry (complete_path) { Entry :: Vacant (v) => { v . insert (attr . span) ; } , Entry :: Occupied (o) => { span_lint_and_then (cx , DUPLICATED_ATTRIBUTES , attr . span , "duplicated attribute" , | diag | { diag . span_note (* o . get () , "first defined here") ; diag . span_help (attr . span , "remove this attribute") ; }) ; } , } }
};
}
