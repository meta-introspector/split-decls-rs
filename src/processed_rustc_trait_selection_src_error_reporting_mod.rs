/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_USE_0001
/* FP:mod.rs-0002 */ use std :: ops :: Deref ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_USE_0002
/* FP:mod.rs-0004 */ use crate :: rustc_complete :: DiagCtxtHandle ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_infer :: infer :: InferCtxt ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_USE_0004
/* FP:mod.rs-0008 */ use crate :: rustc_infer :: traits :: PredicateObligations ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_USE_0005
/* FP:mod.rs-0010 */ use rustc_macros :: extension ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_USE_0006
/* FP:mod.rs-0012 */ use crate :: rustc_complete :: bug ;
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_USE_0007
/* FP:mod.rs-0014 */ use crate :: rustc_complete :: ty :: { self , Ty } ;
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_MOD_0008
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_MOD_0009
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_STRUCT_0010
/* FP:mod.rs-0020 */ # [doc = " A helper for building type related errors. The `typeck_results`"] # [doc = " field is only populated during an in-progress typeck."] # [doc = " Get an instance by calling `InferCtxt::err_ctxt` or `FnCtxt::err_ctxt`."] # [doc = ""] # [doc = " You must only create this if you intend to actually emit an error (or"] # [doc = " perhaps a warning, though preferably not.) It provides a lot of utility"] # [doc = " methods which should not be used during the happy path."] pub struct TypeErrCtxt < 'a , 'tcx > { pub infcx : & 'a InferCtxt < 'tcx > , pub typeck_results : Option < std :: cell :: Ref < 'a , ty :: TypeckResults < 'tcx > > > , pub fallback_has_occurred : bool , pub normalize_fn_sig : Box < dyn Fn (ty :: PolyFnSig < 'tcx >) -> ty :: PolyFnSig < 'tcx > + 'a > , pub autoderef_steps : Box < dyn Fn (Ty < 'tcx >) -> Vec < (Ty < 'tcx > , PredicateObligations < 'tcx >) > + 'a > , }
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_IMPL_0011
/* FP:mod.rs-0022 */ # [extension (pub trait InferCtxtErrorExt <'tcx >)] impl < 'tcx > InferCtxt < 'tcx > { # [doc = " Creates a `TypeErrCtxt` for emitting various inference errors."] # [doc = " During typeck, use `FnCtxt::err_ctxt` instead."] fn err_ctxt (& self) -> TypeErrCtxt < '_ , 'tcx > { TypeErrCtxt { infcx : self , typeck_results : None , fallback_has_occurred : false , normalize_fn_sig : Box :: new (| fn_sig | fn_sig) , autoderef_steps : Box :: new (| ty | { debug_assert ! (false , "shouldn't be using autoderef_steps outside of typeck") ; vec ! [(ty , PredicateObligations :: new ())] }) , } } }
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_IMPL_0012
/* FP:mod.rs-0024 */ impl < 'a , 'tcx > TypeErrCtxt < 'a , 'tcx > { pub fn dcx (& self) -> DiagCtxtHandle < 'a > { self . infcx . dcx () } # [doc = " This is just to avoid a potential footgun of accidentally"] # [doc = " dropping `typeck_results` by calling `InferCtxt::err_ctxt`"] # [deprecated (note = "you already have a `TypeErrCtxt`")] # [allow (unused)] pub fn err_ctxt (& self) -> ! { bug ! ("called `err_ctxt` on `TypeErrCtxt`. Try removing the call") ; } }
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_trait_selection_src_error_reporting_mod_IMPL_0013
/* FP:mod.rs-0026 */ impl < 'tcx > Deref for TypeErrCtxt < '_ , 'tcx > { type Target = InferCtxt < 'tcx > ; fn deref (& self) -> & InferCtxt < 'tcx > { self . infcx } }