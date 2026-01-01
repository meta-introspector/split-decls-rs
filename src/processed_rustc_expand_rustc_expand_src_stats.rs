/* FP:stats.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_USE_0001
/* FP:stats.rs-0002 */ use std :: iter ;
/* FP:stats.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_USE_0002
/* FP:stats.rs-0004 */ use crate :: rustc_complete :: { self as ast , DUMMY_NODE_ID , Expr , ExprKind } ;
/* FP:stats.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_USE_0003
/* FP:stats.rs-0006 */ use rustc_ast_pretty :: pprust ;
/* FP:stats.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_USE_0004
/* FP:stats.rs-0008 */ use crate :: rustc_complete :: hygiene :: { ExpnKind , MacroKind } ;
/* FP:stats.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_USE_0005
/* FP:stats.rs-0010 */ use crate :: rustc_complete :: { Span , Symbol , kw , sym } ;
/* FP:stats.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_USE_0006
/* FP:stats.rs-0012 */ use smallvec :: SmallVec ;
/* FP:stats.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_USE_0007
/* FP:stats.rs-0014 */ use crate :: base :: { Annotatable , ExtCtxt } ;
/* FP:stats.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_USE_0008
/* FP:stats.rs-0016 */ use crate :: expand :: { AstFragment , AstFragmentKind } ;
/* FP:stats.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_STRUCT_0009
/* FP:stats.rs-0018 */ # [derive (Default)] pub struct MacroStat { # [doc = " Number of uses of the macro."] pub uses : usize , # [doc = " Number of lines of code (when pretty-printed)."] pub lines : usize , # [doc = " Number of bytes of code (when pretty-printed)."] pub bytes : usize , }
/* FP:stats.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_FN_0010
/* FP:stats.rs-0020 */ pub (crate) fn elems_to_string < T > (elems : & SmallVec < [T ; 1] > , f : impl Fn (& T) -> String) -> String { let mut s = String :: new () ; for (i , elem) in elems . iter () . enumerate () { if i > 0 { s . push ('\n') ; } s . push_str (& f (elem)) ; } s }
/* FP:stats.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_FN_0011
/* FP:stats.rs-0022 */ pub (crate) fn unreachable_to_string < T > (_ : & T) -> String { unreachable ! () }
/* FP:stats.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_FN_0012
/* FP:stats.rs-0024 */ pub (crate) fn update_bang_macro_stats (ecx : & mut ExtCtxt < '_ > , fragment_kind : AstFragmentKind , span : Span , mac : Box < ast :: MacCall > , fragment : & AstFragment ,) { let is_include_path = mac . path == sym :: include || mac . path == sym :: include_bytes || mac . path == sym :: include_str || mac . path == [sym :: std , sym :: include] . as_slice () || mac . path == [sym :: std , sym :: include_bytes] . as_slice () || mac . path == [sym :: std , sym :: include_str] . as_slice () ; if is_include_path { return ; } let expr = Expr { id : DUMMY_NODE_ID , kind : ExprKind :: MacCall (mac) , span : Default :: default () , attrs : Default :: default () , tokens : None , } ; let input = pprust :: expr_to_string (& expr) ; let ast :: Expr { kind : ExprKind :: MacCall (mac) , .. } = expr else { unreachable ! () } ; update_macro_stats (ecx , MacroKind :: Bang , fragment_kind , span , & mac . path , & input , fragment) ; }
/* FP:stats.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_FN_0013
/* FP:stats.rs-0026 */ pub (crate) fn update_attr_macro_stats (ecx : & mut ExtCtxt < '_ > , fragment_kind : AstFragmentKind , span : Span , path : & ast :: Path , attr : & ast :: Attribute , item : Annotatable , fragment : & AstFragment ,) { let is_derive_path = * path == sym :: derive || * path == [kw :: PathRoot , sym :: core , sym :: prelude , sym :: v1 , sym :: derive] . as_slice () ; if is_derive_path { return ; } let input = format ! ("{}\n{}" , pprust :: attribute_to_string (attr) , fragment_kind . expect_from_annotatables (iter :: once (item)) . to_string () ,) ; update_macro_stats (ecx , MacroKind :: Attr , fragment_kind , span , path , & input , fragment) ; }
/* FP:stats.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_FN_0014
/* FP:stats.rs-0028 */ pub (crate) fn update_derive_macro_stats (ecx : & mut ExtCtxt < '_ > , fragment_kind : AstFragmentKind , span : Span , path : & ast :: Path , fragment : & AstFragment ,) { let input = format ! ("#[derive({})]" , pprust :: path_to_string (path)) ; update_macro_stats (ecx , MacroKind :: Derive , fragment_kind , span , path , & input , fragment) ; }
/* FP:stats.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_expand_src_stats_FN_0015
/* FP:stats.rs-0030 */ pub (crate) fn update_macro_stats (ecx : & mut ExtCtxt < '_ > , macro_kind : MacroKind , fragment_kind : AstFragmentKind , span : Span , path : & ast :: Path , input : & str , fragment : & AstFragment ,) { let name = Symbol :: intern (& pprust :: path_to_string (path)) ; let output = fragment . to_string () ; let num_lines = output . trim_end () . split ('\n') . count () ; let num_bytes = output . len () ; if false { let name = ExpnKind :: Macro (macro_kind , name) . descr () ; let crate_name = & ecx . ecfg . crate_name ; let span = ecx . sess . source_map () . span_to_string (span , crate :: rustc_span :: FileNameDisplayPreference :: Local) ; eprint ! ("\
/* FP:stats.rs-0031 */             -------------------------------\n\
/* FP:stats.rs-0032 */             {name}: [{crate_name}] ({fragment_kind:?}) {span}\n\
/* FP:stats.rs-0033 */             -------------------------------\n\
/* FP:stats.rs-0034 */             {input}\n\
/* FP:stats.rs-0035 */             -- {num_lines} lines, {num_bytes} bytes --\n\
/* FP:stats.rs-0036 */             {output}\n\
/* FP:stats.rs-0037 */         ") ; } let entry = ecx . macro_stats . entry ((name , macro_kind)) . or_insert (MacroStat :: default ()) ; entry . uses += 1 ; entry . lines += num_lines ; entry . bytes += num_bytes ; }