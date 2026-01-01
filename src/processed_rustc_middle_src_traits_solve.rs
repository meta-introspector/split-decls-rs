/* FP:solve.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_USE_0001
/* FP:solve.rs-0002 */ use crate :: rustc_data_structures :: intern :: Interned ;
/* FP:solve.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_USE_0002
/* FP:solve.rs-0004 */ use rustc_macros :: HashStable ;
/* FP:solve.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_USE_0003
/* FP:solve.rs-0006 */ use rustc_type_ir as ir ;
/* FP:solve.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_USE_0004
/* FP:solve.rs-0008 */ pub use rustc_type_ir :: solve :: * ;
/* FP:solve.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_USE_0005
/* FP:solve.rs-0010 */ use crate :: ty :: { self , FallibleTypeFolder , TyCtxt , TypeFoldable , TypeFolder , TypeVisitable , TypeVisitor , try_visit , } ;
/* FP:solve.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_TYPE_0006
/* FP:solve.rs-0012 */ pub type Goal < 'tcx , P > = ir :: solve :: Goal < TyCtxt < 'tcx > , P > ;
/* FP:solve.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_TYPE_0007
/* FP:solve.rs-0014 */ pub type QueryInput < 'tcx , P > = ir :: solve :: QueryInput < TyCtxt < 'tcx > , P > ;
/* FP:solve.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_TYPE_0008
/* FP:solve.rs-0016 */ pub type QueryResult < 'tcx > = ir :: solve :: QueryResult < TyCtxt < 'tcx > > ;
/* FP:solve.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_TYPE_0009
/* FP:solve.rs-0018 */ pub type CandidateSource < 'tcx > = ir :: solve :: CandidateSource < TyCtxt < 'tcx > > ;
/* FP:solve.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_TYPE_0010
/* FP:solve.rs-0020 */ pub type CanonicalInput < 'tcx , P = ty :: Predicate < 'tcx > > = ir :: solve :: CanonicalInput < TyCtxt < 'tcx > , P > ;
/* FP:solve.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_TYPE_0011
/* FP:solve.rs-0022 */ pub type CanonicalResponse < 'tcx > = ir :: solve :: CanonicalResponse < TyCtxt < 'tcx > > ;
/* FP:solve.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_STRUCT_0012
/* FP:solve.rs-0024 */ # [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , HashStable)] pub struct PredefinedOpaques < 'tcx > (pub (crate) Interned < 'tcx , PredefinedOpaquesData < TyCtxt < 'tcx > > >) ;
/* FP:solve.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_IMPL_0013
/* FP:solve.rs-0026 */ impl < 'tcx > std :: ops :: Deref for PredefinedOpaques < 'tcx > { type Target = PredefinedOpaquesData < TyCtxt < 'tcx > > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
/* FP:solve.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_STRUCT_0014
/* FP:solve.rs-0028 */ # [derive (Debug , PartialEq , Eq , Copy , Clone , Hash , HashStable)] pub struct ExternalConstraints < 'tcx > (pub (crate) Interned < 'tcx , ExternalConstraintsData < TyCtxt < 'tcx > > > ,) ;
/* FP:solve.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_IMPL_0015
/* FP:solve.rs-0030 */ impl < 'tcx > std :: ops :: Deref for ExternalConstraints < 'tcx > { type Target = ExternalConstraintsData < TyCtxt < 'tcx > > ; fn deref (& self) -> & Self :: Target { & self . 0 } }
/* FP:solve.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_IMPL_0016
/* FP:solve.rs-0032 */ impl < 'tcx > TypeFoldable < TyCtxt < 'tcx > > for ExternalConstraints < 'tcx > { fn try_fold_with < F : FallibleTypeFolder < TyCtxt < 'tcx > > > (self , folder : & mut F ,) -> Result < Self , F :: Error > { if self . is_empty () { return Ok (self) ; } Ok (FallibleTypeFolder :: cx (folder) . mk_external_constraints (ExternalConstraintsData { region_constraints : self . region_constraints . clone () . try_fold_with (folder) ? , opaque_types : self . opaque_types . iter () . map (| opaque | opaque . try_fold_with (folder)) . collect :: < Result < _ , F :: Error > > () ? , normalization_nested_goals : self . normalization_nested_goals . clone () . try_fold_with (folder) ? , })) } fn fold_with < F : TypeFolder < TyCtxt < 'tcx > > > (self , folder : & mut F) -> Self { if self . is_empty () { return self ; } TypeFolder :: cx (folder) . mk_external_constraints (ExternalConstraintsData { region_constraints : self . region_constraints . clone () . fold_with (folder) , opaque_types : self . opaque_types . iter () . map (| opaque | opaque . fold_with (folder)) . collect () , normalization_nested_goals : self . normalization_nested_goals . clone () . fold_with (folder) , }) } }
/* FP:solve.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_IMPL_0017
/* FP:solve.rs-0034 */ impl < 'tcx > TypeVisitable < TyCtxt < 'tcx > > for ExternalConstraints < 'tcx > { fn visit_with < V : TypeVisitor < TyCtxt < 'tcx > > > (& self , visitor : & mut V) -> V :: Result { try_visit ! (self . region_constraints . visit_with (visitor)) ; try_visit ! (self . opaque_types . visit_with (visitor)) ; self . normalization_nested_goals . visit_with (visitor) } }
/* FP:solve.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_IMPL_0018
/* FP:solve.rs-0036 */ impl < 'tcx > TypeFoldable < TyCtxt < 'tcx > > for PredefinedOpaques < 'tcx > { fn try_fold_with < F : FallibleTypeFolder < TyCtxt < 'tcx > > > (self , folder : & mut F ,) -> Result < Self , F :: Error > { Ok (FallibleTypeFolder :: cx (folder) . mk_predefined_opaques_in_body (PredefinedOpaquesData { opaque_types : self . opaque_types . iter () . map (| opaque | opaque . try_fold_with (folder)) . collect :: < Result < _ , F :: Error > > () ? , })) } fn fold_with < F : TypeFolder < TyCtxt < 'tcx > > > (self , folder : & mut F) -> Self { TypeFolder :: cx (folder) . mk_predefined_opaques_in_body (PredefinedOpaquesData { opaque_types : self . opaque_types . iter () . map (| opaque | opaque . fold_with (folder)) . collect () , }) } }
/* FP:solve.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_traits_solve_IMPL_0019
/* FP:solve.rs-0038 */ impl < 'tcx > TypeVisitable < TyCtxt < 'tcx > > for PredefinedOpaques < 'tcx > { fn visit_with < V : TypeVisitor < TyCtxt < 'tcx > > > (& self , visitor : & mut V) -> V :: Result { self . opaque_types . visit_with (visitor) } }