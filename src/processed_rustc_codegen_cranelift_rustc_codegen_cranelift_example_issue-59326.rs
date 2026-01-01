/* FP:issue-59326.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-59326_TRAIT_0001
/* FP:issue-59326.rs-0002 */ trait Service { type S ; }
/* FP:issue-59326.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-59326_TRAIT_0002
/* FP:issue-59326.rs-0004 */ trait Framing { type F ; }
/* FP:issue-59326.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-59326_IMPL_0003
/* FP:issue-59326.rs-0006 */ impl Framing for () { type F = () ; }
/* FP:issue-59326.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-59326_TRAIT_0004
/* FP:issue-59326.rs-0008 */ trait HttpService < F : Framing > : Service < S = F :: F > { }
/* FP:issue-59326.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-59326_TYPE_0005
/* FP:issue-59326.rs-0010 */ type BoxService = Box < dyn HttpService < () , S = () > > ;
/* FP:issue-59326.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-59326_FN_0006
/* FP:issue-59326.rs-0012 */ fn build_server < F : FnOnce () -> BoxService > (_ : F) { }
/* FP:issue-59326.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-59326_FN_0007
/* FP:issue-59326.rs-0014 */ fn make_server < F : Framing > () -> Box < dyn HttpService < F , S = F :: F > > { unimplemented ! () }
/* FP:issue-59326.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_example_issue-59326_FN_0008
/* FP:issue-59326.rs-0016 */ fn main () { build_server (| | make_server ()) }