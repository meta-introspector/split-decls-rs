// Generated macro for impl_215 (impl)
macro_rules! Depcrate_stringimpl_215 {
() => {
// Module: crate::string
// Provides: {"impl_215"}
// Dependencies: {}
impl < 'r , 'h > Iterator for Split < 'r , 'h > { type Item = & 'h str ; # [inline] fn next (& mut self) -> Option < & 'h str > { match self . finder . next () { None => { let len = self . haystack . len () ; if self . last > len { None } else { let range = self . last .. len ; self . last = len + 1 ; Some (& self . haystack [range]) } } Some (m) => { let range = self . last .. m . start () ; self . last = m . end () ; Some (& self . haystack [range]) } } } }
};
}
