// Generated macro for impl_398 (impl)
macro_rules! Depcrate_re_unicodeimpl_398 {
() => {
// Module: crate::re_unicode
// Provides: {"impl_398"}
// Dependencies: {}
impl < 'c > Iterator for SubCapturesPos < 'c > { type Item = Option < (usize , usize) > ; fn next (& mut self) -> Option < Option < (usize , usize) > > { if self . idx >= self . slots . len () { return None } let r = match (self . slots [self . idx] , self . slots [self . idx + 1]) { (Some (s) , Some (e)) => Some ((s , e)) , (None , None) => None , _ => unreachable ! () } ; self . idx += 2 ; Some (r) } }
};
}
