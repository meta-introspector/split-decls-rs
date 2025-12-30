// Generated macro for cfg_evaluated_before_attr_macros (function)
macro_rules! Depcrate_macro_expansion_tests_proc_macroscfg_evaluated_before_attr_macros {
() => {
// Module: crate::macro_expansion_tests::proc_macros
// Provides: {"cfg_evaluated_before_attr_macros"}
// Dependencies: {}
# [test] fn cfg_evaluated_before_attr_macros () { check_errors (r#"
//- proc_macros: disallow_cfg

use proc_macros::disallow_cfg;

#[disallow_cfg] #[cfg(false)] fn foo() {}
// True cfg are kept.
// #[disallow_cfg] #[cfg(true)] fn bar() {}
#[disallow_cfg] #[cfg_attr(false, inline)] fn baz() {}
#[disallow_cfg] #[cfg_attr(true, inline)] fn qux() {}
    "# , expect ! [[r#""#]] ,) ; }
};
}
