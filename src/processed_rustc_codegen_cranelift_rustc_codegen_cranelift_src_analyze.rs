/* FP:analyze.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_analyze_USE_0001
/* FP:analyze.rs-0002 */ use crate :: rustc_index :: IndexVec ;
/* FP:analyze.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_analyze_USE_0002
/* FP:analyze.rs-0004 */ use crate :: rustc_complete :: mir :: StatementKind :: * ;
/* FP:analyze.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_analyze_USE_0003
/* FP:analyze.rs-0006 */ use crate :: prelude :: * ;
/* FP:analyze.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_analyze_ENUM_0004
/* FP:analyze.rs-0008 */ # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub (crate) enum SsaKind { NotSsa , MaybeSsa , }
/* FP:analyze.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_analyze_IMPL_0005
/* FP:analyze.rs-0010 */ impl SsaKind { pub (crate) fn is_ssa < 'tcx > (self , fx : & FunctionCx < '_ , '_ , 'tcx > , ty : Ty < 'tcx >) -> bool { self == SsaKind :: MaybeSsa && (fx . clif_type (ty) . is_some () || fx . clif_pair_type (ty) . is_some ()) } }
/* FP:analyze.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_codegen_cranelift_src_analyze_FN_0006
/* FP:analyze.rs-0012 */ pub (crate) fn analyze (fx : & FunctionCx < '_ , '_ , '_ >) -> IndexVec < Local , SsaKind > { let mut flag_map = fx . mir . local_decls . iter () . map (| _ | SsaKind :: MaybeSsa) . collect :: < IndexVec < Local , SsaKind > > () ; for bb in fx . mir . basic_blocks . iter () { for stmt in bb . statements . iter () { match & stmt . kind { Assign (place_and_rval) => match & place_and_rval . 1 { Rvalue :: Ref (_ , _ , place) | Rvalue :: RawPtr (_ , place) => { flag_map [place . local] = SsaKind :: NotSsa ; } _ => { } } , _ => { } } } } flag_map }