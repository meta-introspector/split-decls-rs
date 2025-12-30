// Generated macro for impl_218 (impl)
macro_rules! Depcrate_stringimpl_218 {
() => {
// Module: crate::string
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'r , 'h > Iterator for SplitN < 'r , 'h > { type Item = & 'h str ; # [inline] fn next (& mut self) -> Option < & 'h str > { if self . limit == 0 { return None ; } self . limit -= 1 ; if self . limit > 0 { return self . splits . next () ; } let len = self . splits . haystack . len () ; if self . splits . last > len { None } else { Some (& self . splits . haystack [self . splits . last .. len]) } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { self . splits . size_hint () } }
};
}
