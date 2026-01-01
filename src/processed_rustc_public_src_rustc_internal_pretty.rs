/* FP:pretty.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_rustc_internal_pretty_USE_0001
/* FP:pretty.rs-0002 */ use std :: io ;
/* FP:pretty.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_rustc_internal_pretty_USE_0002
/* FP:pretty.rs-0004 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:pretty.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_rustc_internal_pretty_USE_0003
/* FP:pretty.rs-0006 */ use super :: run ;
/* FP:pretty.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_rustc_internal_pretty_FN_0004
/* FP:pretty.rs-0008 */ pub fn write_smir_pretty < 'tcx , W : io :: Write > (tcx : TyCtxt < 'tcx > , w : & mut W) -> io :: Result < () > { writeln ! (w , "// WARNING: This is highly experimental output it's intended for rustc_public developers only.") ? ; writeln ! (w , "// If you find a bug or want to improve the output open a issue at https://github.com/rust-lang/project-stable-mir.") ? ; let _ = run (tcx , | | { let items = crate :: all_local_items () ; let _ = items . iter () . map (| item | -> io :: Result < () > { item . emit_mir (w) }) . collect :: < Vec < _ > > () ; }) ; Ok (()) }