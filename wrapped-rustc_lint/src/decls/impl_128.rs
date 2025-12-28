macro_rules! deps {
    () => {
        LintContext!();
        EarlyContext!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl LintContext for EarlyContext < '_ > { # [doc = " Gets the overall compiler `Session` object."] fn sess (& self) -> & Session { self . builder . sess () } # [rustc_lint_diagnostics] fn opt_span_lint < S : Into < MultiSpan > > (& self , lint : & 'static Lint , span : Option < S > , decorate : impl for < 'a , 'b > FnOnce (& 'b mut Diag < 'a , () >) ,) { self . builder . opt_span_lint (lint , span . map (| s | s . into ()) , decorate) } fn get_lint_level (& self , lint : & 'static Lint) -> LevelAndSource { self . builder . lint_level (lint) } }
    };
}

impl_128!()