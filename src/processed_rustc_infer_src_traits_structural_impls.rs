/* FP:structural_impls.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_structural_impls_USE_0001
/* FP:structural_impls.rs-0002 */ use std :: fmt ;
/* FP:structural_impls.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_structural_impls_USE_0002
/* FP:structural_impls.rs-0004 */ use crate :: rustc_complete :: ty ;
/* FP:structural_impls.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_structural_impls_USE_0003
/* FP:structural_impls.rs-0006 */ use crate :: traits ;
/* FP:structural_impls.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_structural_impls_USE_0004
/* FP:structural_impls.rs-0008 */ use crate :: traits :: project :: Normalized ;
/* FP:structural_impls.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_structural_impls_IMPL_0005
/* FP:structural_impls.rs-0010 */ impl < 'tcx , T : fmt :: Debug > fmt :: Debug for Normalized < 'tcx , T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "Normalized({:?}, {:?})" , self . value , self . obligations) } }
/* FP:structural_impls.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_structural_impls_IMPL_0006
/* FP:structural_impls.rs-0012 */ impl < 'tcx , O : fmt :: Debug > fmt :: Debug for traits :: Obligation < 'tcx , O > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if ty :: tls :: with (| tcx | tcx . sess . verbose_internals ()) { write ! (f , "Obligation(predicate={:?}, cause={:?}, param_env={:?}, depth={})" , self . predicate , self . cause , self . param_env , self . recursion_depth) } else { write ! (f , "Obligation(predicate={:?}, depth={})" , self . predicate , self . recursion_depth) } } }
/* FP:structural_impls.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_structural_impls_IMPL_0007
/* FP:structural_impls.rs-0014 */ impl < 'tcx > fmt :: Debug for traits :: MismatchedProjectionTypes < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "MismatchedProjectionTypes({:?})" , self . err) } }