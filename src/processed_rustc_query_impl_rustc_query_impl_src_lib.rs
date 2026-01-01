/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0001
/* FP:lib.rs-0002 */ # [allow (internal_features)] # [doc (html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")] # [doc (rust_logo)] # [feature (min_specialization)] # [feature (rustc_attrs)] # [feature (rustdoc_internals)] use crate :: rustc_data_structures :: stable_hasher :: HashStable ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0002
/* FP:lib.rs-0004 */ use crate :: rustc_data_structures :: sync :: AtomicU64 ;
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0003
/* FP:lib.rs-0006 */ use crate :: rustc_complete :: arena :: Arena ;
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0004
/* FP:lib.rs-0008 */ use crate :: rustc_complete :: dep_graph :: { self , DepKind , DepKindStruct , DepNodeIndex } ;
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0005
/* FP:lib.rs-0010 */ use crate :: rustc_complete :: query :: erase :: { Erase , erase , restore } ;
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0006
/* FP:lib.rs-0012 */ use crate :: rustc_complete :: query :: on_disk_cache :: { CacheEncoder , EncodedDepNodeIndex , OnDiskCache } ;
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0007
/* FP:lib.rs-0014 */ use crate :: rustc_complete :: query :: plumbing :: { DynamicQuery , QuerySystem , QuerySystemFns } ;
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0008
/* FP:lib.rs-0016 */ use crate :: rustc_complete :: query :: { AsLocalKey , DynamicQueries , ExternProviders , Providers , QueryCaches , QueryEngine , QueryStates , queries , } ;
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0009
/* FP:lib.rs-0018 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0010
/* FP:lib.rs-0020 */ use rustc_query_system :: dep_graph :: SerializedDepNodeIndex ;
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0011
/* FP:lib.rs-0022 */ use rustc_query_system :: ich :: StableHashingContext ;
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0012
/* FP:lib.rs-0024 */ use rustc_query_system :: query :: { CycleError , HashResult , QueryCache , QueryConfig , QueryMap , QueryMode , QueryStackDeferred , QueryState , get_query_incr , get_query_non_incr , } ;
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0013
/* FP:lib.rs-0026 */ use rustc_query_system :: { HandleCycleError , Value } ;
/* FP:lib.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0014
/* FP:lib.rs-0028 */ use crate :: rustc_complete :: { ErrorGuaranteed , Span } ;
/* FP:lib.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0015
/* FP:lib.rs-0030 */ use crate :: plumbing :: { __rust_begin_short_backtrace , encode_all_query_results , try_mark_green } ;
/* FP:lib.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0016
/* FP:lib.rs-0032 */ use crate :: profiling_support :: QueryKeyStringCache ;
/* FP:lib.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_MOD_0017
/* FP:lib.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0018
/* FP:lib.rs-0036 */ pub use crate :: plumbing :: { QueryCtxt , query_key_hash_verify_all } ;
/* FP:lib.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_MOD_0019
/* FP:lib.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_USE_0020
/* FP:lib.rs-0040 */ pub use self :: profiling_support :: alloc_self_profile_query_strings ;
/* FP:lib.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_STRUCT_0021
/* FP:lib.rs-0042 */ struct DynamicConfig < 'tcx , C : QueryCache , const ANON : bool , const DEPTH_LIMIT : bool , const FEEDABLE : bool , > { dynamic : & 'tcx DynamicQuery < 'tcx , C > , }
/* FP:lib.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_IMPL_0022
/* FP:lib.rs-0044 */ impl < 'tcx , C : QueryCache , const ANON : bool , const DEPTH_LIMIT : bool , const FEEDABLE : bool > Copy for DynamicConfig < 'tcx , C , ANON , DEPTH_LIMIT , FEEDABLE > { }
/* FP:lib.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_IMPL_0023
/* FP:lib.rs-0046 */ impl < 'tcx , C : QueryCache , const ANON : bool , const DEPTH_LIMIT : bool , const FEEDABLE : bool > Clone for DynamicConfig < 'tcx , C , ANON , DEPTH_LIMIT , FEEDABLE > { fn clone (& self) -> Self { * self } }
/* FP:lib.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_IMPL_0024
/* FP:lib.rs-0048 */ impl < 'tcx , C : QueryCache , const ANON : bool , const DEPTH_LIMIT : bool , const FEEDABLE : bool > QueryConfig < QueryCtxt < 'tcx > > for DynamicConfig < 'tcx , C , ANON , DEPTH_LIMIT , FEEDABLE > where for < 'a > C :: Key : HashStable < StableHashingContext < 'a > > , { type Key = C :: Key ; type Value = C :: Value ; type Cache = C ; # [inline (always)] fn name (self) -> & 'static str { self . dynamic . name } # [inline (always)] fn cache_on_disk (self , tcx : TyCtxt < 'tcx > , key : & Self :: Key) -> bool { (self . dynamic . cache_on_disk) (tcx , key) } # [inline (always)] fn query_state < 'a > (self , qcx : QueryCtxt < 'tcx > ,) -> & 'a QueryState < Self :: Key , QueryStackDeferred < 'tcx > > where QueryCtxt < 'tcx > : 'a , { unsafe { & * (& qcx . tcx . query_system . states as * const QueryStates < 'tcx >) . byte_add (self . dynamic . query_state) . cast :: < QueryState < Self :: Key , QueryStackDeferred < 'tcx > > > () } } # [inline (always)] fn query_cache < 'a > (self , qcx : QueryCtxt < 'tcx >) -> & 'a Self :: Cache where 'tcx : 'a , { unsafe { & * (& qcx . tcx . query_system . caches as * const QueryCaches < 'tcx >) . byte_add (self . dynamic . query_cache) . cast :: < Self :: Cache > () } } # [inline (always)] fn execute_query (self , tcx : TyCtxt < 'tcx > , key : Self :: Key) -> Self :: Value { (self . dynamic . execute_query) (tcx , key) } # [inline (always)] fn compute (self , qcx : QueryCtxt < 'tcx > , key : Self :: Key) -> Self :: Value { (self . dynamic . compute) (qcx . tcx , key) } # [inline (always)] fn try_load_from_disk (self , qcx : QueryCtxt < 'tcx > , key : & Self :: Key , prev_index : SerializedDepNodeIndex , index : DepNodeIndex ,) -> Option < Self :: Value > { if self . dynamic . can_load_from_disk { (self . dynamic . try_load_from_disk) (qcx . tcx , key , prev_index , index) } else { None } } # [inline] fn loadable_from_disk (self , qcx : QueryCtxt < 'tcx > , key : & Self :: Key , index : SerializedDepNodeIndex ,) -> bool { (self . dynamic . loadable_from_disk) (qcx . tcx , key , index) } fn value_from_cycle_error (self , tcx : TyCtxt < 'tcx > , cycle_error : & CycleError , guar : ErrorGuaranteed ,) -> Self :: Value { (self . dynamic . value_from_cycle_error) (tcx , cycle_error , guar) } # [inline (always)] fn format_value (self) -> fn (& Self :: Value) -> String { self . dynamic . format_value } # [inline (always)] fn anon (self) -> bool { ANON } # [inline (always)] fn eval_always (self) -> bool { self . dynamic . eval_always } # [inline (always)] fn depth_limit (self) -> bool { DEPTH_LIMIT } # [inline (always)] fn feedable (self) -> bool { FEEDABLE } # [inline (always)] fn dep_kind (self) -> DepKind { self . dynamic . dep_kind } # [inline (always)] fn handle_cycle_error (self) -> HandleCycleError { self . dynamic . handle_cycle_error } # [inline (always)] fn hash_result (self) -> HashResult < Self :: Value > { self . dynamic . hash_result } }
/* FP:lib.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_TRAIT_0025
/* FP:lib.rs-0050 */ # [doc = " This is implemented per query. It allows restoring query values from their erased state"] # [doc = " and constructing a QueryConfig."] trait QueryConfigRestored < 'tcx > { type RestoredValue ; type Config : QueryConfig < QueryCtxt < 'tcx > > ; const NAME : & 'static & 'static str ; fn config (tcx : TyCtxt < 'tcx >) -> Self :: Config ; fn restore (value : < Self :: Config as QueryConfig < QueryCtxt < 'tcx > > > :: Value) -> Self :: RestoredValue ; }
/* FP:lib.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_FN_0026
/* FP:lib.rs-0052 */ pub fn query_system < 'a > (local_providers : Providers , extern_providers : ExternProviders , on_disk_cache : Option < OnDiskCache > , incremental : bool ,) -> QuerySystem < 'a > { QuerySystem { states : Default :: default () , arenas : Default :: default () , caches : Default :: default () , dynamic_queries : dynamic_queries () , on_disk_cache , fns : QuerySystemFns { engine : engine (incremental) , local_providers , extern_providers , encode_query_results : encode_all_query_results , try_mark_green , } , jobs : AtomicU64 :: new (1) , } }
/* FP:lib.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_MACRO_0027
/* FP:lib.rs-0054 */ crate :: rustc_middle :: rustc_with_all_queries ! { define_queries ! }
/* FP:lib.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_impl_src_lib_FN_0028
/* FP:lib.rs-0056 */ pub fn provide (providers : & mut crate :: rustc_middle :: util :: Providers) { providers . hooks . alloc_self_profile_query_strings = alloc_self_profile_query_strings ; providers . hooks . query_key_hash_verify_all = query_key_hash_verify_all ; }