// Generated macro for impl_225 (impl)
macro_rules! Depcrate_stringimpl_225 {
() => {
// Module: crate::string
// Provides: {"impl_225"}
// Dependencies: {}
impl < 'c , 'h > Iterator for SubCaptureMatches < 'c , 'h > { type Item = Option < Match < 'h > > ; # [inline] fn next (& mut self) -> Option < Option < Match < 'h > > > { let (group_index , _) = self . it . next () ? ; Some (self . caps . get (group_index)) } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } # [inline] fn count (self) -> usize { self . it . count () } }
};
}
