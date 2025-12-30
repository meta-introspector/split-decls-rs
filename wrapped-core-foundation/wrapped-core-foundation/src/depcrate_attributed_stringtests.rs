// Generated macro for tests (module)
macro_rules! Depcrate_attributed_stringtests {
() => {
// Module: crate::attributed_string
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn attributed_string_type_id_comparison () { assert_eq ! (< CFAttributedString as TCFType >:: type_id () , < CFMutableAttributedString as TCFType >:: type_id ()) ; } }
};
}
