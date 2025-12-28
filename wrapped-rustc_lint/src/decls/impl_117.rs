macro_rules! deps {
    () => {
        LintGroup!();
        LintStore!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl DynLintStore for LintStore { fn lint_groups_iter (& self) -> Box < dyn Iterator < Item = rustc_session :: LintGroup > + '_ > { Box :: new (self . get_lint_groups () . map (| (name , lints , is_externally_loaded) | { rustc_session :: LintGroup { name , lints , is_externally_loaded } })) } }
    };
}

impl_117!()