// Generated macro for sorted_redaction (function)
macro_rules! Depcrate_redactionsorted_redaction {
() => {
// Module: crate::redaction
// Provides: {"sorted_redaction"}
// Dependencies: {}
# [doc = " Creates a dynamic redaction that sorts the value at the selector."] # [doc = ""] # [doc = " This is useful to force something like a set or map to be ordered to make"] # [doc = " it deterministic.  This is necessary as insta's serialization support is"] # [doc = " based on [`serde`] which does not have native set support.  As a result vectors"] # [doc = " (which need to retain order) and sets (which should be given a stable order)"] # [doc = " look the same."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use insta::{Settings, sorted_redaction};"] # [doc = " # let mut settings = Settings::new();"] # [doc = " settings.add_redaction(\".flags\", sorted_redaction());"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (feature = "redactions")))] pub fn sorted_redaction () -> Redaction { fn sort (mut value : Content , _path : ContentPath) -> Content { match value . resolve_inner_mut () { Content :: Seq (ref mut val) => { val . sort_by (| a , b | a . partial_cmp (b) . unwrap_or (std :: cmp :: Ordering :: Equal)) } Content :: Map (ref mut val) => { val . sort_by (| a , b | a . partial_cmp (b) . unwrap_or (std :: cmp :: Ordering :: Equal)) } Content :: Struct (_ , ref mut fields) | Content :: StructVariant (_ , _ , _ , ref mut fields) => { fields . sort_by (| a , b | a . partial_cmp (b) . unwrap_or (std :: cmp :: Ordering :: Equal)) } _ => { } } value } dynamic_redaction (sort) }
};
}
