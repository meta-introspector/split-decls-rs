mkuse!{use std :: assert_matches :: assert_matches ;}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: num :: NonZero ;}
mkuse!{use std :: ptr :: NonNull ;}
mkuse!{use std :: { fmt , iter , str } ;}
mkuse!{pub use adt :: * ;}
mkuse!{pub use assoc :: * ;}
mkuse!{pub use generic_args :: { GenericArgKind , TermKind , * } ;}
mkuse!{pub use generics :: * ;}
mkuse!{pub use intrinsic :: IntrinsicDef ;}
mkuse!{use rustc_abi :: { Align , FieldIdx , Integer , IntegerType , ReprFlags , ReprOptions , VariantIdx } ;}
mkuse!{use rustc_ast :: node_id :: NodeMap ;}
mkuse!{pub use rustc_ast_ir :: { Movability , Mutability , try_visit } ;}
mkuse!{use rustc_data_structures :: fx :: { FxHashMap , FxHashSet , FxIndexMap , FxIndexSet } ;}
mkuse!{use rustc_data_structures :: intern :: Interned ;}
mkuse!{use rustc_data_structures :: stable_hasher :: { HashStable , StableHasher } ;}
mkuse!{use rustc_data_structures :: steal :: Steal ;}
mkuse!{use rustc_data_structures :: unord :: { UnordMap , UnordSet } ;}
mkuse!{use rustc_errors :: { Diag , ErrorGuaranteed , LintBuffer } ;}
mkuse!{use rustc_hir :: attrs :: { AttributeKind , StrippedCfgItem } ;}
mkuse!{use rustc_hir :: def :: { CtorKind , CtorOf , DefKind , DocLinkResMap , LifetimeRes , Res } ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , DefId , DefIdMap , LocalDefId , LocalDefIdMap } ;}
mkuse!{use rustc_hir :: definitions :: DisambiguatorState ;}
mkuse!{use rustc_hir :: { LangItem , attrs as attr , find_attr } ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_index :: bit_set :: BitMatrix ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable , TyDecodable , TyEncodable , TypeFoldable , TypeVisitable , extension , } ;}
mkuse!{use rustc_query_system :: ich :: StableHashingContext ;}
mkuse!{use rustc_serialize :: { Decodable , Encodable } ;}
mkuse!{pub use rustc_session :: lint :: RegisteredTools ;}
mkuse!{use rustc_span :: hygiene :: MacroKind ;}
mkuse!{use rustc_span :: { DUMMY_SP , ExpnId , ExpnKind , Ident , Span , Symbol , sym } ;}
mkuse!{pub use rustc_type_ir :: data_structures :: { DelayedMap , DelayedSet } ;}
mkuse!{pub use rustc_type_ir :: fast_reject :: DeepRejectCtxt ;}
mkuse!{# [allow (hidden_glob_reexports , rustc :: usage_of_type_ir_inherent , rustc :: non_glob_import_of_type_ir_inherent)] use rustc_type_ir :: inherent ;}
mkuse!{pub use rustc_type_ir :: relate :: VarianceDiagInfo ;}
mkuse!{pub use rustc_type_ir :: solve :: SizedTraitKind ;}
mkuse!{pub use rustc_type_ir :: * ;}
mkuse!{# [allow (hidden_glob_reexports , unused_imports)] use rustc_type_ir :: { InferCtxtLike , Interner } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{pub use vtable :: * ;}
mkuse!{use { rustc_ast as ast , rustc_hir as hir } ;}
mkuse!{pub use self :: closure :: { BorrowKind , CAPTURE_STRUCT_LOCAL , CaptureInfo , CapturedPlace , ClosureTypeInfo , MinCaptureInformationMap , MinCaptureList , RootVariableMinCaptureList , UpvarCapture , UpvarId , UpvarPath , analyze_coroutine_closure_captures , is_ancestor_or_same_capture , place_to_string_for_capture , } ;}
mkuse!{pub use self :: consts :: { AnonConstKind , AtomicOrdering , Const , ConstInt , ConstKind , ConstToValTreeResult , Expr , ExprKind , ScalarInt , UnevaluatedConst , ValTree , ValTreeKind , Value , } ;}
mkuse!{pub use self :: context :: { CtxtInterners , CurrentGcx , DeducedParamAttrs , Feed , FreeRegionInfo , GlobalCtxt , Lift , TyCtxt , TyCtxtFeed , tls , } ;}
mkuse!{pub use self :: fold :: * ;}
mkuse!{pub use self :: instance :: { Instance , InstanceKind , ReifyReason , UnusedGenericParams } ;}
mkuse!{pub use self :: list :: { List , ListWithCachedTypeInfo } ;}
mkuse!{pub use self :: opaque_types :: OpaqueTypeKey ;}
mkuse!{pub use self :: pattern :: { Pattern , PatternKind } ;}
mkuse!{pub use self :: predicate :: { AliasTerm , ArgOutlivesPredicate , Clause , ClauseKind , CoercePredicate , ExistentialPredicate , ExistentialPredicateStableCmpExt , ExistentialProjection , ExistentialTraitRef , HostEffectPredicate , NormalizesTo , OutlivesPredicate , PolyCoercePredicate , PolyExistentialPredicate , PolyExistentialProjection , PolyExistentialTraitRef , PolyProjectionPredicate , PolyRegionOutlivesPredicate , PolySubtypePredicate , PolyTraitPredicate , PolyTraitRef , PolyTypeOutlivesPredicate , Predicate , PredicateKind , ProjectionPredicate , RegionOutlivesPredicate , SubtypePredicate , TraitPredicate , TraitRef , TypeOutlivesPredicate , } ;}
mkuse!{pub use self :: region :: { BoundRegion , BoundRegionKind , EarlyParamRegion , LateParamRegion , LateParamRegionKind , Region , RegionKind , RegionVid , } ;}
mkuse!{pub use self :: rvalue_scopes :: RvalueScopes ;}
mkuse!{pub use self :: sty :: { AliasTy , Article , Binder , BoundTy , BoundTyKind , BoundVariableKind , CanonicalPolyFnSig , CoroutineArgsExt , EarlyBinder , FnSig , InlineConstArgs , InlineConstArgsParts , ParamConst , ParamTy , PolyFnSig , TyKind , TypeAndMut , TypingMode , UpvarArgs , } ;}
mkuse!{pub use self :: trait_def :: TraitDef ;}
mkuse!{pub use self :: typeck_results :: { CanonicalUserType , CanonicalUserTypeAnnotation , CanonicalUserTypeAnnotations , IsIdentity , Rust2024IncompatiblePatInfo , TypeckResults , UserType , UserTypeAnnotationIndex , UserTypeKind , } ;}
mkuse!{use crate :: error :: { OpaqueHiddenTypeMismatch , TypeMismatchReason } ;}
mkuse!{use crate :: metadata :: ModChild ;}
mkuse!{use crate :: middle :: privacy :: EffectiveVisibilities ;}
mkuse!{use crate :: mir :: { Body , CoroutineLayout , CoroutineSavedLocal , SourceInfo } ;}
mkuse!{use crate :: query :: { IntoQueryParam , Providers } ;}
mkuse!{use crate :: ty ;}
mkuse!{use crate :: ty :: codec :: { TyDecoder , TyEncoder } ;}
mkuse!{pub use crate :: ty :: diagnostics :: * ;}
mkuse!{use crate :: ty :: fast_reject :: SimplifiedType ;}
mkuse!{use crate :: ty :: layout :: LayoutError ;}
mkuse!{use crate :: ty :: util :: Discr ;}
mkuse!{use crate :: ty :: walk :: TypeWalker ;}
mkmod!{abstract_const, { 
                getname!(abstract_const);
                getsrc!(abstract_const);
                getpath!(abstract_const);
                get_deps!(abstract_const);
                get_crates!(abstract_const);
                mkinclude!(abstract_const);
                 
            }}
mkmod!{adjustment, { 
                getname!(adjustment);
                getsrc!(adjustment);
                getpath!(adjustment);
                get_deps!(adjustment);
                get_crates!(adjustment);
                mkinclude!(adjustment);
                 
            }}
mkmod!{cast, { 
                getname!(cast);
                getsrc!(cast);
                getpath!(cast);
                get_deps!(cast);
                get_crates!(cast);
                mkinclude!(cast);
                 
            }}
mkmod!{codec, { 
                getname!(codec);
                getsrc!(codec);
                getpath!(codec);
                get_deps!(codec);
                get_crates!(codec);
                mkinclude!(codec);
                 
            }}
mkmod!{error, { 
                getname!(error);
                getsrc!(error);
                getpath!(error);
                get_deps!(error);
                get_crates!(error);
                mkinclude!(error);
                 
            }}
mkmod!{fast_reject, { 
                getname!(fast_reject);
                getsrc!(fast_reject);
                getpath!(fast_reject);
                get_deps!(fast_reject);
                get_crates!(fast_reject);
                mkinclude!(fast_reject);
                 
            }}
mkmod!{inhabitedness, { 
                getname!(inhabitedness);
                getsrc!(inhabitedness);
                getpath!(inhabitedness);
                get_deps!(inhabitedness);
                get_crates!(inhabitedness);
                mkinclude!(inhabitedness);
                 
            }}
mkmod!{layout, { 
                getname!(layout);
                getsrc!(layout);
                getpath!(layout);
                get_deps!(layout);
                get_crates!(layout);
                mkinclude!(layout);
                 
            }}
mkmod!{normalize_erasing_regions, { 
                getname!(normalize_erasing_regions);
                getsrc!(normalize_erasing_regions);
                getpath!(normalize_erasing_regions);
                get_deps!(normalize_erasing_regions);
                get_crates!(normalize_erasing_regions);
                mkinclude!(normalize_erasing_regions);
                 
            }}
mkmod!{pattern, { 
                getname!(pattern);
                getsrc!(pattern);
                getpath!(pattern);
                get_deps!(pattern);
                get_crates!(pattern);
                mkinclude!(pattern);
                 
            }}
mkmod!{print, { 
                getname!(print);
                getsrc!(print);
                getpath!(print);
                get_deps!(print);
                get_crates!(print);
                mkinclude!(print);
                 
            }}
mkmod!{relate, { 
                getname!(relate);
                getsrc!(relate);
                getpath!(relate);
                get_deps!(relate);
                get_crates!(relate);
                mkinclude!(relate);
                 
            }}
mkmod!{significant_drop_order, { 
                getname!(significant_drop_order);
                getsrc!(significant_drop_order);
                getpath!(significant_drop_order);
                get_deps!(significant_drop_order);
                get_crates!(significant_drop_order);
                mkinclude!(significant_drop_order);
                 
            }}
mkmod!{trait_def, { 
                getname!(trait_def);
                getsrc!(trait_def);
                getpath!(trait_def);
                get_deps!(trait_def);
                get_crates!(trait_def);
                mkinclude!(trait_def);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkmod!{vtable, { 
                getname!(vtable);
                getsrc!(vtable);
                getpath!(vtable);
                get_deps!(vtable);
                get_crates!(vtable);
                mkinclude!(vtable);
                 
            }}
mkmod!{adt, { 
                getname!(adt);
                getsrc!(adt);
                getpath!(adt);
                get_deps!(adt);
                get_crates!(adt);
                mkinclude!(adt);
                 
            }}
mkmod!{assoc, { 
                getname!(assoc);
                getsrc!(assoc);
                getpath!(assoc);
                get_deps!(assoc);
                get_crates!(assoc);
                mkinclude!(assoc);
                 
            }}
mkmod!{closure, { 
                getname!(closure);
                getsrc!(closure);
                getpath!(closure);
                get_deps!(closure);
                get_crates!(closure);
                mkinclude!(closure);
                 
            }}
mkmod!{consts, { 
                getname!(consts);
                getsrc!(consts);
                getpath!(consts);
                get_deps!(consts);
                get_crates!(consts);
                mkinclude!(consts);
                 
            }}
mkmod!{context, { 
                getname!(context);
                getsrc!(context);
                getpath!(context);
                get_deps!(context);
                get_crates!(context);
                mkinclude!(context);
                 
            }}
mkmod!{diagnostics, { 
                getname!(diagnostics);
                getsrc!(diagnostics);
                getpath!(diagnostics);
                get_deps!(diagnostics);
                get_crates!(diagnostics);
                mkinclude!(diagnostics);
                 
            }}
mkmod!{elaborate_impl, { 
                getname!(elaborate_impl);
                getsrc!(elaborate_impl);
                getpath!(elaborate_impl);
                get_deps!(elaborate_impl);
                get_crates!(elaborate_impl);
                mkinclude!(elaborate_impl);
                 
            }}
mkmod!{erase_regions, { 
                getname!(erase_regions);
                getsrc!(erase_regions);
                getpath!(erase_regions);
                get_deps!(erase_regions);
                get_crates!(erase_regions);
                mkinclude!(erase_regions);
                 
            }}
mkmod!{fold, { 
                getname!(fold);
                getsrc!(fold);
                getpath!(fold);
                get_deps!(fold);
                get_crates!(fold);
                mkinclude!(fold);
                 
            }}
mkmod!{generic_args, { 
                getname!(generic_args);
                getsrc!(generic_args);
                getpath!(generic_args);
                get_deps!(generic_args);
                get_crates!(generic_args);
                mkinclude!(generic_args);
                 
            }}
mkmod!{generics, { 
                getname!(generics);
                getsrc!(generics);
                getpath!(generics);
                get_deps!(generics);
                get_crates!(generics);
                mkinclude!(generics);
                 
            }}
mkmod!{impls_ty, { 
                getname!(impls_ty);
                getsrc!(impls_ty);
                getpath!(impls_ty);
                get_deps!(impls_ty);
                get_crates!(impls_ty);
                mkinclude!(impls_ty);
                 
            }}
mkmod!{instance, { 
                getname!(instance);
                getsrc!(instance);
                getpath!(instance);
                get_deps!(instance);
                get_crates!(instance);
                mkinclude!(instance);
                 
            }}
mkmod!{intrinsic, { 
                getname!(intrinsic);
                getsrc!(intrinsic);
                getpath!(intrinsic);
                get_deps!(intrinsic);
                get_crates!(intrinsic);
                mkinclude!(intrinsic);
                 
            }}
mkmod!{list, { 
                getname!(list);
                getsrc!(list);
                getpath!(list);
                get_deps!(list);
                get_crates!(list);
                mkinclude!(list);
                 
            }}
mkmod!{opaque_types, { 
                getname!(opaque_types);
                getsrc!(opaque_types);
                getpath!(opaque_types);
                get_deps!(opaque_types);
                get_crates!(opaque_types);
                mkinclude!(opaque_types);
                 
            }}
mkmod!{predicate, { 
                getname!(predicate);
                getsrc!(predicate);
                getpath!(predicate);
                get_deps!(predicate);
                get_crates!(predicate);
                mkinclude!(predicate);
                 
            }}
mkmod!{region, { 
                getname!(region);
                getsrc!(region);
                getpath!(region);
                get_deps!(region);
                get_crates!(region);
                mkinclude!(region);
                 
            }}
mkmod!{rvalue_scopes, { 
                getname!(rvalue_scopes);
                getsrc!(rvalue_scopes);
                getpath!(rvalue_scopes);
                get_deps!(rvalue_scopes);
                get_crates!(rvalue_scopes);
                mkinclude!(rvalue_scopes);
                 
            }}
mkmod!{structural_impls, { 
                getname!(structural_impls);
                getsrc!(structural_impls);
                getpath!(structural_impls);
                get_deps!(structural_impls);
                get_crates!(structural_impls);
                mkinclude!(structural_impls);
                 
            }}
mkmod!{sty, { 
                getname!(sty);
                getsrc!(sty);
                getpath!(sty);
                get_deps!(sty);
                get_crates!(sty);
                mkinclude!(sty);
                 
            }}
mkmod!{typeck_results, { 
                getname!(typeck_results);
                getsrc!(typeck_results);
                getpath!(typeck_results);
                get_deps!(typeck_results);
                get_crates!(typeck_results);
                mkinclude!(typeck_results);
                 
            }}
mkmod!{visit, { 
                getname!(visit);
                getsrc!(visit);
                getpath!(visit);
                get_deps!(visit);
                get_crates!(visit);
                mkinclude!(visit);
                 
            }}
mkitem!{mkstruct!{# [derive (Debug , HashStable)] pub struct ResolverGlobalCtxt { pub visibilities_for_hashing : Vec < (LocalDefId , Visibility) > , # [doc = " Item with a given `LocalDefId` was defined during macro expansion with ID `ExpnId`."] pub expn_that_defined : UnordMap < LocalDefId , ExpnId > , pub effective_visibilities : EffectiveVisibilities , pub extern_crate_map : UnordMap < LocalDefId , CrateNum > , pub maybe_unused_trait_imports : FxIndexSet < LocalDefId > , pub module_children : LocalDefIdMap < Vec < ModChild > > , pub glob_map : FxIndexMap < LocalDefId , FxIndexSet < Symbol > > , pub main_def : Option < MainDefinition > , pub trait_impls : FxIndexMap < DefId , Vec < LocalDefId > > , # [doc = " A list of proc macro LocalDefIds, written out in the order in which"] # [doc = " they are declared in the static array generated by proc_macro_harness."] pub proc_macros : Vec < LocalDefId > , # [doc = " Mapping from ident span to path span for paths that don't exist as written, but that"] # [doc = " exist under `std`. For example, wrote `str::from_utf8` instead of `std::str::from_utf8`."] pub confused_type_with_std_module : FxIndexMap < Span , Span > , pub doc_link_resolutions : FxIndexMap < LocalDefId , DocLinkResMap > , pub doc_link_traits_in_scope : FxIndexMap < LocalDefId , Vec < DefId > > , pub all_macro_rules : UnordSet < Symbol > , pub stripped_cfg_items : Vec < StrippedCfgItem > , }}}
mkitem!{mkstruct!{# [doc = " Resolutions that should only be used for lowering."] # [doc = " This struct is meant to be consumed by lowering."] # [derive (Debug)] pub struct ResolverAstLowering { pub legacy_const_generic_args : FxHashMap < DefId , Option < Vec < usize > > > , # [doc = " Resolutions for nodes that have a single resolution."] pub partial_res_map : NodeMap < hir :: def :: PartialRes > , # [doc = " Resolutions for import nodes, which have multiple resolutions in different namespaces."] pub import_res_map : NodeMap < hir :: def :: PerNS < Option < Res < ast :: NodeId > > > > , # [doc = " Resolutions for labels (node IDs of their corresponding blocks or loops)."] pub label_res_map : NodeMap < ast :: NodeId > , # [doc = " Resolutions for lifetimes."] pub lifetimes_res_map : NodeMap < LifetimeRes > , # [doc = " Lifetime parameters that lowering will have to introduce."] pub extra_lifetime_params_map : NodeMap < Vec < (Ident , ast :: NodeId , LifetimeRes) > > , pub next_node_id : ast :: NodeId , pub node_id_to_def_id : NodeMap < LocalDefId > , pub disambiguator : DisambiguatorState , pub trait_map : NodeMap < Vec < hir :: TraitCandidate > > , # [doc = " List functions and methods for which lifetime elision was successful."] pub lifetime_elision_allowed : FxHashSet < ast :: NodeId > , # [doc = " Lints that were emitted by the resolver and early lints."] pub lint_buffer : Steal < LintBuffer > , # [doc = " Information about functions signatures for delegation items expansion"] pub delegation_fn_sigs : LocalDefIdMap < DelegationFnSig > , }}}
mkitem!{mkstruct!{# [derive (Debug)] pub struct DelegationFnSig { pub header : ast :: FnHeader , pub param_count : usize , pub has_self : bool , pub c_variadic : bool , pub target_feature : bool , }}}
mkitem!{mkstruct!{# [derive (Clone , Copy , Debug , HashStable)] pub struct MainDefinition { pub res : Res < ast :: NodeId > , pub is_import : bool , pub span : Span , }}}
mkitem!{mkimpl!{impl MainDefinition { pub fn opt_fn_def_id (self) -> Option < DefId > { if let Res :: Def (DefKind :: Fn , def_id) = self . res { Some (def_id) } else { None } } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , TyEncodable , TyDecodable , HashStable)] pub struct ImplTraitHeader < 'tcx > { pub trait_ref : ty :: EarlyBinder < 'tcx , ty :: TraitRef < 'tcx > > , pub polarity : ImplPolarity , pub safety : hir :: Safety , pub constness : hir :: Constness , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Eq , Debug , TypeFoldable , TypeVisitable)] pub enum ImplSubject < 'tcx > { Trait (TraitRef < 'tcx >) , Inherent (Ty < 'tcx >) , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Eq , Hash , TyEncodable , TyDecodable , HashStable , Debug)] # [derive (TypeFoldable , TypeVisitable)] pub enum Asyncness { Yes , No , }}}
mkitem!{mkimpl!{impl Asyncness { pub fn is_async (self) -> bool { matches ! (self , Asyncness :: Yes) } }}}
mkitem!{mkenum!{# [derive (Clone , Debug , PartialEq , Eq , Copy , Hash , Encodable , Decodable , HashStable)] pub enum Visibility < Id = LocalDefId > { # [doc = " Visible everywhere (including in other crates)."] Public , # [doc = " Visible only in the given crate-local module."] Restricted (Id) , }}}
mkitem!{mkimpl!{impl Visibility { pub fn to_string (self , def_id : LocalDefId , tcx : TyCtxt < '_ >) -> String { match self { ty :: Visibility :: Restricted (restricted_id) => { if restricted_id . is_top_level_module () { "pub(crate)" . to_string () } else if restricted_id == tcx . parent_module_from_def_id (def_id) . to_local_def_id () { "pub(self)" . to_string () } else { format ! ("pub(in crate{})" , tcx . def_path (restricted_id . to_def_id ()) . to_string_no_crate_verbose ()) } } ty :: Visibility :: Public => "pub" . to_string () , } } }}}
mkitem!{mkstruct!{# [derive (Clone , Debug , PartialEq , Eq , Copy , Hash , TyEncodable , TyDecodable , HashStable)] # [derive (TypeFoldable , TypeVisitable)] pub struct ClosureSizeProfileData < 'tcx > { # [doc = " Tuple containing the types of closure captures before the feature `capture_disjoint_fields`"] pub before_feature_tys : Ty < 'tcx > , # [doc = " Tuple containing the types of closure captures after the feature `capture_disjoint_fields`"] pub after_feature_tys : Ty < 'tcx > , }}}
mkitem!{mkimpl!{impl TyCtxt < '_ > { # [inline] pub fn opt_parent (self , id : DefId) -> Option < DefId > { self . def_key (id) . parent . map (| index | DefId { index , .. id }) } # [inline] # [track_caller] pub fn parent (self , id : DefId) -> DefId { match self . opt_parent (id) { Some (id) => id , None => bug ! ("{id:?} doesn't have a parent") , } } # [inline] # [track_caller] pub fn opt_local_parent (self , id : LocalDefId) -> Option < LocalDefId > { self . opt_parent (id . to_def_id ()) . map (DefId :: expect_local) } # [inline] # [track_caller] pub fn local_parent (self , id : impl Into < LocalDefId >) -> LocalDefId { self . parent (id . into () . to_def_id ()) . expect_local () } pub fn is_descendant_of (self , mut descendant : DefId , ancestor : DefId) -> bool { if descendant . krate != ancestor . krate { return false ; } while descendant != ancestor { match self . opt_parent (descendant) { Some (parent) => descendant = parent , None => return false , } } true } }}}
mkitem!{mkimpl!{impl < Id > Visibility < Id > { pub fn is_public (self) -> bool { matches ! (self , Visibility :: Public) } pub fn map_id < OutId > (self , f : impl FnOnce (Id) -> OutId) -> Visibility < OutId > { match self { Visibility :: Public => Visibility :: Public , Visibility :: Restricted (id) => Visibility :: Restricted (f (id)) , } } }}}
mkitem!{mkimpl!{impl < Id : Into < DefId > > Visibility < Id > { pub fn to_def_id (self) -> Visibility < DefId > { self . map_id (Into :: into) } # [doc = " Returns `true` if an item with this visibility is accessible from the given module."] pub fn is_accessible_from (self , module : impl Into < DefId > , tcx : TyCtxt < '_ >) -> bool { match self { Visibility :: Public => true , Visibility :: Restricted (id) => tcx . is_descendant_of (module . into () , id . into ()) , } } # [doc = " Returns `true` if this visibility is at least as accessible as the given visibility"] pub fn is_at_least (self , vis : Visibility < impl Into < DefId > > , tcx : TyCtxt < '_ >) -> bool { match vis { Visibility :: Public => self . is_public () , Visibility :: Restricted (id) => self . is_accessible_from (id , tcx) , } } }}}
mkitem!{mkimpl!{impl Visibility < DefId > { pub fn expect_local (self) -> Visibility { self . map_id (| id | id . expect_local ()) } # [doc = " Returns `true` if this item is visible anywhere in the local crate."] pub fn is_visible_locally (self) -> bool { match self { Visibility :: Public => true , Visibility :: Restricted (def_id) => def_id . is_local () , } } }}}
mkitem!{mkstruct!{# [doc = " The crate variances map is computed during typeck and contains the"] # [doc = " variance of every item in the local crate. You should not use it"] # [doc = " directly, because to do so will make your pass dependent on the"] # [doc = " HIR of every item in the local crate. Instead, use"] # [doc = " `tcx.variances_of()` to get the variance for a *particular*"] # [doc = " item."] # [derive (HashStable , Debug)] pub struct CrateVariancesMap < 'tcx > { # [doc = " For each item with generics, maps to a vector of the variance"] # [doc = " of its generics. If an item has no generics, it will have no"] # [doc = " entry."] pub variances : DefIdMap < & 'tcx [ty :: Variance] > , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , PartialEq , Eq , Hash)] pub struct CReaderCacheKey { pub cnum : Option < CrateNum > , pub pos : usize , }}}
mkitem!{mkstruct!{# [doc = " Use this rather than `TyKind`, whenever possible."] # [derive (Copy , Clone , PartialEq , Eq , Hash , HashStable)] # [rustc_diagnostic_item = "Ty"] # [rustc_pass_by_value] pub struct Ty < 'tcx > (Interned < 'tcx , WithCachedTypeInfo < TyKind < 'tcx > > >) ;}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: IntoKind for Ty < 'tcx > { type Kind = TyKind < 'tcx > ; fn kind (self) -> TyKind < 'tcx > { * self . kind () } }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: Flags for Ty < 'tcx > { fn flags (& self) -> TypeFlags { self . 0 . flags } fn outer_exclusive_binder (& self) -> DebruijnIndex { self . 0 . outer_exclusive_binder } }}}
mkitem!{mkstruct!{# [doc = " The crate outlives map is computed during typeck and contains the"] # [doc = " outlives of every item in the local crate. You should not use it"] # [doc = " directly, because to do so will make your pass dependent on the"] # [doc = " HIR of every item in the local crate. Instead, use"] # [doc = " `tcx.inferred_outlives_of()` to get the outlives for a *particular*"] # [doc = " item."] # [derive (HashStable , Debug)] pub struct CratePredicatesMap < 'tcx > { # [doc = " For each struct with outlive bounds, maps to a vector of the"] # [doc = " predicate of its outlive bounds. If an item has no outlives"] # [doc = " bounds, it will have no entry."] pub predicates : DefIdMap < & 'tcx [(Clause < 'tcx > , Span)] > , }}}
mkitem!{mkstruct!{# [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Term < 'tcx > { ptr : NonNull < () > , marker : PhantomData < (Ty < 'tcx > , Const < 'tcx >) > , }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: Term < TyCtxt < 'tcx > > for Term < 'tcx > { }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: IntoKind for Term < 'tcx > { type Kind = TermKind < 'tcx > ; fn kind (self) -> Self :: Kind { self . kind () } }}}
mkitem!{mkimpl!{unsafe impl < 'tcx > rustc_data_structures :: sync :: DynSend for Term < 'tcx > where & 'tcx (Ty < 'tcx > , Const < 'tcx >) : rustc_data_structures :: sync :: DynSend { }}}
mkitem!{mkimpl!{unsafe impl < 'tcx > rustc_data_structures :: sync :: DynSync for Term < 'tcx > where & 'tcx (Ty < 'tcx > , Const < 'tcx >) : rustc_data_structures :: sync :: DynSync { }}}
mkitem!{mkimpl!{unsafe impl < 'tcx > Send for Term < 'tcx > where & 'tcx (Ty < 'tcx > , Const < 'tcx >) : Send { }}}
mkitem!{mkimpl!{unsafe impl < 'tcx > Sync for Term < 'tcx > where & 'tcx (Ty < 'tcx > , Const < 'tcx >) : Sync { }}}
mkitem!{mkimpl!{impl Debug for Term < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . kind () { TermKind :: Ty (ty) => write ! (f , "Term::Ty({ty:?})") , TermKind :: Const (ct) => write ! (f , "Term::Const({ct:?})") , } } }}}
mkitem!{mkimpl!{impl < 'tcx > From < Ty < 'tcx > > for Term < 'tcx > { fn from (ty : Ty < 'tcx >) -> Self { TermKind :: Ty (ty) . pack () } }}}
mkitem!{mkimpl!{impl < 'tcx > From < Const < 'tcx > > for Term < 'tcx > { fn from (c : Const < 'tcx >) -> Self { TermKind :: Const (c) . pack () } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > HashStable < StableHashingContext < 'a > > for Term < 'tcx > { fn hash_stable (& self , hcx : & mut StableHashingContext < 'a > , hasher : & mut StableHasher) { self . kind () . hash_stable (hcx , hasher) ; } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeFoldable < TyCtxt < 'tcx > > for Term < 'tcx > { fn try_fold_with < F : FallibleTypeFolder < TyCtxt < 'tcx > > > (self , folder : & mut F ,) -> Result < Self , F :: Error > { match self . kind () { ty :: TermKind :: Ty (ty) => ty . try_fold_with (folder) . map (Into :: into) , ty :: TermKind :: Const (ct) => ct . try_fold_with (folder) . map (Into :: into) , } } fn fold_with < F : TypeFolder < TyCtxt < 'tcx > > > (self , folder : & mut F) -> Self { match self . kind () { ty :: TermKind :: Ty (ty) => ty . fold_with (folder) . into () , ty :: TermKind :: Const (ct) => ct . fold_with (folder) . into () , } } }}}
mkitem!{mkimpl!{impl < 'tcx > TypeVisitable < TyCtxt < 'tcx > > for Term < 'tcx > { fn visit_with < V : TypeVisitor < TyCtxt < 'tcx > > > (& self , visitor : & mut V) -> V :: Result { match self . kind () { ty :: TermKind :: Ty (ty) => ty . visit_with (visitor) , ty :: TermKind :: Const (ct) => ct . visit_with (visitor) , } } }}}
mkitem!{mkimpl!{impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for Term < 'tcx > { fn encode (& self , e : & mut E) { self . kind () . encode (e) } }}}
mkitem!{mkimpl!{impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for Term < 'tcx > { fn decode (d : & mut D) -> Self { let res : TermKind < 'tcx > = Decodable :: decode (d) ; res . pack () } }}}
mkitem!{mkimpl!{impl < 'tcx > Term < 'tcx > { # [inline] pub fn kind (self) -> TermKind < 'tcx > { let ptr = unsafe { self . ptr . map_addr (| addr | NonZero :: new_unchecked (addr . get () & ! TAG_MASK)) } ; unsafe { match self . ptr . addr () . get () & TAG_MASK { TYPE_TAG => TermKind :: Ty (Ty (Interned :: new_unchecked (ptr . cast :: < WithCachedTypeInfo < ty :: TyKind < 'tcx > > > () . as_ref () ,))) , CONST_TAG => TermKind :: Const (ty :: Const (Interned :: new_unchecked (ptr . cast :: < WithCachedTypeInfo < ty :: ConstKind < 'tcx > > > () . as_ref () ,))) , _ => core :: intrinsics :: unreachable () , } } } pub fn as_type (& self) -> Option < Ty < 'tcx > > { if let TermKind :: Ty (ty) = self . kind () { Some (ty) } else { None } } pub fn expect_type (& self) -> Ty < 'tcx > { self . as_type () . expect ("expected a type, but found a const") } pub fn as_const (& self) -> Option < Const < 'tcx > > { if let TermKind :: Const (c) = self . kind () { Some (c) } else { None } } pub fn expect_const (& self) -> Const < 'tcx > { self . as_const () . expect ("expected a const, but found a type") } pub fn into_arg (self) -> GenericArg < 'tcx > { match self . kind () { TermKind :: Ty (ty) => ty . into () , TermKind :: Const (c) => c . into () , } } pub fn to_alias_term (self) -> Option < AliasTerm < 'tcx > > { match self . kind () { TermKind :: Ty (ty) => match * ty . kind () { ty :: Alias (_kind , alias_ty) => Some (alias_ty . into ()) , _ => None , } , TermKind :: Const (ct) => match ct . kind () { ConstKind :: Unevaluated (uv) => Some (uv . into ()) , _ => None , } , } } pub fn is_infer (& self) -> bool { match self . kind () { TermKind :: Ty (ty) => ty . is_ty_var () , TermKind :: Const (ct) => ct . is_ct_infer () , } } pub fn is_trivially_wf (& self , tcx : TyCtxt < 'tcx >) -> bool { match self . kind () { TermKind :: Ty (ty) => ty . is_trivially_wf (tcx) , TermKind :: Const (ct) => ct . is_trivially_wf () , } } # [doc = " Iterator that walks `self` and any types reachable from"] # [doc = " `self`, in depth-first order. Note that just walks the types"] # [doc = " that appear in `self`, it does not descend into the fields of"] # [doc = " structs or variants. For example:"] # [doc = ""] # [doc = " ```text"] # [doc = " isize => { isize }"] # [doc = " Foo<Bar<isize>> => { Foo<Bar<isize>>, Bar<isize>, isize }"] # [doc = " [isize] => { [isize], isize }"] # [doc = " ```"] pub fn walk (self) -> TypeWalker < TyCtxt < 'tcx > > { TypeWalker :: new (self . into ()) } }}}
mkitem!{const TAG_MASK : usize = 0b11 ;}
mkitem!{const TYPE_TAG : usize = 0b00 ;}
mkitem!{const CONST_TAG : usize = 0b01 ;}
mkitem!{mkimpl!{# [extension (pub trait TermKindPackExt <'tcx >)] impl < 'tcx > TermKind < 'tcx > { # [inline] fn pack (self) -> Term < 'tcx > { let (tag , ptr) = match self { TermKind :: Ty (ty) => { assert_eq ! (align_of_val (&* ty . 0.0) & TAG_MASK , 0) ; (TYPE_TAG , NonNull :: from (ty . 0 . 0) . cast ()) } TermKind :: Const (ct) => { assert_eq ! (align_of_val (&* ct . 0.0) & TAG_MASK , 0) ; (CONST_TAG , NonNull :: from (ct . 0 . 0) . cast ()) } } ; Term { ptr : ptr . map_addr (| addr | addr | tag) , marker : PhantomData } } }}}
mkitem!{mkstruct!{# [doc = " Represents the bounds declared on a particular set of type"] # [doc = " parameters. Should eventually be generalized into a flag list of"] # [doc = " where-clauses. You can obtain an `InstantiatedPredicates` list from a"] # [doc = " `GenericPredicates` by using the `instantiate` method. Note that this method"] # [doc = " reflects an important semantic invariant of `InstantiatedPredicates`: while"] # [doc = " the `GenericPredicates` are expressed in terms of the bound type"] # [doc = " parameters of the impl/trait/whatever, an `InstantiatedPredicates` instance"] # [doc = " represented a set of bounds for some particular instantiation,"] # [doc = " meaning that the generic parameters have been instantiated with"] # [doc = " their values."] # [doc = ""] # [doc = " Example:"] # [doc = " ```ignore (illustrative)"] # [doc = " struct Foo<T, U: Bar<T>> { ... }"] # [doc = " ```"] # [doc = " Here, the `GenericPredicates` for `Foo` would contain a list of bounds like"] # [doc = " `[[], [U:Bar<T>]]`. Now if there were some particular reference"] # [doc = " like `Foo<isize,usize>`, then the `InstantiatedPredicates` would be `[[],"] # [doc = " [usize:Bar<isize>]]`."] # [derive (Clone , Debug , TypeFoldable , TypeVisitable)] pub struct InstantiatedPredicates < 'tcx > { pub predicates : Vec < Clause < 'tcx > > , pub spans : Vec < Span > , }}}
mkitem!{mkimpl!{impl < 'tcx > InstantiatedPredicates < 'tcx > { pub fn empty () -> InstantiatedPredicates < 'tcx > { InstantiatedPredicates { predicates : vec ! [] , spans : vec ! [] } } pub fn is_empty (& self) -> bool { self . predicates . is_empty () } pub fn iter (& self) -> < & Self as IntoIterator > :: IntoIter { self . into_iter () } }}}
mkitem!{mkimpl!{impl < 'tcx > IntoIterator for InstantiatedPredicates < 'tcx > { type Item = (Clause < 'tcx > , Span) ; type IntoIter = std :: iter :: Zip < std :: vec :: IntoIter < Clause < 'tcx > > , std :: vec :: IntoIter < Span > > ; fn into_iter (self) -> Self :: IntoIter { debug_assert_eq ! (self . predicates . len () , self . spans . len ()) ; std :: iter :: zip (self . predicates , self . spans) } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > IntoIterator for & 'a InstantiatedPredicates < 'tcx > { type Item = (Clause < 'tcx > , Span) ; type IntoIter = std :: iter :: Zip < std :: iter :: Copied < std :: slice :: Iter < 'a , Clause < 'tcx > > > , std :: iter :: Copied < std :: slice :: Iter < 'a , Span > > , > ; fn into_iter (self) -> Self :: IntoIter { debug_assert_eq ! (self . predicates . len () , self . spans . len ()) ; std :: iter :: zip (self . predicates . iter () . copied () , self . spans . iter () . copied ()) } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , TypeFoldable , TypeVisitable , HashStable , TyEncodable , TyDecodable)] pub struct OpaqueHiddenType < 'tcx > { # [doc = " The span of this particular definition of the opaque type. So"] # [doc = " for example:"] # [doc = ""] # [doc = " ```ignore (incomplete snippet)"] # [doc = " type Foo = impl Baz;"] # [doc = " fn bar() -> Foo {"] # [doc = " //          ^^^ This is the span we are looking for!"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " In cases where the fn returns `(impl Trait, impl Trait)` or"] # [doc = " other such combinations, the result is currently"] # [doc = " over-approximated, but better than nothing."] pub span : Span , # [doc = " The type variable that represents the value of the opaque type"] # [doc = " that we require. In other words, after we compile this function,"] # [doc = " we will be created a constraint like:"] # [doc = " ```ignore (pseudo-rust)"] # [doc = " Foo<'a, T> = ?C"] # [doc = " ```"] # [doc = " where `?C` is the value of this type variable. =) It may"] # [doc = " naturally refer to the type and lifetime parameters in scope"] # [doc = " in this function, though ultimately it should only reference"] # [doc = " those that are arguments to `Foo` in the constraint above. (In"] # [doc = " other words, `?C` should not include `'b`, even though it's a"] # [doc = " lifetime parameter on `foo`.)"] pub ty : Ty < 'tcx > , }}}
mkitem!{mkenum!{# [doc = " Whether we're currently in HIR typeck or MIR borrowck."] # [derive (Debug , Clone , Copy)] pub enum DefiningScopeKind { # [doc = " During writeback in typeck, we don't care about regions and simply"] # [doc = " erase them. This means we also don't check whether regions are"] # [doc = " universal in the opaque type key. This will only be checked in"] # [doc = " MIR borrowck."] HirTypeck , MirBorrowck , }}}
mkitem!{mkimpl!{impl < 'tcx > OpaqueHiddenType < 'tcx > { pub fn new_error (tcx : TyCtxt < 'tcx > , guar : ErrorGuaranteed) -> OpaqueHiddenType < 'tcx > { OpaqueHiddenType { span : DUMMY_SP , ty : Ty :: new_error (tcx , guar) } } pub fn build_mismatch_error (& self , other : & Self , tcx : TyCtxt < 'tcx > ,) -> Result < Diag < 'tcx > , ErrorGuaranteed > { (self . ty , other . ty) . error_reported () ? ; let sub_diag = if self . span == other . span { TypeMismatchReason :: ConflictType { span : self . span } } else { TypeMismatchReason :: PreviousUse { span : self . span } } ; Ok (tcx . dcx () . create_err (OpaqueHiddenTypeMismatch { self_ty : self . ty , other_ty : other . ty , other_span : other . span , sub : sub_diag , })) } # [instrument (level = "debug" , skip (tcx) , ret)] pub fn remap_generic_params_to_declaration_params (self , opaque_type_key : OpaqueTypeKey < 'tcx > , tcx : TyCtxt < 'tcx > , defining_scope_kind : DefiningScopeKind ,) -> Self { let OpaqueTypeKey { def_id , args } = opaque_type_key ; let id_args = GenericArgs :: identity_for_item (tcx , def_id) ; debug ! (? id_args) ; let map = args . iter () . zip (id_args) . collect () ; debug ! ("map = {:#?}" , map) ; let this = match defining_scope_kind { DefiningScopeKind :: HirTypeck => fold_regions (tcx , self , | _ , _ | tcx . lifetimes . re_erased) , DefiningScopeKind :: MirBorrowck => self , } ; let result = this . fold_with (& mut opaque_types :: ReverseMapper :: new (tcx , map , self . span)) ; if cfg ! (debug_assertions) && matches ! (defining_scope_kind , DefiningScopeKind :: HirTypeck) { assert_eq ! (result . ty , fold_regions (tcx , result . ty , | _ , _ | tcx . lifetimes . re_erased)) ; } result } }}}
mkitem!{mkstruct!{# [doc = " The \"placeholder index\" fully defines a placeholder region, type, or const. Placeholders are"] # [doc = " identified by both a universe, as well as a name residing within that universe. Distinct bound"] # [doc = " regions/types/consts within the same universe simply have an unknown relationship to one"] # [doc = " another."] # [derive (Copy , Clone , PartialEq , Eq , Hash , PartialOrd , Ord)] # [derive (HashStable , TyEncodable , TyDecodable)] pub struct Placeholder < T > { pub universe : UniverseIndex , pub bound : T , }}}
mkitem!{pub type PlaceholderRegion = Placeholder < BoundRegion > ;}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: PlaceholderLike < TyCtxt < 'tcx > > for PlaceholderRegion { type Bound = BoundRegion ; fn universe (self) -> UniverseIndex { self . universe } fn var (self) -> BoundVar { self . bound . var } fn with_updated_universe (self , ui : UniverseIndex) -> Self { Placeholder { universe : ui , .. self } } fn new (ui : UniverseIndex , bound : BoundRegion) -> Self { Placeholder { universe : ui , bound } } fn new_anon (ui : UniverseIndex , var : BoundVar) -> Self { Placeholder { universe : ui , bound : BoundRegion { var , kind : BoundRegionKind :: Anon } } } }}}
mkitem!{pub type PlaceholderType = Placeholder < BoundTy > ;}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: PlaceholderLike < TyCtxt < 'tcx > > for PlaceholderType { type Bound = BoundTy ; fn universe (self) -> UniverseIndex { self . universe } fn var (self) -> BoundVar { self . bound . var } fn with_updated_universe (self , ui : UniverseIndex) -> Self { Placeholder { universe : ui , .. self } } fn new (ui : UniverseIndex , bound : BoundTy) -> Self { Placeholder { universe : ui , bound } } fn new_anon (ui : UniverseIndex , var : BoundVar) -> Self { Placeholder { universe : ui , bound : BoundTy { var , kind : BoundTyKind :: Anon } } } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , HashStable)] # [derive (TyEncodable , TyDecodable)] pub struct BoundConst { pub var : BoundVar , }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: BoundVarLike < TyCtxt < 'tcx > > for BoundConst { fn var (self) -> BoundVar { self . var } fn assert_eq (self , var : ty :: BoundVariableKind) { var . expect_const () } }}}
mkitem!{pub type PlaceholderConst = Placeholder < BoundConst > ;}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: PlaceholderLike < TyCtxt < 'tcx > > for PlaceholderConst { type Bound = BoundConst ; fn universe (self) -> UniverseIndex { self . universe } fn var (self) -> BoundVar { self . bound . var } fn with_updated_universe (self , ui : UniverseIndex) -> Self { Placeholder { universe : ui , .. self } } fn new (ui : UniverseIndex , bound : BoundConst) -> Self { Placeholder { universe : ui , bound } } fn new_anon (ui : UniverseIndex , var : BoundVar) -> Self { Placeholder { universe : ui , bound : BoundConst { var } } } }}}
mkitem!{pub type Clauses < 'tcx > = & 'tcx ListWithCachedTypeInfo < Clause < 'tcx > > ;}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: Flags for Clauses < 'tcx > { fn flags (& self) -> TypeFlags { (* * self) . flags () } fn outer_exclusive_binder (& self) -> DebruijnIndex { (* * self) . outer_exclusive_binder () } }}}
mkitem!{mkstruct!{# [doc = " When interacting with the type system we must provide information about the"] # [doc = " environment. `ParamEnv` is the type that represents this information. See the"] # [doc = " [dev guide chapter][param_env_guide] for more information."] # [doc = ""] # [doc = " [param_env_guide]: https://rustc-dev-guide.rust-lang.org/typing_parameter_envs.html"] # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] # [derive (HashStable , TypeVisitable , TypeFoldable)] pub struct ParamEnv < 'tcx > { # [doc = " Caller bounds are `Obligation`s that the caller must satisfy. This is"] # [doc = " basically the set of bounds on the in-scope type parameters, translated"] # [doc = " into `Obligation`s, and elaborated and normalized."] # [doc = ""] # [doc = " Use the `caller_bounds()` method to access."] caller_bounds : Clauses < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > rustc_type_ir :: inherent :: ParamEnv < TyCtxt < 'tcx > > for ParamEnv < 'tcx > { fn caller_bounds (self) -> impl inherent :: SliceLike < Item = ty :: Clause < 'tcx > > { self . caller_bounds () } }}}
mkitem!{mkimpl!{impl < 'tcx > ParamEnv < 'tcx > { # [doc = " Construct a trait environment suitable for contexts where there are"] # [doc = " no where-clauses in scope. In the majority of cases it is incorrect"] # [doc = " to use an empty environment. See the [dev guide section][param_env_guide]"] # [doc = " for information on what a `ParamEnv` is and how to acquire one."] # [doc = ""] # [doc = " [param_env_guide]: https://rustc-dev-guide.rust-lang.org/typing_parameter_envs.html"] # [inline] pub fn empty () -> Self { Self :: new (ListWithCachedTypeInfo :: empty ()) } # [inline] pub fn caller_bounds (self) -> Clauses < 'tcx > { self . caller_bounds } # [doc = " Construct a trait environment with the given set of predicates."] # [inline] pub fn new (caller_bounds : Clauses < 'tcx >) -> Self { ParamEnv { caller_bounds } } # [doc = " Creates a pair of param-env and value for use in queries."] pub fn and < T : TypeVisitable < TyCtxt < 'tcx > > > (self , value : T) -> ParamEnvAnd < 'tcx , T > { ParamEnvAnd { param_env : self , value } } }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , TypeFoldable , TypeVisitable)] # [derive (HashStable)] pub struct ParamEnvAnd < 'tcx , T > { pub param_env : ParamEnv < 'tcx > , pub value : T , }}}
mkitem!{mkstruct!{# [doc = " The environment in which to do trait solving."] # [doc = ""] # [doc = " Most of the time you only need to care about the `ParamEnv`"] # [doc = " as the `TypingMode` is simply stored in the `InferCtxt`."] # [doc = ""] # [doc = " However, there are some places which rely on trait solving"] # [doc = " without using an `InferCtxt` themselves. For these to be"] # [doc = " able to use the trait system they have to be able to initialize"] # [doc = " such an `InferCtxt` with the right `typing_mode`, so they need"] # [doc = " to track both."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash , HashStable)] # [derive (TypeVisitable , TypeFoldable)] pub struct TypingEnv < 'tcx > { # [type_foldable (identity)] # [type_visitable (ignore)] pub typing_mode : TypingMode < 'tcx > , pub param_env : ParamEnv < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > TypingEnv < 'tcx > { # [doc = " Create a typing environment with no where-clauses in scope"] # [doc = " where all opaque types and default associated items are revealed."] # [doc = ""] # [doc = " This is only suitable for monomorphized, post-typeck environments."] # [doc = " Do not use this for MIR optimizations, as even though they also"] # [doc = " use `TypingMode::PostAnalysis`, they may still have where-clauses"] # [doc = " in scope."] pub fn fully_monomorphized () -> TypingEnv < 'tcx > { TypingEnv { typing_mode : TypingMode :: PostAnalysis , param_env : ParamEnv :: empty () } } # [doc = " Create a typing environment for use during analysis outside of a body."] # [doc = ""] # [doc = " Using a typing environment inside of bodies is not supported as the body"] # [doc = " may define opaque types. In this case the used functions have to be"] # [doc = " converted to use proper canonical inputs instead."] pub fn non_body_analysis (tcx : TyCtxt < 'tcx > , def_id : impl IntoQueryParam < DefId > ,) -> TypingEnv < 'tcx > { TypingEnv { typing_mode : TypingMode :: non_body_analysis () , param_env : tcx . param_env (def_id) } } pub fn post_analysis (tcx : TyCtxt < 'tcx > , def_id : impl IntoQueryParam < DefId >) -> TypingEnv < 'tcx > { tcx . typing_env_normalized_for_post_analysis (def_id) } # [doc = " Modify the `typing_mode` to `PostAnalysis` and eagerly reveal all"] # [doc = " opaque types in the `param_env`."] pub fn with_post_analysis_normalized (self , tcx : TyCtxt < 'tcx >) -> TypingEnv < 'tcx > { let TypingEnv { typing_mode , param_env } = self ; if let TypingMode :: PostAnalysis = typing_mode { return self ; } let param_env = if tcx . next_trait_solver_globally () { param_env } else { ParamEnv :: new (tcx . reveal_opaque_types_in_bounds (param_env . caller_bounds ())) } ; TypingEnv { typing_mode : TypingMode :: PostAnalysis , param_env } } # [doc = " Combine this typing environment with the given `value` to be used by"] # [doc = " not (yet) canonicalized queries. This only works if the value does not"] # [doc = " contain anything local to some `InferCtxt`, i.e. inference variables or"] # [doc = " placeholders."] pub fn as_query_input < T > (self , value : T) -> PseudoCanonicalInput < 'tcx , T > where T : TypeVisitable < TyCtxt < 'tcx > > , { PseudoCanonicalInput { typing_env : self , value } } }}}
mkitem!{mkstruct!{# [doc = " Similar to `CanonicalInput`, this carries the `typing_mode` and the environment"] # [doc = " necessary to do any kind of trait solving inside of nested queries."] # [doc = ""] # [doc = " Unlike proper canonicalization, this requires the `param_env` and the `value` to not"] # [doc = " contain anything local to the `infcx` of the caller, so we don't actually canonicalize"] # [doc = " anything."] # [doc = ""] # [doc = " This should be created by using `infcx.pseudo_canonicalize_query(param_env, value)`"] # [doc = " or by using `typing_env.as_query_input(value)`."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] # [derive (HashStable , TypeVisitable , TypeFoldable)] pub struct PseudoCanonicalInput < 'tcx , T > { pub typing_env : TypingEnv < 'tcx > , pub value : T , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , HashStable , Encodable , Decodable)] pub struct Destructor { # [doc = " The `DefId` of the destructor method"] pub did : DefId , }}}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , HashStable , Encodable , Decodable)] pub struct AsyncDestructor { # [doc = " The `DefId` of the `impl AsyncDrop`"] pub impl_did : DefId , }}}
mkitem!{mkstruct!{# [derive (Clone , Copy , PartialEq , Eq , HashStable , TyEncodable , TyDecodable)] pub struct VariantFlags (u8) ;}}
mkitem!{bitflags :: bitflags ! { impl VariantFlags : u8 { const NO_VARIANT_FLAGS = 0 ; # [doc = " Indicates whether the field list of this variant is `#[non_exhaustive]`."] const IS_FIELD_LIST_NON_EXHAUSTIVE = 1 << 0 ; } }}
mkitem!{rustc_data_structures :: external_bitflags_debug ! { VariantFlags }}
mkitem!{mkstruct!{# [doc = " Definition of a variant -- a struct's fields or an enum variant."] # [derive (Debug , HashStable , TyEncodable , TyDecodable)] pub struct VariantDef { # [doc = " `DefId` that identifies the variant itself."] # [doc = " If this variant belongs to a struct or union, then this is a copy of its `DefId`."] pub def_id : DefId , # [doc = " `DefId` that identifies the variant's constructor."] # [doc = " If this variant is a struct variant, then this is `None`."] pub ctor : Option < (CtorKind , DefId) > , # [doc = " Variant or struct name."] pub name : Symbol , # [doc = " Discriminant of this variant."] pub discr : VariantDiscr , # [doc = " Fields of this variant."] pub fields : IndexVec < FieldIdx , FieldDef > , # [doc = " The error guarantees from parser, if any."] tainted : Option < ErrorGuaranteed > , # [doc = " Flags of the variant (e.g. is field list non-exhaustive)?"] flags : VariantFlags , }}}
mkitem!{mkimpl!{impl VariantDef { # [doc = " Creates a new `VariantDef`."] # [doc = ""] # [doc = " `variant_did` is the `DefId` that identifies the enum variant (if this `VariantDef`"] # [doc = " represents an enum variant)."] # [doc = ""] # [doc = " `ctor_did` is the `DefId` that identifies the constructor of unit or"] # [doc = " tuple-variants/structs. If this is a `struct`-variant then this should be `None`."] # [doc = ""] # [doc = " `parent_did` is the `DefId` of the `AdtDef` representing the enum or struct that"] # [doc = " owns this variant. It is used for checking if a struct has `#[non_exhaustive]` w/out having"] # [doc = " to go through the redirect of checking the ctor's attributes - but compiling a small crate"] # [doc = " requires loading the `AdtDef`s for all the structs in the universe (e.g., coherence for any"] # [doc = " built-in trait), and we do not want to load attributes twice."] # [doc = ""] # [doc = " If someone speeds up attribute loading to not be a performance concern, they can"] # [doc = " remove this hack and use the constructor `DefId` everywhere."] # [instrument (level = "debug")] pub fn new (name : Symbol , variant_did : Option < DefId > , ctor : Option < (CtorKind , DefId) > , discr : VariantDiscr , fields : IndexVec < FieldIdx , FieldDef > , parent_did : DefId , recover_tainted : Option < ErrorGuaranteed > , is_field_list_non_exhaustive : bool ,) -> Self { let mut flags = VariantFlags :: NO_VARIANT_FLAGS ; if is_field_list_non_exhaustive { flags |= VariantFlags :: IS_FIELD_LIST_NON_EXHAUSTIVE ; } VariantDef { def_id : variant_did . unwrap_or (parent_did) , ctor , name , discr , fields , flags , tainted : recover_tainted , } } # [doc = " Returns `true` if the field list of this variant is `#[non_exhaustive]`."] # [doc = ""] # [doc = " Note that this function will return `true` even if the type has been"] # [doc = " defined in the crate currently being compiled. If that's not what you"] # [doc = " want, see [`Self::field_list_has_applicable_non_exhaustive`]."] # [inline] pub fn is_field_list_non_exhaustive (& self) -> bool { self . flags . intersects (VariantFlags :: IS_FIELD_LIST_NON_EXHAUSTIVE) } # [doc = " Returns `true` if the field list of this variant is `#[non_exhaustive]`"] # [doc = " and the type has been defined in another crate."] # [inline] pub fn field_list_has_applicable_non_exhaustive (& self) -> bool { self . is_field_list_non_exhaustive () && ! self . def_id . is_local () } # [doc = " Computes the `Ident` of this variant by looking up the `Span`"] pub fn ident (& self , tcx : TyCtxt < '_ >) -> Ident { Ident :: new (self . name , tcx . def_ident_span (self . def_id) . unwrap ()) } # [doc = " Was this variant obtained as part of recovering from a syntactic error?"] # [inline] pub fn has_errors (& self) -> Result < () , ErrorGuaranteed > { self . tainted . map_or (Ok (()) , Err) } # [inline] pub fn ctor_kind (& self) -> Option < CtorKind > { self . ctor . map (| (kind , _) | kind) } # [inline] pub fn ctor_def_id (& self) -> Option < DefId > { self . ctor . map (| (_ , def_id) | def_id) } # [doc = " Returns the one field in this variant."] # [doc = ""] # [doc = " `panic!`s if there are no fields or multiple fields."] # [inline] pub fn single_field (& self) -> & FieldDef { assert ! (self . fields . len () == 1) ; & self . fields [FieldIdx :: ZERO] } # [doc = " Returns the last field in this variant, if present."] # [inline] pub fn tail_opt (& self) -> Option < & FieldDef > { self . fields . raw . last () } # [doc = " Returns the last field in this variant."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics, if the variant has no fields."] # [inline] pub fn tail (& self) -> & FieldDef { self . tail_opt () . expect ("expected unsized ADT to have a tail field") } # [doc = " Returns whether this variant has unsafe fields."] pub fn has_unsafe_fields (& self) -> bool { self . fields . iter () . any (| x | x . safety . is_unsafe ()) } }}}
mkitem!{mkimpl!{impl PartialEq for VariantDef { # [inline] fn eq (& self , other : & Self) -> bool { let Self { def_id : lhs_def_id , ctor : _ , name : _ , discr : _ , fields : _ , flags : _ , tainted : _ , } = & self ; let Self { def_id : rhs_def_id , ctor : _ , name : _ , discr : _ , fields : _ , flags : _ , tainted : _ , } = other ; let res = lhs_def_id == rhs_def_id ; if cfg ! (debug_assertions) && res { let deep = self . ctor == other . ctor && self . name == other . name && self . discr == other . discr && self . fields == other . fields && self . flags == other . flags ; assert ! (deep , "VariantDef for the same def-id has differing data") ; } res } }}}
mkitem!{mkimpl!{impl Eq for VariantDef { }}}
mkitem!{mkimpl!{impl Hash for VariantDef { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { let Self { def_id , ctor : _ , name : _ , discr : _ , fields : _ , flags : _ , tainted : _ } = & self ; def_id . hash (s) } }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , PartialEq , Eq , TyEncodable , TyDecodable , HashStable)] pub enum VariantDiscr { # [doc = " Explicit value for this variant, i.e., `X = 123`."] # [doc = " The `DefId` corresponds to the embedded constant."] Explicit (DefId) , # [doc = " The previous variant's discriminant plus one."] # [doc = " For efficiency reasons, the distance from the"] # [doc = " last `Explicit` discriminant is being stored,"] # [doc = " or `0` for the first variant, if it has none."] Relative (u32) , }}}
mkitem!{mkstruct!{# [derive (Debug , HashStable , TyEncodable , TyDecodable)] pub struct FieldDef { pub did : DefId , pub name : Symbol , pub vis : Visibility < DefId > , pub safety : hir :: Safety , pub value : Option < DefId > , }}}
mkitem!{mkimpl!{impl PartialEq for FieldDef { # [inline] fn eq (& self , other : & Self) -> bool { let Self { did : lhs_did , name : _ , vis : _ , safety : _ , value : _ } = & self ; let Self { did : rhs_did , name : _ , vis : _ , safety : _ , value : _ } = other ; let res = lhs_did == rhs_did ; if cfg ! (debug_assertions) && res { let deep = self . name == other . name && self . vis == other . vis && self . safety == other . safety ; assert ! (deep , "FieldDef for the same def-id has differing data") ; } res } }}}
mkitem!{mkimpl!{impl Eq for FieldDef { }}}
mkitem!{mkimpl!{impl Hash for FieldDef { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { let Self { did , name : _ , vis : _ , safety : _ , value : _ } = & self ; did . hash (s) } }}}
mkitem!{mkimpl!{impl < 'tcx > FieldDef { # [doc = " Returns the type of this field. The resulting type is not normalized. The `arg` is"] # [doc = " typically obtained via the second field of [`TyKind::Adt`]."] pub fn ty (& self , tcx : TyCtxt < 'tcx > , args : GenericArgsRef < 'tcx >) -> Ty < 'tcx > { tcx . type_of (self . did) . instantiate (tcx , args) } # [doc = " Computes the `Ident` of this variant by looking up the `Span`"] pub fn ident (& self , tcx : TyCtxt < '_ >) -> Ident { Ident :: new (self . name , tcx . def_ident_span (self . did) . unwrap ()) } }}}
mkitem!{mkenum!{# [derive (Debug , PartialEq , Eq)] pub enum ImplOverlapKind { # [doc = " These impls are always allowed to overlap."] Permitted { # [doc = " Whether or not the impl is permitted due to the trait being a `#[marker]` trait"] marker : bool , } , }}}
mkitem!{mkenum!{# [doc = " Useful source information about where a desugared associated type for an"] # [doc = " RPITIT originated from."] # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash , Encodable , Decodable , HashStable)] pub enum ImplTraitInTraitData { Trait { fn_def_id : DefId , opaque_def_id : DefId } , Impl { fn_def_id : DefId } , }}}
mkitem!{mkimpl!{impl < 'tcx > TyCtxt < 'tcx > { pub fn typeck_body (self , body : hir :: BodyId) -> & 'tcx TypeckResults < 'tcx > { self . typeck (self . hir_body_owner_def_id (body)) } pub fn provided_trait_methods (self , id : DefId) -> impl 'tcx + Iterator < Item = & 'tcx AssocItem > { self . associated_items (id) . in_definition_order () . filter (move | item | item . is_fn () && item . defaultness (self) . has_value ()) } pub fn repr_options_of_def (self , did : LocalDefId) -> ReprOptions { let mut flags = ReprFlags :: empty () ; let mut size = None ; let mut max_align : Option < Align > = None ; let mut min_pack : Option < Align > = None ; let mut field_shuffle_seed = self . def_path_hash (did . to_def_id ()) . 0 . to_smaller_hash () ; if let Some (user_seed) = self . sess . opts . unstable_opts . layout_seed { field_shuffle_seed ^= user_seed ; } if let Some (reprs) = find_attr ! (self . get_all_attrs (did) , AttributeKind :: Repr { reprs , .. } => reprs) { for (r , _) in reprs { flags . insert (match * r { attr :: ReprRust => ReprFlags :: empty () , attr :: ReprC => ReprFlags :: IS_C , attr :: ReprPacked (pack) => { min_pack = Some (if let Some (min_pack) = min_pack { min_pack . min (pack) } else { pack }) ; ReprFlags :: empty () } attr :: ReprTransparent => ReprFlags :: IS_TRANSPARENT , attr :: ReprSimd => ReprFlags :: IS_SIMD , attr :: ReprInt (i) => { size = Some (match i { attr :: IntType :: SignedInt (x) => match x { ast :: IntTy :: Isize => IntegerType :: Pointer (true) , ast :: IntTy :: I8 => IntegerType :: Fixed (Integer :: I8 , true) , ast :: IntTy :: I16 => IntegerType :: Fixed (Integer :: I16 , true) , ast :: IntTy :: I32 => IntegerType :: Fixed (Integer :: I32 , true) , ast :: IntTy :: I64 => IntegerType :: Fixed (Integer :: I64 , true) , ast :: IntTy :: I128 => IntegerType :: Fixed (Integer :: I128 , true) , } , attr :: IntType :: UnsignedInt (x) => match x { ast :: UintTy :: Usize => IntegerType :: Pointer (false) , ast :: UintTy :: U8 => IntegerType :: Fixed (Integer :: I8 , false) , ast :: UintTy :: U16 => IntegerType :: Fixed (Integer :: I16 , false) , ast :: UintTy :: U32 => IntegerType :: Fixed (Integer :: I32 , false) , ast :: UintTy :: U64 => IntegerType :: Fixed (Integer :: I64 , false) , ast :: UintTy :: U128 => IntegerType :: Fixed (Integer :: I128 , false) , } , }) ; ReprFlags :: empty () } attr :: ReprAlign (align) => { max_align = max_align . max (Some (align)) ; ReprFlags :: empty () } }) ; } } if self . sess . opts . unstable_opts . randomize_layout { flags . insert (ReprFlags :: RANDOMIZE_LAYOUT) ; } let is_box = self . is_lang_item (did . to_def_id () , LangItem :: OwnedBox) ; if is_box { flags . insert (ReprFlags :: IS_LINEAR) ; } ReprOptions { int : size , align : max_align , pack : min_pack , flags , field_shuffle_seed } } # [doc = " Look up the name of a definition across crates. This does not look at HIR."] pub fn opt_item_name (self , def_id : impl IntoQueryParam < DefId >) -> Option < Symbol > { let def_id = def_id . into_query_param () ; if let Some (cnum) = def_id . as_crate_root () { Some (self . crate_name (cnum)) } else { let def_key = self . def_key (def_id) ; match def_key . disambiguated_data . data { rustc_hir :: definitions :: DefPathData :: Ctor => self . opt_item_name (DefId { krate : def_id . krate , index : def_key . parent . unwrap () }) , _ => def_key . get_opt_name () , } } } # [doc = " Look up the name of a definition across crates. This does not look at HIR."] # [doc = ""] # [doc = " This method will ICE if the corresponding item does not have a name. In these cases, use"] # [doc = " [`opt_item_name`] instead."] # [doc = ""] # [doc = " [`opt_item_name`]: Self::opt_item_name"] pub fn item_name (self , id : impl IntoQueryParam < DefId >) -> Symbol { let id = id . into_query_param () ; self . opt_item_name (id) . unwrap_or_else (| | { bug ! ("item_name: no name for {:?}" , self . def_path (id)) ; }) } # [doc = " Look up the name and span of a definition."] # [doc = ""] # [doc = " See [`item_name`][Self::item_name] for more information."] pub fn opt_item_ident (self , def_id : impl IntoQueryParam < DefId >) -> Option < Ident > { let def_id = def_id . into_query_param () ; let def = self . opt_item_name (def_id) ? ; let span = self . def_ident_span (def_id) . unwrap_or_else (| | bug ! ("missing ident span for {def_id:?}")) ; Some (Ident :: new (def , span)) } # [doc = " Look up the name and span of a definition."] # [doc = ""] # [doc = " See [`item_name`][Self::item_name] for more information."] pub fn item_ident (self , def_id : impl IntoQueryParam < DefId >) -> Ident { let def_id = def_id . into_query_param () ; self . opt_item_ident (def_id) . unwrap_or_else (| | { bug ! ("item_ident: no name for {:?}" , self . def_path (def_id)) ; }) } pub fn opt_associated_item (self , def_id : DefId) -> Option < AssocItem > { if let DefKind :: AssocConst | DefKind :: AssocFn | DefKind :: AssocTy = self . def_kind (def_id) { Some (self . associated_item (def_id)) } else { None } } # [doc = " If the `def_id` is an associated type that was desugared from a"] # [doc = " return-position `impl Trait` from a trait, then provide the source info"] # [doc = " about where that RPITIT came from."] pub fn opt_rpitit_info (self , def_id : DefId) -> Option < ImplTraitInTraitData > { if let DefKind :: AssocTy = self . def_kind (def_id) && let AssocKind :: Type { data : AssocTypeData :: Rpitit (rpitit_info) } = self . associated_item (def_id) . kind { Some (rpitit_info) } else { None } } pub fn find_field_index (self , ident : Ident , variant : & VariantDef) -> Option < FieldIdx > { variant . fields . iter_enumerated () . find_map (| (i , field) | { self . hygienic_eq (ident , field . ident (self) , variant . def_id) . then_some (i) }) } # [doc = " Returns `Some` if the impls are the same polarity and the trait either"] # [doc = " has no items or is annotated `#[marker]` and prevents item overrides."] # [instrument (level = "debug" , skip (self) , ret)] pub fn impls_are_allowed_to_overlap (self , def_id1 : DefId , def_id2 : DefId ,) -> Option < ImplOverlapKind > { let impl1 = self . impl_trait_header (def_id1) . unwrap () ; let impl2 = self . impl_trait_header (def_id2) . unwrap () ; let trait_ref1 = impl1 . trait_ref . skip_binder () ; let trait_ref2 = impl2 . trait_ref . skip_binder () ; if trait_ref1 . references_error () || trait_ref2 . references_error () { return Some (ImplOverlapKind :: Permitted { marker : false }) ; } match (impl1 . polarity , impl2 . polarity) { (ImplPolarity :: Reservation , _) | (_ , ImplPolarity :: Reservation) => { return Some (ImplOverlapKind :: Permitted { marker : false }) ; } (ImplPolarity :: Positive , ImplPolarity :: Negative) | (ImplPolarity :: Negative , ImplPolarity :: Positive) => { return None ; } (ImplPolarity :: Positive , ImplPolarity :: Positive) | (ImplPolarity :: Negative , ImplPolarity :: Negative) => { } } ; let is_marker_impl = | trait_ref : TraitRef < '_ > | self . trait_def (trait_ref . def_id) . is_marker ; let is_marker_overlap = is_marker_impl (trait_ref1) && is_marker_impl (trait_ref2) ; if is_marker_overlap { return Some (ImplOverlapKind :: Permitted { marker : true }) ; } None } # [doc = " Returns `ty::VariantDef` if `res` refers to a struct,"] # [doc = " or variant or their constructors, panics otherwise."] pub fn expect_variant_res (self , res : Res) -> & 'tcx VariantDef { match res { Res :: Def (DefKind :: Variant , did) => { let enum_did = self . parent (did) ; self . adt_def (enum_did) . variant_with_id (did) } Res :: Def (DefKind :: Struct | DefKind :: Union , did) => self . adt_def (did) . non_enum_variant () , Res :: Def (DefKind :: Ctor (CtorOf :: Variant , ..) , variant_ctor_did) => { let variant_did = self . parent (variant_ctor_did) ; let enum_did = self . parent (variant_did) ; self . adt_def (enum_did) . variant_with_ctor_id (variant_ctor_did) } Res :: Def (DefKind :: Ctor (CtorOf :: Struct , ..) , ctor_did) => { let struct_did = self . parent (ctor_did) ; self . adt_def (struct_did) . non_enum_variant () } _ => bug ! ("expect_variant_res used with unexpected res {:?}" , res) , } } # [doc = " Returns the possibly-auto-generated MIR of a [`ty::InstanceKind`]."] # [instrument (skip (self) , level = "debug")] pub fn instance_mir (self , instance : ty :: InstanceKind < 'tcx >) -> & 'tcx Body < 'tcx > { match instance { ty :: InstanceKind :: Item (def) => { debug ! ("calling def_kind on def: {:?}" , def) ; let def_kind = self . def_kind (def) ; debug ! ("returned from def_kind: {:?}" , def_kind) ; match def_kind { DefKind :: Const | DefKind :: Static { .. } | DefKind :: AssocConst | DefKind :: Ctor (..) | DefKind :: AnonConst | DefKind :: InlineConst => self . mir_for_ctfe (def) , _ => self . optimized_mir (def) , } } ty :: InstanceKind :: VTableShim (..) | ty :: InstanceKind :: ReifyShim (..) | ty :: InstanceKind :: Intrinsic (..) | ty :: InstanceKind :: FnPtrShim (..) | ty :: InstanceKind :: Virtual (..) | ty :: InstanceKind :: ClosureOnceShim { .. } | ty :: InstanceKind :: ConstructCoroutineInClosureShim { .. } | ty :: InstanceKind :: FutureDropPollShim (..) | ty :: InstanceKind :: DropGlue (..) | ty :: InstanceKind :: CloneShim (..) | ty :: InstanceKind :: ThreadLocalShim (..) | ty :: InstanceKind :: FnPtrAddrShim (..) | ty :: InstanceKind :: AsyncDropGlueCtorShim (..) | ty :: InstanceKind :: AsyncDropGlue (..) => self . mir_shims (instance) , } } # [doc = " Gets all attributes with the given name."] pub fn get_attrs (self , did : impl Into < DefId > , attr : Symbol ,) -> impl Iterator < Item = & 'tcx hir :: Attribute > { self . get_all_attrs (did) . iter () . filter (move | a : & & hir :: Attribute | a . has_name (attr)) } # [doc = " Gets all attributes."] # [doc = ""] # [doc = " To see if an item has a specific attribute, you should use"] # [doc = " [`rustc_hir::find_attr!`] so you can use matching."] pub fn get_all_attrs (self , did : impl Into < DefId >) -> & 'tcx [hir :: Attribute] { let did : DefId = did . into () ; if let Some (did) = did . as_local () { self . hir_attrs (self . local_def_id_to_hir_id (did)) } else { self . attrs_for_def (did) } } # [doc = " Get an attribute from the diagnostic attribute namespace"] # [doc = ""] # [doc = " This function requests an attribute with the following structure:"] # [doc = ""] # [doc = " `#[diagnostic::$attr]`"] # [doc = ""] # [doc = " This function performs feature checking, so if an attribute is returned"] # [doc = " it can be used by the consumer"] pub fn get_diagnostic_attr (self , did : impl Into < DefId > , attr : Symbol ,) -> Option < & 'tcx hir :: Attribute > { let did : DefId = did . into () ; if did . as_local () . is_some () { if rustc_feature :: is_stable_diagnostic_attribute (attr , self . features ()) { self . get_attrs_by_path (did , & [sym :: diagnostic , sym :: do_not_recommend]) . next () } else { None } } else { debug_assert ! (rustc_feature :: encode_cross_crate (attr)) ; self . attrs_for_def (did) . iter () . find (| a | matches ! (a . path () . as_ref () , [sym :: diagnostic , a] if * a == attr)) } } pub fn get_attrs_by_path (self , did : DefId , attr : & [Symbol] ,) -> impl Iterator < Item = & 'tcx hir :: Attribute > { let filter_fn = move | a : & & hir :: Attribute | a . path_matches (attr) ; if let Some (did) = did . as_local () { self . hir_attrs (self . local_def_id_to_hir_id (did)) . iter () . filter (filter_fn) } else { self . attrs_for_def (did) . iter () . filter (filter_fn) } } pub fn get_attr (self , did : impl Into < DefId > , attr : Symbol) -> Option < & 'tcx hir :: Attribute > { if cfg ! (debug_assertions) && ! rustc_feature :: is_valid_for_get_attr (attr) { let did : DefId = did . into () ; bug ! ("get_attr: unexpected called with DefId `{:?}`, attr `{:?}`" , did , attr) ; } else { self . get_attrs (did , attr) . next () } } # [doc = " Determines whether an item is annotated with an attribute."] pub fn has_attr (self , did : impl Into < DefId > , attr : Symbol) -> bool { self . get_attrs (did , attr) . next () . is_some () } # [doc = " Determines whether an item is annotated with a multi-segment attribute"] pub fn has_attrs_with_path (self , did : impl Into < DefId > , attrs : & [Symbol]) -> bool { self . get_attrs_by_path (did . into () , attrs) . next () . is_some () } # [doc = " Returns `true` if this is an `auto trait`."] pub fn trait_is_auto (self , trait_def_id : DefId) -> bool { self . trait_def (trait_def_id) . has_auto_impl } # [doc = " Returns `true` if this is coinductive, either because it is"] # [doc = " an auto trait or because it has the `#[rustc_coinductive]` attribute."] pub fn trait_is_coinductive (self , trait_def_id : DefId) -> bool { self . trait_def (trait_def_id) . is_coinductive } # [doc = " Returns `true` if this is a trait alias."] pub fn trait_is_alias (self , trait_def_id : DefId) -> bool { self . def_kind (trait_def_id) == DefKind :: TraitAlias } # [doc = " Arena-alloc of LayoutError for coroutine layout"] fn layout_error (self , err : LayoutError < 'tcx >) -> & 'tcx LayoutError < 'tcx > { self . arena . alloc (err) } # [doc = " Returns layout of a non-async-drop coroutine. Layout might be unavailable if the"] # [doc = " coroutine is tainted by errors."] # [doc = ""] # [doc = " Takes `coroutine_kind` which can be acquired from the `CoroutineArgs::kind_ty`,"] # [doc = " e.g. `args.as_coroutine().kind_ty()`."] fn ordinary_coroutine_layout (self , def_id : DefId , args : GenericArgsRef < 'tcx > ,) -> Result < & 'tcx CoroutineLayout < 'tcx > , & 'tcx LayoutError < 'tcx > > { let coroutine_kind_ty = args . as_coroutine () . kind_ty () ; let mir = self . optimized_mir (def_id) ; let ty = | | Ty :: new_coroutine (self , def_id , args) ; if coroutine_kind_ty . is_unit () { mir . coroutine_layout_raw () . ok_or_else (| | self . layout_error (LayoutError :: Unknown (ty ()))) } else { let ty :: Coroutine (_ , identity_args) = * self . type_of (def_id) . instantiate_identity () . kind () else { unreachable ! () ; } ; let identity_kind_ty = identity_args . as_coroutine () . kind_ty () ; if identity_kind_ty == coroutine_kind_ty { mir . coroutine_layout_raw () . ok_or_else (| | self . layout_error (LayoutError :: Unknown (ty ()))) } else { assert_matches ! (coroutine_kind_ty . to_opt_closure_kind () , Some (ClosureKind :: FnOnce)) ; assert_matches ! (identity_kind_ty . to_opt_closure_kind () , Some (ClosureKind :: Fn | ClosureKind :: FnMut)) ; self . optimized_mir (self . coroutine_by_move_body_def_id (def_id)) . coroutine_layout_raw () . ok_or_else (| | self . layout_error (LayoutError :: Unknown (ty ()))) } } } # [doc = " Returns layout of a `async_drop_in_place::{closure}` coroutine"] # [doc = "   (returned from `async fn async_drop_in_place<T>(..)`)."] # [doc = " Layout might be unavailable if the coroutine is tainted by errors."] fn async_drop_coroutine_layout (self , def_id : DefId , args : GenericArgsRef < 'tcx > ,) -> Result < & 'tcx CoroutineLayout < 'tcx > , & 'tcx LayoutError < 'tcx > > { let ty = | | Ty :: new_coroutine (self , def_id , args) ; if args [0] . has_placeholders () || args [0] . has_non_region_param () { return Err (self . layout_error (LayoutError :: TooGeneric (ty ()))) ; } let instance = InstanceKind :: AsyncDropGlue (def_id , Ty :: new_coroutine (self , def_id , args)) ; self . mir_shims (instance) . coroutine_layout_raw () . ok_or_else (| | self . layout_error (LayoutError :: Unknown (ty ()))) } # [doc = " Returns layout of a coroutine. Layout might be unavailable if the"] # [doc = " coroutine is tainted by errors."] pub fn coroutine_layout (self , def_id : DefId , args : GenericArgsRef < 'tcx > ,) -> Result < & 'tcx CoroutineLayout < 'tcx > , & 'tcx LayoutError < 'tcx > > { if self . is_async_drop_in_place_coroutine (def_id) { let arg_cor_ty = args . first () . unwrap () . expect_ty () ; if arg_cor_ty . is_coroutine () { let span = self . def_span (def_id) ; let source_info = SourceInfo :: outermost (span) ; let variant_fields : IndexVec < VariantIdx , IndexVec < FieldIdx , CoroutineSavedLocal > > = iter :: repeat (IndexVec :: new ()) . take (CoroutineArgs :: RESERVED_VARIANTS) . collect () ; let variant_source_info : IndexVec < VariantIdx , SourceInfo > = iter :: repeat (source_info) . take (CoroutineArgs :: RESERVED_VARIANTS) . collect () ; let proxy_layout = CoroutineLayout { field_tys : [] . into () , field_names : [] . into () , variant_fields , variant_source_info , storage_conflicts : BitMatrix :: new (0 , 0) , } ; return Ok (self . arena . alloc (proxy_layout)) ; } else { self . async_drop_coroutine_layout (def_id , args) } } else { self . ordinary_coroutine_layout (def_id , args) } } # [doc = " Given the `DefId` of an impl, returns the `DefId` of the trait it implements."] # [doc = " If it implements no trait, returns `None`."] pub fn trait_id_of_impl (self , def_id : DefId) -> Option < DefId > { self . impl_trait_ref (def_id) . map (| tr | tr . skip_binder () . def_id) } # [doc = " If the given `DefId` is an associated item, returns the `DefId` and `DefKind` of the parent trait or impl."] pub fn assoc_parent (self , def_id : DefId) -> Option < (DefId , DefKind) > { if ! self . def_kind (def_id) . is_assoc () { return None ; } let parent = self . parent (def_id) ; let def_kind = self . def_kind (parent) ; Some ((parent , def_kind)) } # [doc = " Returns the trait item that is implemented by the given item `DefId`."] pub fn trait_item_of (self , def_id : impl IntoQueryParam < DefId >) -> Option < DefId > { self . opt_associated_item (def_id . into_query_param ()) ? . trait_item_def_id () } # [doc = " If the given `DefId` is an associated item of a trait,"] # [doc = " returns the `DefId` of the trait; otherwise, returns `None`."] pub fn trait_of_assoc (self , def_id : DefId) -> Option < DefId > { match self . assoc_parent (def_id) { Some ((id , DefKind :: Trait)) => Some (id) , _ => None , } } # [doc = " If the given `DefId` is an associated item of an impl,"] # [doc = " returns the `DefId` of the impl; otherwise returns `None`."] pub fn impl_of_assoc (self , def_id : DefId) -> Option < DefId > { match self . assoc_parent (def_id) { Some ((id , DefKind :: Impl { .. })) => Some (id) , _ => None , } } # [doc = " If the given `DefId` is an associated item of an inherent impl,"] # [doc = " returns the `DefId` of the impl; otherwise, returns `None`."] pub fn inherent_impl_of_assoc (self , def_id : DefId) -> Option < DefId > { match self . assoc_parent (def_id) { Some ((id , DefKind :: Impl { of_trait : false })) => Some (id) , _ => None , } } # [doc = " If the given `DefId` is an associated item of a trait impl,"] # [doc = " returns the `DefId` of the impl; otherwise, returns `None`."] pub fn trait_impl_of_assoc (self , def_id : DefId) -> Option < DefId > { match self . assoc_parent (def_id) { Some ((id , DefKind :: Impl { of_trait : true })) => Some (id) , _ => None , } } pub fn is_exportable (self , def_id : DefId) -> bool { self . exportable_items (def_id . krate) . contains (& def_id) } # [doc = " Check if the given `DefId` is `#\\[automatically_derived\\]`, *and*"] # [doc = " whether it was produced by expanding a builtin derive macro."] pub fn is_builtin_derived (self , def_id : DefId) -> bool { if self . is_automatically_derived (def_id) && let Some (def_id) = def_id . as_local () && let outer = self . def_span (def_id) . ctxt () . outer_expn_data () && matches ! (outer . kind , ExpnKind :: Macro (MacroKind :: Derive , _)) && find_attr ! (self . get_all_attrs (outer . macro_def_id . unwrap ()) , AttributeKind :: RustcBuiltinMacro { .. }) { true } else { false } } # [doc = " Check if the given `DefId` is `#\\[automatically_derived\\]`."] pub fn is_automatically_derived (self , def_id : DefId) -> bool { find_attr ! (self . get_all_attrs (def_id) , AttributeKind :: AutomaticallyDerived (..)) } # [doc = " Looks up the span of `impl_did` if the impl is local; otherwise returns `Err`"] # [doc = " with the name of the crate containing the impl."] pub fn span_of_impl (self , impl_def_id : DefId) -> Result < Span , Symbol > { if let Some (impl_def_id) = impl_def_id . as_local () { Ok (self . def_span (impl_def_id)) } else { Err (self . crate_name (impl_def_id . krate)) } } # [doc = " Hygienically compares a use-site name (`use_name`) for a field or an associated item with"] # [doc = " its supposed definition name (`def_name`). The method also needs `DefId` of the supposed"] # [doc = " definition's parent/scope to perform comparison."] pub fn hygienic_eq (self , use_ident : Ident , def_ident : Ident , def_parent_def_id : DefId) -> bool { use_ident . name == def_ident . name && use_ident . span . ctxt () . hygienic_eq (def_ident . span . ctxt () , self . expn_that_defined (def_parent_def_id)) } pub fn adjust_ident (self , mut ident : Ident , scope : DefId) -> Ident { ident . span . normalize_to_macros_2_0_and_adjust (self . expn_that_defined (scope)) ; ident } pub fn adjust_ident_and_get_scope (self , mut ident : Ident , scope : DefId , block : hir :: HirId ,) -> (Ident , DefId) { let scope = ident . span . normalize_to_macros_2_0_and_adjust (self . expn_that_defined (scope)) . and_then (| actual_expansion | actual_expansion . expn_data () . parent_module) . unwrap_or_else (| | self . parent_module (block) . to_def_id ()) ; (ident , scope) } # [doc = " Checks whether this is a `const fn`. Returns `false` for non-functions."] # [doc = ""] # [doc = " Even if this returns `true`, constness may still be unstable!"] # [inline] pub fn is_const_fn (self , def_id : DefId) -> bool { matches ! (self . def_kind (def_id) , DefKind :: Fn | DefKind :: AssocFn | DefKind :: Ctor (_ , CtorKind :: Fn) | DefKind :: Closure) && self . constness (def_id) == hir :: Constness :: Const } # [doc = " Whether this item is conditionally constant for the purposes of the"] # [doc = " effects implementation."] # [doc = ""] # [doc = " This roughly corresponds to all const functions and other callable"] # [doc = " items, along with const impls and traits, and associated types within"] # [doc = " those impls and traits."] pub fn is_conditionally_const (self , def_id : impl Into < DefId >) -> bool { let def_id : DefId = def_id . into () ; match self . def_kind (def_id) { DefKind :: Impl { of_trait : true } => { let header = self . impl_trait_header (def_id) . unwrap () ; header . constness == hir :: Constness :: Const && self . is_const_trait (header . trait_ref . skip_binder () . def_id) } DefKind :: Fn | DefKind :: Ctor (_ , CtorKind :: Fn) => { self . constness (def_id) == hir :: Constness :: Const } DefKind :: Trait => self . is_const_trait (def_id) , DefKind :: AssocTy => { let parent_def_id = self . parent (def_id) ; match self . def_kind (parent_def_id) { DefKind :: Impl { of_trait : false } => false , DefKind :: Impl { of_trait : true } | DefKind :: Trait => { self . is_conditionally_const (parent_def_id) } _ => bug ! ("unexpected parent item of associated type: {parent_def_id:?}") , } } DefKind :: AssocFn => { let parent_def_id = self . parent (def_id) ; match self . def_kind (parent_def_id) { DefKind :: Impl { of_trait : false } => { self . constness (def_id) == hir :: Constness :: Const } DefKind :: Impl { of_trait : true } | DefKind :: Trait => { self . is_conditionally_const (parent_def_id) } _ => bug ! ("unexpected parent item of associated fn: {parent_def_id:?}") , } } DefKind :: OpaqueTy => match self . opaque_ty_origin (def_id) { hir :: OpaqueTyOrigin :: FnReturn { parent , .. } => self . is_conditionally_const (parent) , hir :: OpaqueTyOrigin :: AsyncFn { .. } => false , hir :: OpaqueTyOrigin :: TyAlias { .. } => false , } , DefKind :: Closure => { false } DefKind :: Ctor (_ , CtorKind :: Const) | DefKind :: Impl { of_trait : false } | DefKind :: Mod | DefKind :: Struct | DefKind :: Union | DefKind :: Enum | DefKind :: Variant | DefKind :: TyAlias | DefKind :: ForeignTy | DefKind :: TraitAlias | DefKind :: TyParam | DefKind :: Const | DefKind :: ConstParam | DefKind :: Static { .. } | DefKind :: AssocConst | DefKind :: Macro (_) | DefKind :: ExternCrate | DefKind :: Use | DefKind :: ForeignMod | DefKind :: AnonConst | DefKind :: InlineConst | DefKind :: Field | DefKind :: LifetimeParam | DefKind :: GlobalAsm | DefKind :: SyntheticCoroutineBody => false , } } # [inline] pub fn is_const_trait (self , def_id : DefId) -> bool { self . trait_def (def_id) . constness == hir :: Constness :: Const } # [inline] pub fn is_const_default_method (self , def_id : DefId) -> bool { matches ! (self . trait_of_assoc (def_id) , Some (trait_id) if self . is_const_trait (trait_id)) } pub fn impl_method_has_trait_impl_trait_tys (self , def_id : DefId) -> bool { if self . def_kind (def_id) != DefKind :: AssocFn { return false ; } let Some (item) = self . opt_associated_item (def_id) else { return false ; } ; let AssocContainer :: TraitImpl (Ok (trait_item_def_id)) = item . container else { return false ; } ; ! self . associated_types_for_impl_traits_in_associated_fn (trait_item_def_id) . is_empty () } }}}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { closure :: provide (providers) ; context :: provide (providers) ; erase_regions :: provide (providers) ; inhabitedness :: provide (providers) ; util :: provide (providers) ; print :: provide (providers) ; super :: util :: bug :: provide (providers) ; * providers = Providers { trait_impls_of : trait_def :: trait_impls_of_provider , incoherent_impls : trait_def :: incoherent_impls_provider , trait_impls_in_crate : trait_def :: trait_impls_in_crate_provider , traits : trait_def :: traits_provider , vtable_allocation : vtable :: vtable_allocation_provider , .. * providers } ; }
}
mkitem!{mkstruct!{# [doc = " A map for the local crate mapping each type to a vector of its"] # [doc = " inherent impls. This is not meant to be used outside of coherence;"] # [doc = " rather, you should request the vector for a specific type via"] # [doc = " `tcx.inherent_impls(def_id)` so as to minimize your dependencies"] # [doc = " (constructing this map requires touching the entire crate)."] # [derive (Clone , Debug , Default , HashStable)] pub struct CrateInherentImpls { pub inherent_impls : FxIndexMap < LocalDefId , Vec < DefId > > , pub incoherent_impls : FxIndexMap < SimplifiedType , Vec < LocalDefId > > , }}}
mkitem!{mkstruct!{# [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , TyEncodable , HashStable)] pub struct SymbolName < 'tcx > { # [doc = " `&str` gives a consistent ordering, which ensures reproducible builds."] pub name : & 'tcx str , }}}
mkitem!{mkimpl!{impl < 'tcx > SymbolName < 'tcx > { pub fn new (tcx : TyCtxt < 'tcx > , name : & str) -> SymbolName < 'tcx > { SymbolName { name : tcx . arena . alloc_str (name) } } }}}
mkitem!{mkimpl!{impl < 'tcx > fmt :: Display for SymbolName < 'tcx > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . name , fmt) } }}}
mkitem!{mkimpl!{impl < 'tcx > fmt :: Debug for SymbolName < 'tcx > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Display :: fmt (& self . name , fmt) } }}}
mkitem!{mkstruct!{# [doc = " The constituent parts of a type level constant of kind ADT or array."] # [derive (Copy , Clone , Debug , HashStable)] pub struct DestructuredConst < 'tcx > { pub variant : Option < VariantIdx > , pub fields : & 'tcx [ty :: Const < 'tcx >] , }}}