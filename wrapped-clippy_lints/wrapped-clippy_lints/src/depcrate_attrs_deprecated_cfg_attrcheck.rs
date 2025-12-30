// Generated macro for check (function)
macro_rules! Depcrate_attrs_deprecated_cfg_attrcheck {
() => {
// Module: crate::attrs::deprecated_cfg_attr
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & EarlyContext < '_ > , attr : & Attribute , msrv : & MsrvStack) { if attr . has_name (sym :: cfg_attr) && let Some (items) = attr . meta_item_list () && items . len () == 2 && let Some (feature_item) = items [0] . meta_item () { if feature_item . has_name (sym :: rustfmt) && msrv . meets (msrvs :: TOOL_ATTRIBUTES) && let Some (skip_item) = & items [1] . meta_item () && (skip_item . has_name (sym :: rustfmt_skip) || skip_item . path . segments . last () . expect ("empty path in attribute") . ident . name == sym :: skip) && attr . style == AttrStyle :: Outer { span_lint_and_sugg (cx , DEPRECATED_CFG_ATTR , attr . span , "`cfg_attr` is deprecated for rustfmt and got replaced by tool attributes" , "use" , "#[rustfmt::skip]" . to_string () , Applicability :: MachineApplicable ,) ; } else { check_deprecated_cfg_recursively (cx , feature_item) ; if let Some (behind_cfg_attr) = items [1] . meta_item () { unnecessary_clippy_cfg :: check (cx , feature_item , behind_cfg_attr , attr) ; } } } }
};
}
