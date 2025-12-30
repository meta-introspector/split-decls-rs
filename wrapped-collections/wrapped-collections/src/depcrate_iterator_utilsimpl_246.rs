// Generated macro for impl_246 (impl)
macro_rules! Depcrate_iterator_utilsimpl_246 {
() => {
// Module: crate::iterator_utils
// Provides: {"impl_246"}
// Dependencies: {}
impl < I , T : Eq > Iterator for RangeListIteratorCoalescer < I , T > where I : Iterator < Item = CodePointMapRange < T > > , { type Item = CodePointMapRange < T > ; fn next (& mut self) -> Option < Self :: Item > { let mut ret = if let Some (peek) = self . peek . take () { peek } else if let Some (next) = self . iter . next () { next } else { return None ; } ; # [expect (clippy :: while_let_on_iterator)] while let Some (next) = self . iter . next () { if * next . range . start () == ret . range . end () + 1 && next . value == ret . value { ret . range = * ret . range . start () ..= * next . range . end () ; } else { self . peek = Some (next) ; return Some (ret) ; } } Some (ret) } }
};
}
