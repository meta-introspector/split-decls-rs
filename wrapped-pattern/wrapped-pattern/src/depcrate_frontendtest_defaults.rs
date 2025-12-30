// Generated macro for test_defaults (function)
macro_rules! Depcrate_frontendtest_defaults {
() => {
// Module: crate::frontend
// Provides: {"test_defaults"}
// Dependencies: {}
# [test] fn test_defaults () { assert_eq ! (Box ::< Pattern ::< crate :: SinglePlaceholder >>:: default () , Pattern :: try_from_items (core :: iter :: empty ()) . unwrap ()) ; assert_eq ! (Box ::< Pattern ::< crate :: DoublePlaceholder >>:: default () , Pattern :: try_from_items (core :: iter :: empty ()) . unwrap ()) ; assert_eq ! (Box ::< Pattern ::< crate :: MultiNamedPlaceholder >>:: default () , Pattern :: try_from_items (core :: iter :: empty ()) . unwrap ()) ; }
};
}
