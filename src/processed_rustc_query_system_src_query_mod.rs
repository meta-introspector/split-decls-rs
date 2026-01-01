/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_MOD_0001
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0002
/* FP:mod.rs-0004 */ use std :: fmt :: Debug ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0003
/* FP:mod.rs-0006 */ use std :: marker :: PhantomData ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0004
/* FP:mod.rs-0008 */ use std :: mem :: transmute ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0005
/* FP:mod.rs-0010 */ use std :: sync :: Arc ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0006
/* FP:mod.rs-0012 */ pub use self :: plumbing :: * ;
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_MOD_0007
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0008
/* FP:mod.rs-0016 */ pub use self :: job :: { QueryInfo , QueryJob , QueryJobId , QueryJobInfo , QueryMap , break_query_cycles , print_query_stack , report_cycle , } ;
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_MOD_0009
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0010
/* FP:mod.rs-0020 */ pub use self :: caches :: { DefIdCache , DefaultCache , QueryCache , SingleCache , VecCache } ;
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_MOD_0011
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0012
/* FP:mod.rs-0024 */ use crate :: rustc_data_structures :: jobserver :: Proxy ;
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0013
/* FP:mod.rs-0026 */ use crate :: rustc_data_structures :: sync :: { DynSend , DynSync } ;
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0014
/* FP:mod.rs-0028 */ use crate :: rustc_complete :: DiagInner ;
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0015
/* FP:mod.rs-0030 */ use rustc_hashes :: Hash64 ;
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0016
/* FP:mod.rs-0032 */ use crate :: rustc_complete :: def :: DefKind ;
/* FP:mod.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0017
/* FP:mod.rs-0034 */ use rustc_macros :: { Decodable , Encodable } ;
/* FP:mod.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0018
/* FP:mod.rs-0036 */ use crate :: rustc_complete :: Span ;
/* FP:mod.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0019
/* FP:mod.rs-0038 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:mod.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0020
/* FP:mod.rs-0040 */ pub use self :: config :: { HashResult , QueryConfig } ;
/* FP:mod.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_USE_0021
/* FP:mod.rs-0042 */ use crate :: dep_graph :: { DepKind , DepNodeIndex , HasDepContext , SerializedDepNodeIndex } ;
/* FP:mod.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_STRUCT_0022
/* FP:mod.rs-0044 */ # [doc = " Description of a frame in the query stack."] # [doc = ""] # [doc = " This is mostly used in case of cycles for error reporting."] # [derive (Clone , Debug)] pub struct QueryStackFrame < I > { # [doc = " This field initially stores a `QueryStackDeferred` during collection,"] # [doc = " but can later be changed to `QueryStackFrameExtra` containing concrete information"] # [doc = " by calling `lift`. This is done so that collecting query does not need to invoke"] # [doc = " queries, instead `lift` will call queries in a more appropriate location."] pub info : I , pub dep_kind : DepKind , # [doc = " This hash is used to deterministically pick"] # [doc = " a query to remove cycles in the parallel compiler."] hash : Hash64 , pub def_id : Option < DefId > , # [doc = " A def-id that is extracted from a `Ty` in a query key"] pub def_id_for_ty_in_cycle : Option < DefId > , }
/* FP:mod.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_IMPL_0023
/* FP:mod.rs-0046 */ impl < I > QueryStackFrame < I > { # [inline] pub fn new (info : I , dep_kind : DepKind , hash : impl FnOnce () -> Hash64 , def_id : Option < DefId > , def_id_for_ty_in_cycle : Option < DefId > ,) -> Self { Self { info , def_id , dep_kind , hash : hash () , def_id_for_ty_in_cycle } } fn lift < Qcx : QueryContext < QueryInfo = I > > (& self , qcx : Qcx ,) -> QueryStackFrame < QueryStackFrameExtra > { QueryStackFrame { info : qcx . lift_query_info (& self . info) , dep_kind : self . dep_kind , hash : self . hash , def_id : self . def_id , def_id_for_ty_in_cycle : self . def_id_for_ty_in_cycle , } } }
/* FP:mod.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_STRUCT_0024
/* FP:mod.rs-0048 */ # [derive (Clone , Debug)] pub struct QueryStackFrameExtra { pub description : String , span : Option < Span > , pub def_kind : Option < DefKind > , }
/* FP:mod.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_IMPL_0025
/* FP:mod.rs-0050 */ impl QueryStackFrameExtra { # [inline] pub fn new (description : String , span : Option < Span > , def_kind : Option < DefKind >) -> Self { Self { description , span , def_kind } } # [inline] pub fn default_span (& self , span : Span) -> Span { if ! span . is_dummy () { return span ; } self . span . unwrap_or (span) } }
/* FP:mod.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_STRUCT_0026
/* FP:mod.rs-0052 */ # [doc = " Track a 'side effect' for a particular query."] # [doc = " This is used to hold a closure which can create `QueryStackFrameExtra`."] # [derive (Clone)] pub struct QueryStackDeferred < 'tcx > { _dummy : PhantomData < & 'tcx () > , extract : Arc < dyn Fn () -> QueryStackFrameExtra + DynSync + DynSend > , }
/* FP:mod.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_IMPL_0027
/* FP:mod.rs-0054 */ impl < 'tcx > QueryStackDeferred < 'tcx > { pub fn new < C : Copy + DynSync + DynSend + 'tcx > (context : C , extract : fn (C) -> QueryStackFrameExtra ,) -> Self { let extract : Arc < dyn Fn () -> QueryStackFrameExtra + DynSync + DynSend + 'tcx > = Arc :: new (move | | extract (context)) ; Self { _dummy : PhantomData , extract : unsafe { transmute (extract) } } } pub fn extract (& self) -> QueryStackFrameExtra { (self . extract) () } }
/* FP:mod.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_IMPL_0028
/* FP:mod.rs-0056 */ impl < 'tcx > Debug for QueryStackDeferred < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str ("QueryStackDeferred") } }
/* FP:mod.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_ENUM_0029
/* FP:mod.rs-0058 */ # [doc = " Tracks 'side effects' for a particular query."] # [doc = " This struct is saved to disk along with the query result,"] # [doc = " and loaded from disk if we mark the query as green."] # [doc = " This allows us to 'replay' changes to global state"] # [doc = " that would otherwise only occur if we actually"] # [doc = " executed the query method."] # [doc = ""] # [doc = " Each side effect gets an unique dep node index which is added"] # [doc = " as a dependency of the query which had the effect."] # [derive (Debug , Encodable , Decodable)] pub enum QuerySideEffect { # [doc = " Stores a diagnostic emitted during query execution."] # [doc = " This diagnostic will be re-emitted if we mark"] # [doc = " the query as green, as that query will have the side"] # [doc = " effect dep node as a dependency."] Diagnostic (DiagInner) , }
/* FP:mod.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_query_mod_TRAIT_0030
/* FP:mod.rs-0060 */ pub trait QueryContext : HasDepContext { type QueryInfo : Clone ; # [doc = " Gets a jobserver reference which is used to release then acquire"] # [doc = " a token while waiting on a query."] fn jobserver_proxy (& self) -> & Proxy ; fn next_job_id (self) -> QueryJobId ; # [doc = " Get the query information from the TLS context."] fn current_query_job (self) -> Option < QueryJobId > ; fn collect_active_jobs (self) -> Result < QueryMap < Self :: QueryInfo > , QueryMap < Self :: QueryInfo > > ; fn lift_query_info (self , info : & Self :: QueryInfo) -> QueryStackFrameExtra ; # [doc = " Load a side effect associated to the node in the previous session."] fn load_side_effect (self , prev_dep_node_index : SerializedDepNodeIndex ,) -> Option < QuerySideEffect > ; # [doc = " Register a side effect for the given node, for use in next session."] fn store_side_effect (self , dep_node_index : DepNodeIndex , side_effect : QuerySideEffect) ; # [doc = " Executes a job by changing the `ImplicitCtxt` to point to the"] # [doc = " new query job while it executes."] fn start_query < R > (self , token : QueryJobId , depth_limit : bool , compute : impl FnOnce () -> R) -> R ; fn depth_limit_error (self , job : QueryJobId) ; }