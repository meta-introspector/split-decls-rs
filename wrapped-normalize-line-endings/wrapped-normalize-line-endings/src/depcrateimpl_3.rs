// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl < I > Iterator for Normalized < I > where I : Iterator < Item = char > , { type Item = char ; fn next (& mut self) -> Option < char > { match self . iter . next () { Some ('\n') if self . prev_was_cr => { self . prev_was_cr = false ; match self . iter . next () { Some ('\r') => { self . prev_was_cr = true ; Some ('\n') } any => { self . prev_was_cr = false ; any } } } Some ('\r') => { self . prev_was_cr = true ; Some ('\n') } any => { self . prev_was_cr = false ; any } } } }
};
}
