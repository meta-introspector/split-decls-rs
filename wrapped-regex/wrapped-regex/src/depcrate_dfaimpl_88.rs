// Generated macro for impl_88 (impl)
macro_rules! Depcrate_dfaimpl_88 {
() => {
// Module: crate::dfa
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'a > Iterator for InstPtrs < 'a > { type Item = usize ; fn next (& mut self) -> Option < usize > { if self . data . is_empty () { return None ; } let (delta , nread) = read_vari32 (self . data) ; let base = self . base as i32 + delta ; debug_assert ! (base >= 0) ; debug_assert ! (nread > 0) ; self . data = & self . data [nread ..] ; self . base = base as usize ; Some (self . base) } }
};
}
