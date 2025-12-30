// Generated macro for impl_184 (impl)
macro_rules! Depcrate_stringimpl_184 {
() => {
// Module: crate::string
// Provides: {"impl_184"}
// Dependencies: {}
impl < 'a > Iterator for CharWindows < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { let elt ; if let Some (c) = char_get (self . s , self . a) { elt = & self . s [self . a .. self . b] ; self . a += c . len_utf8 () ; } else { return None ; } if let Some (c) = char_get (self . s , self . b) { self . b += c . len_utf8 () ; } else { self . a = self . s . len () ; } Some (elt) } }
};
}
