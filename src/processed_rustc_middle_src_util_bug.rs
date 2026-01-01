/* FP:bug.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_USE_0001
/* FP:bug.rs-0002 */ use std :: fmt ;
/* FP:bug.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_USE_0002
/* FP:bug.rs-0004 */ use std :: panic :: { Location , panic_any } ;
/* FP:bug.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_USE_0003
/* FP:bug.rs-0006 */ use crate :: rustc_complete :: MultiSpan ;
/* FP:bug.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_USE_0004
/* FP:bug.rs-0008 */ use crate :: rustc_complete :: Span ;
/* FP:bug.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_USE_0005
/* FP:bug.rs-0010 */ use crate :: ty :: { TyCtxt , tls } ;
/* FP:bug.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_FN_0006
/* FP:bug.rs-0012 */ # [cold] # [inline (never)] # [track_caller] pub fn bug_fmt (args : fmt :: Arguments < '_ >) -> ! { opt_span_bug_fmt (None :: < Span > , args , Location :: caller ()) ; }
/* FP:bug.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_FN_0007
/* FP:bug.rs-0014 */ # [cold] # [inline (never)] # [track_caller] pub fn span_bug_fmt < S : Into < MultiSpan > > (span : S , args : fmt :: Arguments < '_ >) -> ! { opt_span_bug_fmt (Some (span) , args , Location :: caller ()) ; }
/* FP:bug.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_FN_0008
/* FP:bug.rs-0016 */ # [track_caller] fn opt_span_bug_fmt < S : Into < MultiSpan > > (span : Option < S > , args : fmt :: Arguments < '_ > , location : & Location < '_ > ,) -> ! { tls :: with_opt (# [track_caller] move | tcx | { let msg = format ! ("{location}: {args}") ; match (tcx , span) { (Some (tcx) , Some (span)) => tcx . dcx () . span_bug (span , msg) , (Some (tcx) , None) => tcx . dcx () . bug (msg) , (None , _) => panic_any (msg) , } } ,) }
/* FP:bug.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_FN_0009
/* FP:bug.rs-0018 */ # [doc = " A query to trigger a delayed bug. Clearly, if one has a `tcx` one can already trigger a"] # [doc = " delayed bug, so what is the point of this? It exists to help us test the interaction of delayed"] # [doc = " bugs with the query system and incremental."] pub fn trigger_delayed_bug (tcx : TyCtxt < '_ > , key : crate :: rustc_hir :: def_id :: DefId) { tcx . dcx () . span_delayed_bug (tcx . def_span (key) , "delayed bug triggered by #[rustc_delayed_bug_from_inside_query]" ,) ; }
/* FP:bug.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_util_bug_FN_0010
/* FP:bug.rs-0020 */ pub fn provide (providers : & mut crate :: query :: Providers) { * providers = crate :: query :: Providers { trigger_delayed_bug , .. * providers } ; }