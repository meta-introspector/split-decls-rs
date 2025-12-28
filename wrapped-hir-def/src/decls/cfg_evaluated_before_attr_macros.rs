macro_rules! cfg_evaluated_before_attr_macros {
    () => {
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

cfg_evaluated_before_attr_macros!()