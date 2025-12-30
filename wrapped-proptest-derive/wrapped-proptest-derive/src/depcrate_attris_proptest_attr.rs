// Generated macro for is_proptest_attr (function)
macro_rules! Depcrate_attris_proptest_attr {
() => {
// Module: crate::attr
// Provides: {"is_proptest_attr"}
// Dependencies: {}
# [doc = " Returns `true` iff the attribute has to do with proptest."] # [doc = " Otherwise, the attribute is irrevant to us and we will simply"] # [doc = " ignore it in our processing."] fn is_proptest_attr (attr : & Attribute) -> bool { util :: eq_simple_path ("proptest" , attr . path ()) }
};
}
