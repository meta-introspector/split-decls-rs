macro_rules! deps {
    () => {
        EarlyContext!();
        LintPassByHand!();
    };
}

macro_rules! impl_287 {
    () => {
        deps!();
        impl EarlyLintPass for LintPassImpl { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & ast :: Item) { if let ast :: ItemKind :: Impl (ast :: Impl { of_trait : Some (of_trait) , .. }) = & item . kind && let Some (last) = of_trait . trait_ref . path . segments . last () && last . ident . name == sym :: LintPass { let expn_data = of_trait . trait_ref . path . span . ctxt () . outer_expn_data () ; let call_site = expn_data . call_site ; if expn_data . kind != ExpnKind :: Macro (MacroKind :: Bang , sym :: impl_lint_pass) && call_site . ctxt () . outer_expn_data () . kind != ExpnKind :: Macro (MacroKind :: Bang , sym :: declare_lint_pass) { cx . emit_span_lint (LINT_PASS_IMPL_WITHOUT_MACRO , of_trait . trait_ref . path . span , LintPassByHand ,) ; } } } }
    };
}

impl_287!()