// Generated macro for namespace_and_lint (function)
macro_rules! Depcrate_attrs_utilsnamespace_and_lint {
() => {
// Module: crate::attrs::utils
// Provides: {"namespace_and_lint"}
// Dependencies: {}
# [doc = " Returns the lint namespace, if any, as well as the lint name. (`None`, `None`) means"] # [doc = " the lint had less than 1 or more than 2 segments."] pub (super) fn namespace_and_lint (lint : & MetaItemInner) -> (Option < Symbol > , Option < Symbol >) { match lint . meta_item () . map (| m | m . path . segments . as_slice ()) . unwrap_or_default () { [name] => (None , Some (name . ident . name)) , [namespace , name] => (Some (namespace . ident . name) , Some (name . ident . name)) , _ => (None , None) , } }
};
}
