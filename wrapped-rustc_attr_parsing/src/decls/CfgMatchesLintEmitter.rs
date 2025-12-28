macro_rules! CfgMatchesLintEmitter {
    () => {
        # [doc = " Emitter of a builtin lint from `cfg_matches`."] # [doc = ""] # [doc = " Used to support emitting a lint (currently on check-cfg), either:"] # [doc = "  - as an early buffered lint (in `rustc`)"] # [doc = "  - or has a \"normal\" lint from HIR (in `rustdoc`)"] pub trait CfgMatchesLintEmitter { fn emit_span_lint (& self , sess : & Session , lint : & 'static Lint , sp : Span , diag : BuiltinLintDiag) ; }
    };
}

CfgMatchesLintEmitter!();