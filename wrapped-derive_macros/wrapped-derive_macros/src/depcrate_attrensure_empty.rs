// Generated macro for ensure_empty (function)
macro_rules! Depcrate_attrensure_empty {
() => {
// Module: crate::attr
// Provides: {"ensure_empty"}
// Dependencies: {}
pub (crate) fn ensure_empty (ctxt : & str , attrs : & [Attribute]) { for (value_key , _) in attrs . iter () . filter_map (| attr | sval_attr (ctxt , attr)) . flatten () { panic ! ("unsupported attribute `{}` on {}" , quote ! (# value_key) , ctxt) ; } }
};
}
