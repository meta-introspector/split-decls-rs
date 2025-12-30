// Generated macro for priority_from_query_string (function)
macro_rules! Depcrate_commonpriority_from_query_string {
() => {
// Module: crate::common
// Provides: {"priority_from_query_string"}
// Dependencies: {}
# [doc = " Construct a Priority from quiche apps custom query string."] pub fn priority_from_query_string (url : & url :: Url) -> Option < Priority > { let mut urgency = None ; let mut incremental = None ; for param in url . query_pairs () { if param . 0 == "u" { urgency = Some (param . 1 . parse :: < u8 > () . unwrap ()) ; } if param . 0 == "i" && param . 1 == "1" { incremental = Some (true) ; } } match (urgency , incremental) { (Some (u) , Some (i)) => Some (Priority :: new (u , i)) , (Some (u) , None) => Some (Priority :: new (u , false)) , (None , Some (i)) => Some (Priority :: new (3 , i)) , (None , None) => None , } }
};
}
