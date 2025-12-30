// Generated macro for impl_1428 (impl)
macro_rules! Depcrate_stringimpl_1428 {
() => {
// Module: crate::string
// Provides: {"impl_1428"}
// Dependencies: {}
impl Strategy for str { type Tree = RegexGeneratorValueTree < String > ; type Value = String ; fn new_tree (& self , runner : & mut TestRunner) -> NewTree < Self > { string_regex (self) . unwrap () . new_tree (runner) } }
};
}
