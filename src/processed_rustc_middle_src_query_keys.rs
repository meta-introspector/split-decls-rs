/* FP:keys.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0001
/* FP:keys.rs-0002 */ use std :: ffi :: OsStr ;
/* FP:keys.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0002
/* FP:keys.rs-0004 */ use crate :: rustc_complete :: def_id :: { CrateNum , DefId , LOCAL_CRATE , LocalDefId , LocalModDefId , ModDefId } ;
/* FP:keys.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0003
/* FP:keys.rs-0006 */ use crate :: rustc_complete :: hir_id :: { HirId , OwnerId } ;
/* FP:keys.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0004
/* FP:keys.rs-0008 */ use rustc_query_system :: dep_graph :: DepNodeIndex ;
/* FP:keys.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0005
/* FP:keys.rs-0010 */ use rustc_query_system :: query :: { DefIdCache , DefaultCache , SingleCache , VecCache } ;
/* FP:keys.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0006
/* FP:keys.rs-0012 */ use crate :: rustc_complete :: { DUMMY_SP , Ident , Span , Symbol } ;
/* FP:keys.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0007
/* FP:keys.rs-0014 */ use crate :: infer :: canonical :: CanonicalQueryInput ;
/* FP:keys.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0008
/* FP:keys.rs-0016 */ use crate :: mir :: mono :: CollectionMode ;
/* FP:keys.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0009
/* FP:keys.rs-0018 */ use crate :: ty :: fast_reject :: SimplifiedType ;
/* FP:keys.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0010
/* FP:keys.rs-0020 */ use crate :: ty :: layout :: { TyAndLayout , ValidityRequirement } ;
/* FP:keys.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0011
/* FP:keys.rs-0022 */ use crate :: ty :: { self , GenericArg , GenericArgsRef , Ty , TyCtxt } ;
/* FP:keys.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_USE_0012
/* FP:keys.rs-0024 */ use crate :: { mir , traits } ;
/* FP:keys.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_STRUCT_0013
/* FP:keys.rs-0026 */ # [doc = " Placeholder for `CrateNum`'s \"local\" counterpart"] # [derive (Copy , Clone , Debug)] pub struct LocalCrate ;
/* FP:keys.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_TRAIT_0014
/* FP:keys.rs-0028 */ # [doc = " The `Key` trait controls what types can legally be used as the key"] # [doc = " for a query."] pub trait Key : Sized { # [doc = " The type of in-memory cache to use for queries with this key type."] # [doc = ""] # [doc = " In practice the cache type must implement [`QueryCache`], though that"] # [doc = " constraint is not enforced here."] # [doc = ""] # [doc = " [`QueryCache`]: rustc_query_system::query::QueryCache"] type Cache < V > ; # [doc = " In the event that a cycle occurs, if no explicit span has been"] # [doc = " given for a query with key `self`, what span should we use?"] fn default_span (& self , tcx : TyCtxt < '_ >) -> Span ; # [doc = " If the key is a [`DefId`] or `DefId`--equivalent, return that `DefId`."] # [doc = " Otherwise, return `None`."] fn key_as_def_id (& self) -> Option < DefId > { None } # [doc = " Used to detect when ADT def ids are used as keys in a cycle for better error reporting."] fn def_id_for_ty_in_cycle (& self) -> Option < DefId > { None } }
/* FP:keys.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_TRAIT_0015
/* FP:keys.rs-0030 */ pub trait AsLocalKey : Key { type LocalKey ; # [doc = " Given an instance of this key, what crate is it referring to?"] # [doc = " This is used to find the provider."] fn as_local_key (& self) -> Option < Self :: LocalKey > ; }
/* FP:keys.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0016
/* FP:keys.rs-0032 */ impl Key for () { type Cache < V > = SingleCache < V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0017
/* FP:keys.rs-0034 */ impl < 'tcx > Key for ty :: InstanceKind < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (self . def_id ()) } }
/* FP:keys.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0018
/* FP:keys.rs-0036 */ impl < 'tcx > AsLocalKey for ty :: InstanceKind < 'tcx > { type LocalKey = Self ; # [inline (always)] fn as_local_key (& self) -> Option < Self :: LocalKey > { self . def_id () . is_local () . then (| | * self) } }
/* FP:keys.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0019
/* FP:keys.rs-0038 */ impl < 'tcx > Key for ty :: Instance < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (self . def_id ()) } }
/* FP:keys.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0020
/* FP:keys.rs-0040 */ impl < 'tcx > Key for mir :: interpret :: GlobalId < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . instance . default_span (tcx) } }
/* FP:keys.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0021
/* FP:keys.rs-0042 */ impl < 'tcx > Key for (Ty < 'tcx > , Option < ty :: ExistentialTraitRef < 'tcx > >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0022
/* FP:keys.rs-0044 */ impl < 'tcx > Key for mir :: interpret :: LitToConstInput < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0023
/* FP:keys.rs-0046 */ impl Key for CrateNum { type Cache < V > = VecCache < Self , V , DepNodeIndex > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0024
/* FP:keys.rs-0048 */ impl AsLocalKey for CrateNum { type LocalKey = LocalCrate ; # [inline (always)] fn as_local_key (& self) -> Option < Self :: LocalKey > { (* self == LOCAL_CRATE) . then_some (LocalCrate) } }
/* FP:keys.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0025
/* FP:keys.rs-0050 */ impl Key for OwnerId { type Cache < V > = VecCache < Self , V , DepNodeIndex > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . to_def_id () . default_span (tcx) } fn key_as_def_id (& self) -> Option < DefId > { Some (self . to_def_id ()) } }
/* FP:keys.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0026
/* FP:keys.rs-0052 */ impl Key for LocalDefId { type Cache < V > = VecCache < Self , V , DepNodeIndex > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . to_def_id () . default_span (tcx) } fn key_as_def_id (& self) -> Option < DefId > { Some (self . to_def_id ()) } }
/* FP:keys.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0027
/* FP:keys.rs-0054 */ impl Key for DefId { type Cache < V > = DefIdCache < V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (* self) } # [inline (always)] fn key_as_def_id (& self) -> Option < DefId > { Some (* self) } }
/* FP:keys.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0028
/* FP:keys.rs-0056 */ impl AsLocalKey for DefId { type LocalKey = LocalDefId ; # [inline (always)] fn as_local_key (& self) -> Option < Self :: LocalKey > { self . as_local () } }
/* FP:keys.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0029
/* FP:keys.rs-0058 */ impl Key for LocalModDefId { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (* self) } # [inline (always)] fn key_as_def_id (& self) -> Option < DefId > { Some (self . to_def_id ()) } }
/* FP:keys.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0030
/* FP:keys.rs-0060 */ impl Key for ModDefId { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (* self) } # [inline (always)] fn key_as_def_id (& self) -> Option < DefId > { Some (self . to_def_id ()) } }
/* FP:keys.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0031
/* FP:keys.rs-0062 */ impl AsLocalKey for ModDefId { type LocalKey = LocalModDefId ; # [inline (always)] fn as_local_key (& self) -> Option < Self :: LocalKey > { self . as_local () } }
/* FP:keys.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0032
/* FP:keys.rs-0064 */ impl Key for SimplifiedType { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0033
/* FP:keys.rs-0066 */ impl Key for (DefId , DefId) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 1 . default_span (tcx) } }
/* FP:keys.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0034
/* FP:keys.rs-0068 */ impl < 'tcx > Key for (ty :: Instance < 'tcx > , LocalDefId) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 0 . default_span (tcx) } }
/* FP:keys.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0035
/* FP:keys.rs-0070 */ impl Key for (DefId , LocalDefId) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 1 . default_span (tcx) } }
/* FP:keys.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0036
/* FP:keys.rs-0072 */ impl Key for (LocalDefId , DefId) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 0 . default_span (tcx) } }
/* FP:keys.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0037
/* FP:keys.rs-0074 */ impl Key for (LocalDefId , LocalDefId) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 0 . default_span (tcx) } }
/* FP:keys.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0038
/* FP:keys.rs-0076 */ impl Key for (DefId , Ident) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (self . 0) } # [inline (always)] fn key_as_def_id (& self) -> Option < DefId > { Some (self . 0) } }
/* FP:keys.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0039
/* FP:keys.rs-0078 */ impl Key for (LocalDefId , LocalDefId , Ident) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 1 . default_span (tcx) } }
/* FP:keys.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0040
/* FP:keys.rs-0080 */ impl Key for (CrateNum , DefId) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 1 . default_span (tcx) } }
/* FP:keys.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0041
/* FP:keys.rs-0082 */ impl AsLocalKey for (CrateNum , DefId) { type LocalKey = DefId ; # [inline (always)] fn as_local_key (& self) -> Option < Self :: LocalKey > { (self . 0 == LOCAL_CRATE) . then (| | self . 1) } }
/* FP:keys.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0042
/* FP:keys.rs-0084 */ impl Key for (CrateNum , SimplifiedType) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0043
/* FP:keys.rs-0086 */ impl AsLocalKey for (CrateNum , SimplifiedType) { type LocalKey = SimplifiedType ; # [inline (always)] fn as_local_key (& self) -> Option < Self :: LocalKey > { (self . 0 == LOCAL_CRATE) . then (| | self . 1) } }
/* FP:keys.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0044
/* FP:keys.rs-0088 */ impl Key for (DefId , SimplifiedType) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 0 . default_span (tcx) } }
/* FP:keys.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0045
/* FP:keys.rs-0090 */ impl Key for (DefId , ty :: SizedTraitKind) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 0 . default_span (tcx) } }
/* FP:keys.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0046
/* FP:keys.rs-0092 */ impl < 'tcx > Key for GenericArgsRef < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0047
/* FP:keys.rs-0094 */ impl < 'tcx > Key for (DefId , GenericArgsRef < 'tcx >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 0 . default_span (tcx) } }
/* FP:keys.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0048
/* FP:keys.rs-0096 */ impl < 'tcx > Key for (ty :: UnevaluatedConst < 'tcx > , ty :: UnevaluatedConst < 'tcx >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { (self . 0) . def . default_span (tcx) } }
/* FP:keys.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0049
/* FP:keys.rs-0098 */ impl < 'tcx > Key for (LocalDefId , DefId , GenericArgsRef < 'tcx >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 0 . default_span (tcx) } }
/* FP:keys.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0050
/* FP:keys.rs-0100 */ impl < 'tcx > Key for (ty :: ParamEnv < 'tcx > , ty :: TraitRef < 'tcx >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (self . 1 . def_id) } }
/* FP:keys.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0051
/* FP:keys.rs-0102 */ impl < 'tcx > Key for ty :: ParamEnvAnd < 'tcx , Ty < 'tcx > > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0052
/* FP:keys.rs-0104 */ impl < 'tcx > Key for ty :: TraitRef < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (self . def_id) } }
/* FP:keys.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0053
/* FP:keys.rs-0106 */ impl < 'tcx > Key for ty :: PolyTraitRef < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (self . def_id ()) } }
/* FP:keys.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0054
/* FP:keys.rs-0108 */ impl < 'tcx > Key for ty :: PolyExistentialTraitRef < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (self . def_id ()) } }
/* FP:keys.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0055
/* FP:keys.rs-0110 */ impl < 'tcx > Key for (ty :: PolyTraitRef < 'tcx > , ty :: PolyTraitRef < 'tcx >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . def_span (self . 0 . def_id ()) } }
/* FP:keys.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0056
/* FP:keys.rs-0112 */ impl < 'tcx > Key for GenericArg < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0057
/* FP:keys.rs-0114 */ impl < 'tcx > Key for ty :: Const < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0058
/* FP:keys.rs-0116 */ impl < 'tcx > Key for Ty < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } fn def_id_for_ty_in_cycle (& self) -> Option < DefId > { match * self . kind () { ty :: Adt (adt , _) => Some (adt . did ()) , ty :: Coroutine (def_id , ..) => Some (def_id) , _ => None , } } }
/* FP:keys.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0059
/* FP:keys.rs-0118 */ impl < 'tcx > Key for TyAndLayout < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0060
/* FP:keys.rs-0120 */ impl < 'tcx > Key for (Ty < 'tcx > , Ty < 'tcx >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0061
/* FP:keys.rs-0122 */ impl < 'tcx > Key for ty :: Clauses < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0062
/* FP:keys.rs-0124 */ impl < 'tcx > Key for ty :: ParamEnv < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0063
/* FP:keys.rs-0126 */ impl < 'tcx , T : Key > Key for ty :: PseudoCanonicalInput < 'tcx , T > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . value . default_span (tcx) } fn def_id_for_ty_in_cycle (& self) -> Option < DefId > { self . value . def_id_for_ty_in_cycle () } }
/* FP:keys.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0064
/* FP:keys.rs-0128 */ impl Key for Symbol { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0065
/* FP:keys.rs-0130 */ impl Key for Option < Symbol > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0066
/* FP:keys.rs-0132 */ impl < 'tcx > Key for & 'tcx OsStr { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0067
/* FP:keys.rs-0134 */ # [doc = " Canonical query goals correspond to abstract trait operations that"] # [doc = " are not tied to any crate in particular."] impl < 'tcx , T : Clone > Key for CanonicalQueryInput < 'tcx , T > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0068
/* FP:keys.rs-0136 */ impl < 'tcx , T : Clone > Key for (CanonicalQueryInput < 'tcx , T > , bool) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0069
/* FP:keys.rs-0138 */ impl Key for (Symbol , u32 , u32) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0070
/* FP:keys.rs-0140 */ impl < 'tcx > Key for (DefId , Ty < 'tcx > , GenericArgsRef < 'tcx > , ty :: ParamEnv < 'tcx >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0071
/* FP:keys.rs-0142 */ impl < 'tcx > Key for (Ty < 'tcx > , crate :: rustc_abi :: VariantIdx) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0072
/* FP:keys.rs-0144 */ impl < 'tcx > Key for (ty :: Predicate < 'tcx > , traits :: WellFormedLoc) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _tcx : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0073
/* FP:keys.rs-0146 */ impl < 'tcx > Key for (ty :: PolyFnSig < 'tcx > , & 'tcx ty :: List < Ty < 'tcx > >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0074
/* FP:keys.rs-0148 */ impl < 'tcx > Key for (ty :: Instance < 'tcx > , & 'tcx ty :: List < Ty < 'tcx > >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 0 . default_span (tcx) } }
/* FP:keys.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0075
/* FP:keys.rs-0150 */ impl < 'tcx > Key for ty :: Value < 'tcx > { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } }
/* FP:keys.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0076
/* FP:keys.rs-0152 */ impl Key for HirId { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . hir_span (* self) } # [inline (always)] fn key_as_def_id (& self) -> Option < DefId > { None } }
/* FP:keys.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0077
/* FP:keys.rs-0154 */ impl Key for (LocalDefId , HirId) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { tcx . hir_span (self . 1) } # [inline (always)] fn key_as_def_id (& self) -> Option < DefId > { Some (self . 0 . into ()) } }
/* FP:keys.rs-0155 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0078
/* FP:keys.rs-0156 */ impl < 'tcx > Key for (ValidityRequirement , ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , _ : TyCtxt < '_ >) -> Span { DUMMY_SP } fn def_id_for_ty_in_cycle (& self) -> Option < DefId > { match self . 1 . value . kind () { ty :: Adt (adt , _) => Some (adt . did ()) , _ => None , } } }
/* FP:keys.rs-0157 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_query_keys_IMPL_0079
/* FP:keys.rs-0158 */ impl < 'tcx > Key for (ty :: Instance < 'tcx > , CollectionMode) { type Cache < V > = DefaultCache < Self , V > ; fn default_span (& self , tcx : TyCtxt < '_ >) -> Span { self . 0 . default_span (tcx) } }