// Generated macro for EnumMeta (struct)
macro_rules! Depcrate_schema_metaEnumMeta {
() => {
// Module: crate::schema::meta
// Provides: {"EnumMeta"}
// Dependencies: {}
# [doc = " Enum type metadata"] # [derive (Debug)] pub struct EnumMeta < S > { # [doc (hidden)] pub name : ArcStr , # [doc (hidden)] pub description : Option < ArcStr > , # [doc (hidden)] pub values : Vec < EnumValue > , # [debug (ignore)] pub (crate) try_parse_fn : InputValueParseFn < S > , }
};
}
