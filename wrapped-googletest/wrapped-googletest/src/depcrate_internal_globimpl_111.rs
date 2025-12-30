// Generated macro for impl_111 (impl)
macro_rules! Depcrate_internal_globimpl_111 {
() => {
// Module: crate::internal::glob
// Provides: {"impl_111"}
// Dependencies: {}
impl Pattern { # [doc = " Creates a new pattern matcher.  Each pattern consists of"] # [doc = " regular characters, single-character wildcards `'?'`, and"] # [doc = " multi-character wildcards `'*'`."] pub fn new (pattern : String) -> Self { Self (pattern) } # [doc = " Returns true if and only if the wildcard pattern matches the"] # [doc = " string."] pub fn matches (& self , string : & str) -> bool { let processor = Processor { pattern : self . 0 . chars () . peekable () , string : string . chars () . peekable () , restart : None , } ; processor . process () } }
};
}
