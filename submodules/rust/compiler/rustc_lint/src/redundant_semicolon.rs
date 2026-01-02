mkuse!{use rustc_ast :: { Block , StmtKind } ;}
mkuse!{use rustc_session :: { declare_lint , declare_lint_pass } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use crate :: lints :: { RedundantSemicolonsDiag , RedundantSemicolonsSuggestion } ;}
mkuse!{use crate :: { EarlyContext , EarlyLintPass , LintContext } ;}
mkitem!{declare_lint ! { # [doc = " The `redundant_semicolons` lint detects unnecessary trailing"] # [doc = " semicolons."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let _ = 123;;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Extra semicolons are not needed, and may be removed to avoid confusion"] # [doc = " and visual clutter."] pub REDUNDANT_SEMICOLONS , Warn , "detects unnecessary trailing semicolons" }}
mkitem!{declare_lint_pass ! (RedundantSemicolons => [REDUNDANT_SEMICOLONS]) ;}
mkitem!{mkimpl!{impl EarlyLintPass for RedundantSemicolons { fn check_block (& mut self , cx : & EarlyContext < '_ > , block : & Block) { let mut seq = None ; for stmt in block . stmts . iter () { match (& stmt . kind , & mut seq) { (StmtKind :: Empty , None) => seq = Some ((stmt . span , false)) , (StmtKind :: Empty , Some (seq)) => * seq = (seq . 0 . to (stmt . span) , true) , (_ , seq) => maybe_lint_redundant_semis (cx , seq) , } } maybe_lint_redundant_semis (cx , & mut seq) ; } }}}

macro_rules! maybe_lint_redundant_semis_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function maybe_lint_redundant_semis in module {}", module_path!());
    };
}

mkfn!{
    maybe_lint_redundant_semis_introspect!();
    fn maybe_lint_redundant_semis (cx : & EarlyContext < '_ > , seq : & mut Option < (Span , bool) >) { if let Some ((span , multiple)) = seq . take () { if span == rustc_span :: DUMMY_SP { return ; } let suggestion = if span . from_expansion () { None } else { Some (RedundantSemicolonsSuggestion { multiple_semicolons : multiple , span }) } ; cx . emit_span_lint (REDUNDANT_SEMICOLONS , span , RedundantSemicolonsDiag { multiple , suggestion } ,) ; } }
}