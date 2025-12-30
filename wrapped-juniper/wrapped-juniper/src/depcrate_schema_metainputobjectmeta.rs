// Generated macro for InputObjectMeta (struct)
macro_rules! Depcrate_schema_metaInputObjectMeta {
() => {
// Module: crate::schema::meta
// Provides: {"InputObjectMeta"}
// Dependencies: {}
# [doc = " Input object metadata"] # [derive (Debug)] pub struct InputObjectMeta < S > { # [doc (hidden)] pub name : ArcStr , # [doc (hidden)] pub description : Option < ArcStr > , # [doc (hidden)] pub input_fields : Vec < Argument < S > > , # [doc (hidden)] pub is_one_of : bool , # [debug (ignore)] pub (crate) try_parse_fn : InputValueParseFn < S > , }
};
}
