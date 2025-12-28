macro_rules! deps {
    () => {
        EarlyContext!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl UnsafeCode { fn report_unsafe (& self , cx : & EarlyContext < '_ > , span : Span , decorate : impl for < 'a > LintDiagnostic < 'a , () > ,) { if span . allows_unsafe () { return ; } cx . emit_span_lint (UNSAFE_CODE , span , decorate) ; } }
    };
}

impl_25!()