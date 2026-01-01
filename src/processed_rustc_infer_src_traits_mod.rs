/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_MOD_0001
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_MOD_0002
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_MOD_0003
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_MOD_0004
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0005
/* FP:mod.rs-0010 */ use std :: cmp ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0006
/* FP:mod.rs-0012 */ use std :: hash :: { Hash , Hasher } ;
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0007
/* FP:mod.rs-0014 */ use hir :: def_id :: LocalDefId ;
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0008
/* FP:mod.rs-0016 */ use rustc_hir as hir ;
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0009
/* FP:mod.rs-0018 */ use rustc_macros :: { TypeFoldable , TypeVisitable } ;
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0010
/* FP:mod.rs-0020 */ use crate :: rustc_complete :: traits :: query :: NoSolution ;
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0011
/* FP:mod.rs-0022 */ use crate :: rustc_complete :: traits :: solve :: Certainty ;
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0012
/* FP:mod.rs-0024 */ pub use crate :: rustc_complete :: traits :: * ;
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0013
/* FP:mod.rs-0026 */ use crate :: rustc_complete :: ty :: { self , Ty , TyCtxt , Upcast } ;
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0014
/* FP:mod.rs-0028 */ use crate :: rustc_complete :: Span ;
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0015
/* FP:mod.rs-0030 */ use thin_vec :: ThinVec ;
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0016
/* FP:mod.rs-0032 */ pub use self :: engine :: { FromSolverError , ScrubbedTraitError , TraitEngine } ;
/* FP:mod.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0017
/* FP:mod.rs-0034 */ pub (crate) use self :: project :: UndoLog ;
/* FP:mod.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0018
/* FP:mod.rs-0036 */ pub use self :: project :: { MismatchedProjectionTypes , Normalized , NormalizedTerm , ProjectionCache , ProjectionCacheEntry , ProjectionCacheKey , ProjectionCacheStorage , } ;
/* FP:mod.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_USE_0019
/* FP:mod.rs-0038 */ use crate :: infer :: InferCtxt ;
/* FP:mod.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_STRUCT_0020
/* FP:mod.rs-0040 */ # [doc = " An `Obligation` represents some trait reference (e.g., `i32: Eq`) for"] # [doc = " which the \"impl_source\" must be found. The process of finding an \"impl_source\" is"] # [doc = " called \"resolving\" the `Obligation`. This process consists of"] # [doc = " either identifying an `impl` (e.g., `impl Eq for i32`) that"] # [doc = " satisfies the obligation, or else finding a bound that is in"] # [doc = " scope. The eventual result is usually a `Selection` (defined below)."] # [derive (Clone , TypeFoldable , TypeVisitable)] pub struct Obligation < 'tcx , T > { # [doc = " The reason we have to prove this thing."] # [type_foldable (identity)] # [type_visitable (ignore)] pub cause : ObligationCause < 'tcx > , # [doc = " The environment in which we should prove this thing."] pub param_env : ty :: ParamEnv < 'tcx > , # [doc = " The thing we are trying to prove."] pub predicate : T , # [doc = " If we started proving this as a result of trying to prove"] # [doc = " something else, track the total depth to ensure termination."] # [doc = " If this goes over a certain threshold, we abort compilation --"] # [doc = " in such cases, we can not say whether or not the predicate"] # [doc = " holds for certain. Stupid halting problem; such a drag."] # [type_foldable (identity)] # [type_visitable (ignore)] pub recursion_depth : usize , }
/* FP:mod.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_IMPL_0021
/* FP:mod.rs-0042 */ impl < 'tcx , T : Copy > Obligation < 'tcx , T > { pub fn as_goal (& self) -> solve :: Goal < 'tcx , T > { solve :: Goal { param_env : self . param_env , predicate : self . predicate } } }
/* FP:mod.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_IMPL_0022
/* FP:mod.rs-0044 */ impl < 'tcx , T : PartialEq > PartialEq < Obligation < 'tcx , T > > for Obligation < 'tcx , T > { # [inline] fn eq (& self , other : & Obligation < 'tcx , T >) -> bool { self . param_env == other . param_env && self . predicate == other . predicate } }
/* FP:mod.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_IMPL_0023
/* FP:mod.rs-0046 */ impl < T : Eq > Eq for Obligation < '_ , T > { }
/* FP:mod.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_IMPL_0024
/* FP:mod.rs-0048 */ impl < T : Hash > Hash for Obligation < '_ , T > { fn hash < H : Hasher > (& self , state : & mut H) -> () { self . param_env . hash (state) ; self . predicate . hash (state) ; } }
/* FP:mod.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_TYPE_0025
/* FP:mod.rs-0050 */ pub type PredicateObligation < 'tcx > = Obligation < 'tcx , ty :: Predicate < 'tcx > > ;
/* FP:mod.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_TYPE_0026
/* FP:mod.rs-0052 */ pub type TraitObligation < 'tcx > = Obligation < 'tcx , ty :: TraitPredicate < 'tcx > > ;
/* FP:mod.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_TYPE_0027
/* FP:mod.rs-0054 */ pub type PolyTraitObligation < 'tcx > = Obligation < 'tcx , ty :: PolyTraitPredicate < 'tcx > > ;
/* FP:mod.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_TYPE_0028
/* FP:mod.rs-0056 */ pub type PredicateObligations < 'tcx > = ThinVec < PredicateObligation < 'tcx > > ;
/* FP:mod.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_IMPL_0029
/* FP:mod.rs-0058 */ impl < 'tcx > PredicateObligation < 'tcx > { # [doc = " Flips the polarity of the inner predicate."] # [doc = ""] # [doc = " Given `T: Trait` predicate it returns `T: !Trait` and given `T: !Trait` returns `T: Trait`."] pub fn flip_polarity (& self , tcx : TyCtxt < 'tcx >) -> Option < PredicateObligation < 'tcx > > { Some (PredicateObligation { cause : self . cause . clone () , param_env : self . param_env , predicate : self . predicate . flip_polarity (tcx) ? , recursion_depth : self . recursion_depth , }) } }
/* FP:mod.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_IMPL_0030
/* FP:mod.rs-0060 */ impl < 'tcx > PolyTraitObligation < 'tcx > { pub fn derived_cause (& self , variant : impl FnOnce (DerivedCause < 'tcx >) -> ObligationCauseCode < 'tcx > ,) -> ObligationCause < 'tcx > { self . cause . clone () . derived_cause (self . predicate , variant) } }
/* FP:mod.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_MACRO_0031
/* FP:mod.rs-0062 */ # [cfg (target_pointer_width = "64")] crate :: rustc_data_structures :: static_assert_size ! (PredicateObligation <'_ >, 48) ;
/* FP:mod.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_TYPE_0032
/* FP:mod.rs-0064 */ pub type Selection < 'tcx > = ImplSource < 'tcx , PredicateObligation < 'tcx > > ;
/* FP:mod.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_TYPE_0033
/* FP:mod.rs-0066 */ # [doc = " A callback that can be provided to `inspect_typeck`. Invoked on evaluation"] # [doc = " of root obligations."] pub type ObligationInspector < 'tcx > = fn (& InferCtxt < 'tcx > , & PredicateObligation < 'tcx > , Result < Certainty , NoSolution >) ;
/* FP:mod.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_IMPL_0034
/* FP:mod.rs-0068 */ impl < 'tcx , O > Obligation < 'tcx , O > { pub fn new (tcx : TyCtxt < 'tcx > , cause : ObligationCause < 'tcx > , param_env : ty :: ParamEnv < 'tcx > , predicate : impl Upcast < TyCtxt < 'tcx > , O > ,) -> Obligation < 'tcx , O > { Self :: with_depth (tcx , cause , 0 , param_env , predicate) } # [doc = " We often create nested obligations without setting the correct depth."] # [doc = ""] # [doc = " To deal with this evaluate and fulfill explicitly update the depth"] # [doc = " of nested obligations using this function."] pub fn set_depth_from_parent (& mut self , parent_depth : usize) { self . recursion_depth = cmp :: max (parent_depth + 1 , self . recursion_depth) ; } pub fn with_depth (tcx : TyCtxt < 'tcx > , cause : ObligationCause < 'tcx > , recursion_depth : usize , param_env : ty :: ParamEnv < 'tcx > , predicate : impl Upcast < TyCtxt < 'tcx > , O > ,) -> Obligation < 'tcx , O > { let predicate = predicate . upcast (tcx) ; Obligation { cause , param_env , recursion_depth , predicate } } pub fn misc (tcx : TyCtxt < 'tcx > , span : Span , body_id : LocalDefId , param_env : ty :: ParamEnv < 'tcx > , trait_ref : impl Upcast < TyCtxt < 'tcx > , O > ,) -> Obligation < 'tcx , O > { Obligation :: new (tcx , ObligationCause :: misc (span , body_id) , param_env , trait_ref) } pub fn with < P > (& self , tcx : TyCtxt < 'tcx > , value : impl Upcast < TyCtxt < 'tcx > , P > ,) -> Obligation < 'tcx , P > { Obligation :: with_depth (tcx , self . cause . clone () , self . recursion_depth , self . param_env , value) } }
/* FP:mod.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_infer_src_traits_mod_IMPL_0035
/* FP:mod.rs-0070 */ impl < 'tcx > PolyTraitObligation < 'tcx > { pub fn polarity (& self) -> ty :: PredicatePolarity { self . predicate . skip_binder () . polarity } pub fn self_ty (& self) -> ty :: Binder < 'tcx , Ty < 'tcx > > { self . predicate . map_bound (| p | p . self_ty ()) } }