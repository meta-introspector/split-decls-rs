// Generated macro for dynamic_redaction (function)
macro_rules! Depcrate_redactiondynamic_redaction {
() => {
// Module: crate::redaction
// Provides: {"dynamic_redaction"}
// Dependencies: {}
# [doc = " Creates a dynamic redaction."] # [doc = ""] # [doc = " This can be used to redact a value with a different value but instead of"] # [doc = " statically declaring it a dynamic value can be computed.  This can also"] # [doc = " be used to perform assertions before replacing the value."] # [doc = ""] # [doc = " The closure is passed two arguments: the value as [`Content`]"] # [doc = " and the path that was selected (as [`ContentPath`])"] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use insta::{Settings, dynamic_redaction};"] # [doc = " # let mut settings = Settings::new();"] # [doc = " settings.add_redaction(\".id\", dynamic_redaction(|value, path| {"] # [doc = "     assert_eq!(path.to_string(), \".id\");"] # [doc = "     assert_eq!("] # [doc = "         value"] # [doc = "             .as_str()"] # [doc = "             .unwrap()"] # [doc = "             .chars()"] # [doc = "             .filter(|&c| c == '-')"] # [doc = "             .count(),"] # [doc = "         4"] # [doc = "     );"] # [doc = "     \"[uuid]\""] # [doc = " }));"] # [doc = " ```"] # [cfg_attr (docsrs , doc (cfg (feature = "redactions")))] pub fn dynamic_redaction < I , F > (func : F) -> Redaction where I : Into < Content > , F : Fn (Content , ContentPath < '_ >) -> I + Send + Sync + 'static , { Redaction :: Dynamic (Box :: new (move | c , p | func (c , p) . into ())) }
};
}
