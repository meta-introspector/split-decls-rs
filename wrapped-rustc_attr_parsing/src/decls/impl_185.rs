macro_rules! deps {
    () => {
        Stage!();
        AcceptContext!();
        MultipleStabilityLevels!();
        StabilityParser!();
    };
}

macro_rules! impl_185 {
    () => {
        deps!();
        impl StabilityParser { # [doc = " Checks, and emits an error when a stability (or unstability) was already set, which would be a duplicate."] fn check_duplicate < S : Stage > (& self , cx : & AcceptContext < '_ , '_ , S >) -> bool { if let Some ((_ , _)) = self . stability { cx . emit_err (session_diagnostics :: MultipleStabilityLevels { span : cx . attr_span }) ; true } else { false } } }
    };
}

impl_185!();