// Generated macro for impl_39 (impl)
macro_rules! Depcrate_literalsimpl_39 {
() => {
// Module: crate::literals
// Provides: {"impl_39"}
// Dependencies: {}
impl LiteralParser { pub fn new () -> LiteralParser { Default :: default () } pub fn into_literals (self) -> HashSet < Vec < u8 > > { assert_eq ! (self . condition_depth , 0) ; self . literals } pub fn extract_literals_from_file < P : AsRef < Path > > (& mut self , path : P) { let path = path . as_ref () ; let content = fs :: read_to_string (path) . expect (& format ! ("unable to read file {:?}" , path)) ; let parsed = syn :: parse_file (& content) . expect (& format ! ("unable to parse file {:?}" , path)) ; self . visit_file (& parsed) ; } }
};
}
