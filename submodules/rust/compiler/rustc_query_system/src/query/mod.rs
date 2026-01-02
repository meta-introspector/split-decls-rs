mkmod!{plumbing, { 
                getname!(plumbing);
                getsrc!(plumbing);
                getpath!(plumbing);
                get_deps!(plumbing);
                get_crates!(plumbing);
                mkinclude!(plumbing);
                 
            }}
mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: marker :: PhantomData ;}
mkuse!{use std :: mem :: transmute ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{pub use self :: plumbing :: * ;}
mkmod!{job, { 
                getname!(job);
                getsrc!(job);
                getpath!(job);
                get_deps!(job);
                get_crates!(job);
                mkinclude!(job);
                 
            }}
mkuse!{pub use self :: job :: { QueryInfo , QueryJob , QueryJobId , QueryJobInfo , QueryMap , break_query_cycles , print_query_stack , report_cycle , } ;}
mkmod!{caches, { 
                getname!(caches);
                getsrc!(caches);
                getpath!(caches);
                get_deps!(caches);
                get_crates!(caches);
                mkinclude!(caches);
                 
            }}
mkuse!{pub use self :: caches :: { DefIdCache , DefaultCache , QueryCache , SingleCache , VecCache } ;}
mkmod!{config, { 
                getname!(config);
                getsrc!(config);
                getpath!(config);
                get_deps!(config);
                get_crates!(config);
                mkinclude!(config);
                 
            }}
mkuse!{use rustc_data_structures :: jobserver :: Proxy ;}
mkuse!{use rustc_data_structures :: sync :: { DynSend , DynSync } ;}
mkuse!{use rustc_errors :: DiagInner ;}
mkuse!{use rustc_hashes :: Hash64 ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_macros :: { Decodable , Encodable } ;}
mkuse!{use rustc_span :: Span ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{pub use self :: config :: { HashResult , QueryConfig } ;}
mkuse!{use crate :: dep_graph :: { DepKind , DepNodeIndex , HasDepContext , SerializedDepNodeIndex } ;}
mkitem!{mkstruct!{# [doc = " Description of a frame in the query stack."] # [doc = ""] # [doc = " This is mostly used in case of cycles for error reporting."] # [derive (Clone , Debug)] pub struct QueryStackFrame < I > { # [doc = " This field initially stores a `QueryStackDeferred` during collection,"] # [doc = " but can later be changed to `QueryStackFrameExtra` containing concrete information"] # [doc = " by calling `lift`. This is done so that collecting query does not need to invoke"] # [doc = " queries, instead `lift` will call queries in a more appropriate location."] pub info : I , pub dep_kind : DepKind , # [doc = " This hash is used to deterministically pick"] # [doc = " a query to remove cycles in the parallel compiler."] hash : Hash64 , pub def_id : Option < DefId > , # [doc = " A def-id that is extracted from a `Ty` in a query key"] pub def_id_for_ty_in_cycle : Option < DefId > , }}}
mkitem!{mkimpl!{impl < I > QueryStackFrame < I > { # [inline] pub fn new (info : I , dep_kind : DepKind , hash : impl FnOnce () -> Hash64 , def_id : Option < DefId > , def_id_for_ty_in_cycle : Option < DefId > ,) -> Self { Self { info , def_id , dep_kind , hash : hash () , def_id_for_ty_in_cycle } } fn lift < Qcx : QueryContext < QueryInfo = I > > (& self , qcx : Qcx ,) -> QueryStackFrame < QueryStackFrameExtra > { QueryStackFrame { info : qcx . lift_query_info (& self . info) , dep_kind : self . dep_kind , hash : self . hash , def_id : self . def_id , def_id_for_ty_in_cycle : self . def_id_for_ty_in_cycle , } } }}}
mkitem!{mkstruct!{# [derive (Clone , Debug)] pub struct QueryStackFrameExtra { pub description : String , span : Option < Span > , pub def_kind : Option < DefKind > , }}}
mkitem!{mkimpl!{impl QueryStackFrameExtra { # [inline] pub fn new (description : String , span : Option < Span > , def_kind : Option < DefKind >) -> Self { Self { description , span , def_kind } } # [inline] pub fn default_span (& self , span : Span) -> Span { if ! span . is_dummy () { return span ; } self . span . unwrap_or (span) } }}}
mkitem!{mkstruct!{# [doc = " Track a 'side effect' for a particular query."] # [doc = " This is used to hold a closure which can create `QueryStackFrameExtra`."] # [derive (Clone)] pub struct QueryStackDeferred < 'tcx > { _dummy : PhantomData < & 'tcx () > , extract : Arc < dyn Fn () -> QueryStackFrameExtra + DynSync + DynSend > , }}}
mkitem!{mkimpl!{impl < 'tcx > QueryStackDeferred < 'tcx > { pub fn new < C : Copy + DynSync + DynSend + 'tcx > (context : C , extract : fn (C) -> QueryStackFrameExtra ,) -> Self { let extract : Arc < dyn Fn () -> QueryStackFrameExtra + DynSync + DynSend + 'tcx > = Arc :: new (move | | extract (context)) ; Self { _dummy : PhantomData , extract : unsafe { transmute (extract) } } } pub fn extract (& self) -> QueryStackFrameExtra { (self . extract) () } }}}
mkitem!{mkimpl!{impl < 'tcx > Debug for QueryStackDeferred < 'tcx > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . write_str ("QueryStackDeferred") } }}}
mkitem!{mkenum!{# [doc = " Tracks 'side effects' for a particular query."] # [doc = " This struct is saved to disk along with the query result,"] # [doc = " and loaded from disk if we mark the query as green."] # [doc = " This allows us to 'replay' changes to global state"] # [doc = " that would otherwise only occur if we actually"] # [doc = " executed the query method."] # [doc = ""] # [doc = " Each side effect gets an unique dep node index which is added"] # [doc = " as a dependency of the query which had the effect."] # [derive (Debug , Encodable , Decodable)] pub enum QuerySideEffect { # [doc = " Stores a diagnostic emitted during query execution."] # [doc = " This diagnostic will be re-emitted if we mark"] # [doc = " the query as green, as that query will have the side"] # [doc = " effect dep node as a dependency."] Diagnostic (DiagInner) , }}}
mkitem!{mktrait!{pub trait QueryContext : HasDepContext { type QueryInfo : Clone ; # [doc = " Gets a jobserver reference which is used to release then acquire"] # [doc = " a token while waiting on a query."] fn jobserver_proxy (& self) -> & Proxy ; fn next_job_id (self) -> QueryJobId ; # [doc = " Get the query information from the TLS context."] fn current_query_job (self) -> Option < QueryJobId > ; fn collect_active_jobs (self) -> Result < QueryMap < Self :: QueryInfo > , QueryMap < Self :: QueryInfo > > ; fn lift_query_info (self , info : & Self :: QueryInfo) -> QueryStackFrameExtra ; # [doc = " Load a side effect associated to the node in the previous session."] fn load_side_effect (self , prev_dep_node_index : SerializedDepNodeIndex ,) -> Option < QuerySideEffect > ; # [doc = " Register a side effect for the given node, for use in next session."] fn store_side_effect (self , dep_node_index : DepNodeIndex , side_effect : QuerySideEffect) ; # [doc = " Executes a job by changing the `ImplicitCtxt` to point to the"] # [doc = " new query job while it executes."] fn start_query < R > (self , token : QueryJobId , depth_limit : bool , compute : impl FnOnce () -> R) -> R ; fn depth_limit_error (self , job : QueryJobId) ; }}}