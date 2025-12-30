// Generated macro for impl_203 (impl)
macro_rules! Depcrate_internalsimpl_203 {
() => {
// Module: crate::internals
// Provides: {"impl_203"}
// Dependencies: {}
impl FullMappingResult < '_ > { # [allow (dead_code)] fn add_to_set < S : ClosureSink > (& self , set : & mut S) { match * self { FullMappingResult :: CodePoint (c) => set . add_char (c) , FullMappingResult :: String (s) => set . add_string (s) , FullMappingResult :: Remove => { } } } }
};
}
