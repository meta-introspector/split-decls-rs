macro_rules! deps {
    () => {
        CfgMatchesLintEmitter!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl CfgMatchesLintEmitter for NodeId { fn emit_span_lint (& self , sess : & Session , lint : & 'static Lint , sp : Span , diag : BuiltinLintDiag) { sess . psess . buffer_lint (lint , sp , * self , diag) ; } }
    };
}

impl_23!();