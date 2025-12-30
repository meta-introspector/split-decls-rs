// Generated macro for impl_385 (impl)
macro_rules! Depcrate_re_unicodeimpl_385 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_385"}
// Dependencies: {}
impl NamedGroups { fn from_regex (regex : & Regex) -> NamedGroups { match regex . 0 { _Regex :: Plugin (ref plug) => NamedGroups :: Plugin (& plug . groups) , _Regex :: Dynamic (ref exec) => { NamedGroups :: Dynamic (exec . capture_name_idx () . clone ()) } } } fn pos (& self , name : & str) -> Option < usize > { match * self { NamedGroups :: Plugin (groups) => { groups . binary_search_by (| & (n , _) | n . cmp (name)) . ok () . map (| i | groups [i] . 1) } , NamedGroups :: Dynamic (ref groups) => { groups . get (name) . map (| i | * i) } , } } fn iter < 'n > (& 'n self) -> NamedGroupsIter < 'n > { match * self { NamedGroups :: Plugin (g) => NamedGroupsIter :: Plugin (g . iter ()) , NamedGroups :: Dynamic (ref g) => NamedGroupsIter :: Dynamic (g . iter ()) , } } }
};
}
