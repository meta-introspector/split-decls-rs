// Generated macro for impl_49 (impl)
macro_rules! Depcrate_matcherimpl_49 {
() => {
// Module: crate::matcher
// Provides: {"impl_49"}
// Dependencies: {}
impl Captures for RegexCaptures { # [inline] fn len (& self) -> usize { self . caps . group_info () . all_group_len () } # [inline] fn get (& self , i : usize) -> Option < Match > { self . caps . get_group (i) . map (| sp | Match :: new (sp . start , sp . end)) } }
};
}
