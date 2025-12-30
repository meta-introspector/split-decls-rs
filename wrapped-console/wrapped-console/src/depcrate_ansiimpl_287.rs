// Generated macro for impl_287 (impl)
macro_rules! Depcrate_ansiimpl_287 {
() => {
// Module: crate::ansi
// Provides: {"impl_287"}
// Dependencies: {}
impl < 'a > Iterator for AnsiCodeIterator < 'a > { type Item = (& 'a str , bool) ; fn next (& mut self) -> Option < (& 'a str , bool) > { if let Some (pending_item) = self . pending_item . take () { self . cur_idx += pending_item . 0 . len () ; Some (pending_item) } else if let Some (m) = self . iter . next () { let s = & self . s [self . last_idx .. m . start] ; self . last_idx = m . end ; if s . is_empty () { self . cur_idx = m . end ; Some ((m . as_str () , true)) } else { self . cur_idx = m . start ; self . pending_item = Some ((m . as_str () , true)) ; Some ((s , false)) } } else if self . last_idx < self . s . len () { let rv = & self . s [self . last_idx ..] ; self . cur_idx = self . s . len () ; self . last_idx = self . s . len () ; Some ((rv , false)) } else { None } } }
};
}
