// Generated macro for impl_41 (impl)
macro_rules! Depcrate_matcherimpl_41 {
() => {
// Module: crate::matcher
// Provides: {"impl_41"}
// Dependencies: {}
impl Matcher { # [doc = " Find the match on the input `string`."] pub fn find (& self , string : & str) -> Option < String > { match * self { Self :: AllTrimmed => Some (string . trim () . to_string ()) , Self :: PrefixedWord { prefix } => find_prefixed_word (string , prefix) . map (str :: to_owned) , Self :: PrefixedVersion { prefix } => find_prefixed_word (string , prefix) . filter (| & v | is_valid_version (v)) . map (str :: to_owned) , Self :: KeyValue { key } => find_by_key (string , key) . map (str :: to_owned) , } } }
};
}
