// Generated macro for impl_428 (impl)
macro_rules! Depcrate_redactionimpl_428 {
() => {
// Module: crate::redaction
// Provides: {"impl_428"}
// Dependencies: {}
impl PathItem { fn as_str (& self) -> Option < & str > { match * self { PathItem :: Content (ref content) => content . as_str () , PathItem :: Field (s) => Some (s) , PathItem :: Index (..) => None , } } fn as_u64 (& self) -> Option < u64 > { match * self { PathItem :: Content (ref content) => content . as_u64 () , PathItem :: Field (_) => None , PathItem :: Index (idx , _) => Some (idx) , } } fn range_check (& self , start : Option < i64 > , end : Option < i64 >) -> bool { fn expand_range (sel : i64 , len : i64) -> i64 { if sel < 0 { (len + sel) . max (0) } else { sel } } let (idx , len) = match * self { PathItem :: Index (idx , len) => (idx as i64 , len as i64) , _ => return false , } ; match (start , end) { (None , None) => true , (None , Some (end)) => idx < expand_range (end , len) , (Some (start) , None) => idx >= expand_range (start , len) , (Some (start) , Some (end)) => { idx >= expand_range (start , len) && idx < expand_range (end , len) } } } }
};
}
