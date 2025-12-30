// Generated macro for SerializeOpts (struct)
macro_rules! Depcrate_serializeSerializeOpts {
() => {
// Module: crate::serialize
// Provides: {"SerializeOpts"}
// Dependencies: {}
# [derive (Clone)] pub struct SerializeOpts { # [doc = " Is scripting enabled? Default: true"] pub scripting_enabled : bool , # [doc = " Serialize the root node? Default: ChildrenOnly"] pub traversal_scope : TraversalScope , # [doc = " If the serializer is asked to serialize an invalid tree, the default"] # [doc = " behavior is to panic in the event that an `end_elem` is created without a"] # [doc = " matching `start_elem`. Setting this to true will prevent those panics by"] # [doc = " creating a default parent on the element stack. No extra start elem will"] # [doc = " actually be written. Default: false"] pub create_missing_parent : bool , }
};
}
