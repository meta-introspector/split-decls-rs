macro_rules! deps {
    () => {
        EarlyContext!();
        UnderMacro!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl EarlyLintPass for KeywordIdents { fn check_mac_def (& mut self , cx : & EarlyContext < '_ > , mac_def : & ast :: MacroDef) { self . check_tokens (cx , & mac_def . body . tokens) ; } fn check_mac (& mut self , cx : & EarlyContext < '_ > , mac : & ast :: MacCall) { self . check_tokens (cx , & mac . args . tokens) ; } fn check_ident (& mut self , cx : & EarlyContext < '_ > , ident : & Ident) { if ident . name . as_str () . starts_with ('\'') { self . check_ident_token (cx , UnderMacro (false) , ident . without_first_quote () , "'") ; } else { self . check_ident_token (cx , UnderMacro (false) , * ident , "") ; } } }
    };
}

impl_86!();