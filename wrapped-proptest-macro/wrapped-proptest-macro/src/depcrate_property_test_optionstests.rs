// Generated macro for tests (module)
macro_rules! Depcrate_property_test_optionstests {
() => {
// Module: crate::property_test::options
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use syn :: parse_str ; use super :: * ; # [test] fn simple_parse_example () { let Options { errors , config } = parse_str ("config = (), random = 123") . unwrap () ; assert ! (config . is_some ()) ; assert_eq ! (errors . len () , 1) ; } }
};
}
