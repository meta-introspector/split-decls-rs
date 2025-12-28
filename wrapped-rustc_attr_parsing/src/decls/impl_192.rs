macro_rules! deps {
    () => {
        ConstStabilityParser!();
        AcceptContext!();
        MultipleStabilityLevels!();
        Stage!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl ConstStabilityParser { # [doc = " Checks, and emits an error when a stability (or unstability) was already set, which would be a duplicate."] fn check_duplicate < S : Stage > (& self , cx : & AcceptContext < '_ , '_ , S >) -> bool { if let Some ((_ , _)) = self . stability { cx . emit_err (session_diagnostics :: MultipleStabilityLevels { span : cx . attr_span }) ; true } else { false } } }
    };
}

impl_192!();