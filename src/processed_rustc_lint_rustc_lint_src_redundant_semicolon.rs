/* FP:redundant_semicolon.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_redundant_semicolon_USE_0001
/* FP:redundant_semicolon.rs-0002 */ use crate :: rustc_complete :: { Block , StmtKind } ;
/* FP:redundant_semicolon.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_redundant_semicolon_USE_0002
/* FP:redundant_semicolon.rs-0004 */ use crate :: rustc_complete :: { declare_lint , declare_lint_pass } ;
/* FP:redundant_semicolon.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_redundant_semicolon_USE_0003
/* FP:redundant_semicolon.rs-0006 */ use crate :: rustc_complete :: Span ;
/* FP:redundant_semicolon.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_redundant_semicolon_USE_0004
/* FP:redundant_semicolon.rs-0008 */ use crate :: lints :: { RedundantSemicolonsDiag , RedundantSemicolonsSuggestion } ;
/* FP:redundant_semicolon.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_redundant_semicolon_USE_0005
/* FP:redundant_semicolon.rs-0010 */ use crate :: { EarlyContext , EarlyLintPass , LintContext } ;
/* FP:redundant_semicolon.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_redundant_semicolon_MACRO_0006
/* FP:redundant_semicolon.rs-0012 */ declare_lint ! { # [doc = " The `redundant_semicolons` lint detects unnecessary trailing"] # [doc = " semicolons."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " let _ = 123;;"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Extra semicolons are not needed, and may be removed to avoid confusion"] # [doc = " and visual clutter."] pub REDUNDANT_SEMICOLONS , Warn , "detects unnecessary trailing semicolons" }
/* FP:redundant_semicolon.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_redundant_semicolon_MACRO_0007
/* FP:redundant_semicolon.rs-0014 */ declare_lint_pass ! (RedundantSemicolons => [REDUNDANT_SEMICOLONS]) ;
/* FP:redundant_semicolon.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_redundant_semicolon_IMPL_0008
/* FP:redundant_semicolon.rs-0016 */ impl EarlyLintPass for RedundantSemicolons { fn check_block (& mut self , cx : & EarlyContext < '_ > , block : & Block) { let mut seq = None ; for stmt in block . stmts . iter () { match (& stmt . kind , & mut seq) { (StmtKind :: Empty , None) => seq = Some ((stmt . span , false)) , (StmtKind :: Empty , Some (seq)) => * seq = (seq . 0 . to (stmt . span) , true) , (_ , seq) => maybe_lint_redundant_semis (cx , seq) , } } maybe_lint_redundant_semis (cx , & mut seq) ; } }
/* FP:redundant_semicolon.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_lint_src_redundant_semicolon_FN_0009
/* FP:redundant_semicolon.rs-0018 */ fn maybe_lint_redundant_semis (cx : & EarlyContext < '_ > , seq : & mut Option < (Span , bool) >) { if let Some ((span , multiple)) = seq . take () { if span == crate :: rustc_span :: DUMMY_SP { return ; } let suggestion = if span . from_expansion () { None } else { Some (RedundantSemicolonsSuggestion { multiple_semicolons : multiple , span }) } ; cx . emit_span_lint (REDUNDANT_SEMICOLONS , span , RedundantSemicolonsDiag { multiple , suggestion } ,) ; } }