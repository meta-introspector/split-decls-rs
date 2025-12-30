// Generated macro for impl_221 (impl)
macro_rules! Depcrate_literalsimpl_221 {
() => {
// Module: crate::literals
// Provides: {"impl_221"}
// Dependencies: {}
impl < 'a > Iterator for LiteralIter < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < Self :: Item > { match * self { LiteralIter :: Empty => None , LiteralIter :: Bytes (ref mut many) => { if many . is_empty () { None } else { let next = & many [0 .. 1] ; * many = & many [1 ..] ; Some (next) } } LiteralIter :: Single (ref mut one) => { if one . is_empty () { None } else { let next = & one [..] ; * one = & [] ; Some (next) } } LiteralIter :: AC (ref mut lits) => { if lits . is_empty () { None } else { let next = & lits [0] ; * lits = & lits [1 ..] ; Some (& * * next) } } LiteralIter :: Teddy128 (ref mut lits) => { if lits . is_empty () { None } else { let next = & lits [0] ; * lits = & lits [1 ..] ; Some (& * * next) } } } } }
};
}
