macro_rules! deps {
    () => {
        RuntimeCombinedLateLintPass!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        # [allow (rustc :: lint_pass_impl_without_macro)] impl LintPass for RuntimeCombinedLateLintPass < '_ , '_ > { fn name (& self) -> & 'static str { panic ! () } fn get_lints (& self) -> crate :: LintVec { panic ! () } }
    };
}

impl_315!()