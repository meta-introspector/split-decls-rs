macro_rules! deps {
    () => {
        EarlyContext!();
    };
}

macro_rules! impl_652 {
    () => {
        deps!();
        impl EarlyLintPass for Expr2024 { fn check_mac_def (& mut self , cx : & crate :: EarlyContext < '_ > , mc : & rustc_ast :: MacroDef) { self . check_tokens (cx , & mc . body . tokens) ; } }
    };
}

impl_652!();