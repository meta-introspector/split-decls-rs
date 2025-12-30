// Generated macro for impl_320 (impl)
macro_rules! Depcrate_re_bytesimpl_320 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_320"}
// Dependencies: {}
impl < 'c > Iterator for SubCapturesPos < 'c > { type Item = Option < (usize , usize) > ; fn next (& mut self) -> Option < Option < (usize , usize) > > { if self . idx >= self . slots . len () { return None } let r = match (self . slots [self . idx] , self . slots [self . idx + 1]) { (Some (s) , Some (e)) => Some ((s , e)) , _ => None , } ; self . idx += 2 ; Some (r) } }
};
}
