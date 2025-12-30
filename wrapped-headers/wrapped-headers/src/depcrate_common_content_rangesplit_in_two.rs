// Generated macro for split_in_two (function)
macro_rules! Depcrate_common_content_rangesplit_in_two {
() => {
// Module: crate::common::content_range
// Provides: {"split_in_two"}
// Dependencies: {}
fn split_in_two (s : & str , separator : char) -> Option < (& str , & str) > { let mut iter = s . splitn (2 , separator) ; match (iter . next () , iter . next ()) { (Some (a) , Some (b)) => Some ((a , b)) , _ => None , } }
};
}
