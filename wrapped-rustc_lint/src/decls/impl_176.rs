macro_rules! deps {
    () => {
        RuntimeCombinedEarlyLintPass!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        # [allow (rustc :: lint_pass_impl_without_macro)] impl LintPass for RuntimeCombinedEarlyLintPass < '_ > { fn name (& self) -> & 'static str { panic ! () } fn get_lints (& self) -> crate :: LintVec { panic ! () } }
    };
}

impl_176!()