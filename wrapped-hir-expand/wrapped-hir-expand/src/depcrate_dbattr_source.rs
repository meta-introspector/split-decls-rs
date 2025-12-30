// Generated macro for attr_source (function)
macro_rules! Depcrate_dbattr_source {
() => {
// Module: crate::db
// Provides: {"attr_source"}
// Dependencies: {}
# [doc = " Attributes expect the invoking attribute to be stripped"] fn attr_source (invoc_attr_index : AttrId , node : & ast :: Item) -> Option < ast :: Attr > { cov_mark :: hit ! (attribute_macro_attr_censoring) ; collect_attrs (node) . nth (invoc_attr_index . ast_index ()) . and_then (| (_ , attr) | Either :: left (attr)) }
};
}
