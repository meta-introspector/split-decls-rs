// Generated macro for ScalarMeta (struct)
macro_rules! Depcrate_schema_metaScalarMeta {
() => {
// Module: crate::schema::meta
// Provides: {"ScalarMeta"}
// Dependencies: {}
# [doc = " Scalar type metadata"] # [derive (Debug)] pub struct ScalarMeta < S > { # [doc (hidden)] pub name : ArcStr , # [doc (hidden)] pub description : Option < ArcStr > , # [doc (hidden)] pub specified_by_url : Option < ArcStr > , # [debug (ignore)] pub (crate) try_parse_fn : InputValueParseFn < S > , # [debug (ignore)] pub (crate) parse_fn : ScalarTokenParseFn < S > , }
};
}
