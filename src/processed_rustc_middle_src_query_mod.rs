/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0001
/* FP:mod.rs-0002 */ # [allow (unused_parens)] use std :: ffi :: OsStr ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0002
/* FP:mod.rs-0004 */ use std :: mem ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0003
/* FP:mod.rs-0006 */ use std :: path :: PathBuf ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0004
/* FP:mod.rs-0008 */ use std :: sync :: Arc ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0005
/* FP:mod.rs-0010 */ use crate :: rustc_abi :: Align ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0006
/* FP:mod.rs-0012 */ use rustc_arena :: TypedArena ;
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0007
/* FP:mod.rs-0014 */ use crate :: rustc_complete :: expand :: allocator :: AllocatorKind ;
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0008
/* FP:mod.rs-0016 */ use crate :: rustc_data_structures :: fingerprint :: Fingerprint ;
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0009
/* FP:mod.rs-0018 */ use crate :: rustc_data_structures :: fx :: { FxIndexMap , FxIndexSet } ;
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0010
/* FP:mod.rs-0020 */ use crate :: rustc_data_structures :: sorted_map :: SortedMap ;
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0011
/* FP:mod.rs-0022 */ use crate :: rustc_data_structures :: steal :: Steal ;
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0012
/* FP:mod.rs-0024 */ use crate :: rustc_data_structures :: svh :: Svh ;
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0013
/* FP:mod.rs-0026 */ use crate :: rustc_data_structures :: unord :: { UnordMap , UnordSet } ;
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0014
/* FP:mod.rs-0028 */ use crate :: rustc_complete :: ErrorGuaranteed ;
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0015
/* FP:mod.rs-0030 */ use crate :: rustc_complete :: attrs :: StrippedCfgItem ;
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0016
/* FP:mod.rs-0032 */ use crate :: rustc_complete :: def :: { DefKind , DocLinkResMap } ;
/* FP:mod.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0017
/* FP:mod.rs-0034 */ use crate :: rustc_complete :: def_id :: { CrateNum , DefId , DefIdMap , LocalDefId , LocalDefIdMap , LocalDefIdSet , LocalModDefId , } ;
/* FP:mod.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0018
/* FP:mod.rs-0036 */ use crate :: rustc_complete :: lang_items :: { LangItem , LanguageItems } ;
/* FP:mod.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0019
/* FP:mod.rs-0038 */ use crate :: rustc_complete :: { Crate , ItemLocalId , ItemLocalMap , PreciseCapturingArgKind , TraitCandidate } ;
/* FP:mod.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0020
/* FP:mod.rs-0040 */ use crate :: rustc_index :: IndexVec ;
/* FP:mod.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0021
/* FP:mod.rs-0042 */ use crate :: rustc_lint_defs :: LintId ;
/* FP:mod.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0022
/* FP:mod.rs-0044 */ use rustc_macros :: rustc_queries ;
/* FP:mod.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0023
/* FP:mod.rs-0046 */ use rustc_query_system :: ich :: StableHashingContext ;
/* FP:mod.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0024
/* FP:mod.rs-0048 */ use rustc_query_system :: query :: { QueryCache , QueryMode , QueryStackDeferred , QueryState , try_get_cached , } ;
/* FP:mod.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0025
/* FP:mod.rs-0050 */ use crate :: rustc_complete :: Limits ;
/* FP:mod.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0026
/* FP:mod.rs-0052 */ use crate :: rustc_complete :: config :: { EntryFnType , OptLevel , OutputFilenames , SymbolManglingVersion } ;
/* FP:mod.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0027
/* FP:mod.rs-0054 */ use crate :: rustc_complete :: cstore :: { CrateDepKind , CrateSource , ExternCrate , ForeignModule , LinkagePreference , NativeLib , } ;
/* FP:mod.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0028
/* FP:mod.rs-0056 */ use crate :: rustc_complete :: lint :: LintExpectationId ;
/* FP:mod.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0029
/* FP:mod.rs-0058 */ use crate :: rustc_complete :: def_id :: LOCAL_CRATE ;
/* FP:mod.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0030
/* FP:mod.rs-0060 */ use crate :: rustc_complete :: source_map :: Spanned ;
/* FP:mod.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0031
/* FP:mod.rs-0062 */ use crate :: rustc_complete :: { DUMMY_SP , Span , Symbol } ;
/* FP:mod.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0032
/* FP:mod.rs-0064 */ use crate :: rustc_target :: spec :: { PanicStrategy , SanitizerSet } ;
/* FP:mod.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0033
/* FP:mod.rs-0066 */ use { rustc_abi as abi , rustc_ast as ast , rustc_hir as hir } ;
/* FP:mod.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0034
/* FP:mod.rs-0068 */ use crate :: infer :: canonical :: { self , Canonical } ;
/* FP:mod.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0035
/* FP:mod.rs-0070 */ use crate :: lint :: LintExpectation ;
/* FP:mod.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0036
/* FP:mod.rs-0072 */ use crate :: metadata :: ModChild ;
/* FP:mod.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0037
/* FP:mod.rs-0074 */ use crate :: middle :: codegen_fn_attrs :: CodegenFnAttrs ;
/* FP:mod.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0038
/* FP:mod.rs-0076 */ use crate :: middle :: debugger_visualizer :: DebuggerVisualizerFile ;
/* FP:mod.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0039
/* FP:mod.rs-0078 */ use crate :: middle :: exported_symbols :: { ExportedSymbol , SymbolExportInfo } ;
/* FP:mod.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0040
/* FP:mod.rs-0080 */ use crate :: middle :: lib_features :: LibFeatures ;
/* FP:mod.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0041
/* FP:mod.rs-0082 */ use crate :: middle :: privacy :: EffectiveVisibilities ;
/* FP:mod.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0042
/* FP:mod.rs-0084 */ use crate :: middle :: resolve_bound_vars :: { ObjectLifetimeDefault , ResolveBoundVars , ResolvedArg } ;
/* FP:mod.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0043
/* FP:mod.rs-0086 */ use crate :: middle :: stability :: DeprecationEntry ;
/* FP:mod.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0044
/* FP:mod.rs-0088 */ use crate :: mir :: interpret :: { EvalStaticInitializerRawResult , EvalToAllocationRawResult , EvalToConstValueResult , EvalToValTreeResult , GlobalId , LitToConstInput , } ;
/* FP:mod.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0045
/* FP:mod.rs-0090 */ use crate :: mir :: mono :: { CodegenUnit , CollectionMode , MonoItem , MonoItemPartitions } ;
/* FP:mod.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0046
/* FP:mod.rs-0092 */ use crate :: query :: erase :: { Erase , erase , restore } ;
/* FP:mod.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0047
/* FP:mod.rs-0094 */ use crate :: query :: plumbing :: { CyclePlaceholder , DynamicQuery , query_ensure , query_ensure_error_guaranteed , query_get_at , } ;
/* FP:mod.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0048
/* FP:mod.rs-0096 */ use crate :: traits :: query :: { CanonicalAliasGoal , CanonicalDropckOutlivesGoal , CanonicalImpliedOutlivesBoundsGoal , CanonicalPredicateGoal , CanonicalTyGoal , CanonicalTypeOpAscribeUserTypeGoal , CanonicalTypeOpNormalizeGoal , CanonicalTypeOpProvePredicateGoal , DropckConstraint , DropckOutlivesResult , MethodAutoderefStepsResult , NoSolution , NormalizationResult , OutlivesBound , } ;
/* FP:mod.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0049
/* FP:mod.rs-0098 */ use crate :: traits :: { CodegenObligationError , DynCompatibilityViolation , EvaluationResult , ImplSource , ObligationCause , OverflowError , WellFormedLoc , solve , specialization_graph , } ;
/* FP:mod.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0050
/* FP:mod.rs-0100 */ use crate :: ty :: fast_reject :: SimplifiedType ;
/* FP:mod.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0051
/* FP:mod.rs-0102 */ use crate :: ty :: layout :: ValidityRequirement ;
/* FP:mod.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0052
/* FP:mod.rs-0104 */ use crate :: ty :: print :: PrintTraitRefExt ;
/* FP:mod.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0053
/* FP:mod.rs-0106 */ use crate :: ty :: util :: AlwaysRequiresDrop ;
/* FP:mod.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0054
/* FP:mod.rs-0108 */ use crate :: ty :: { self , CrateInherentImpls , GenericArg , GenericArgsRef , PseudoCanonicalInput , SizedTraitKind , Ty , TyCtxt , TyCtxtFeed , } ;
/* FP:mod.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0055
/* FP:mod.rs-0110 */ use crate :: { dep_graph , mir , thir } ;
/* FP:mod.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_MOD_0056
/* FP:mod.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_MOD_0057
/* FP:mod.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_MOD_0058
/* FP:mod.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0059
/* FP:mod.rs-0118 */ pub use keys :: { AsLocalKey , Key , LocalCrate } ;
/* FP:mod.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_MOD_0060
/* FP:mod.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_MOD_0061
/* FP:mod.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_USE_0062
/* FP:mod.rs-0124 */ pub use plumbing :: { IntoQueryParam , TyCtxtAt , TyCtxtEnsureDone , TyCtxtEnsureOk } ;
/* FP:mod.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_MACRO_0063
/* FP:mod.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_MACRO_0064
/* FP:mod.rs-0128 */ rustc_with_all_queries ! { define_callbacks ! }
/* FP:mod.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_MACRO_0065
/* FP:mod.rs-0130 */ rustc_feedable_queries ! { define_feedable ! }
/* FP:mod.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_mod_FN_0066
/* FP:mod.rs-0132 */ fn describe_as_module (def_id : impl Into < LocalDefId > , tcx : TyCtxt < '_ >) -> String { let def_id = def_id . into () ; if def_id . is_top_level_module () { "top-level module" . to_string () } else { format ! ("module `{}`" , tcx . def_path_str (def_id)) } }