mkmod!{debug, { 
                getname!(debug);
                getsrc!(debug);
                getpath!(debug);
                get_deps!(debug);
                get_crates!(debug);
                mkinclude!(debug);
                 
            }}
mkmod!{dep_node, { 
                getname!(dep_node);
                getsrc!(dep_node);
                getpath!(dep_node);
                get_deps!(dep_node);
                get_crates!(dep_node);
                mkinclude!(dep_node);
                 
            }}
mkmod!{edges, { 
                getname!(edges);
                getsrc!(edges);
                getpath!(edges);
                get_deps!(edges);
                get_crates!(edges);
                mkinclude!(edges);
                 
            }}
mkmod!{graph, { 
                getname!(graph);
                getsrc!(graph);
                getpath!(graph);
                get_deps!(graph);
                get_crates!(graph);
                mkinclude!(graph);
                 
            }}
mkmod!{query, { 
                getname!(query);
                getsrc!(query);
                getpath!(query);
                get_deps!(query);
                get_crates!(query);
                mkinclude!(query);
                 
            }}
mkmod!{serialized, { 
                getname!(serialized);
                getsrc!(serialized);
                getpath!(serialized);
                get_deps!(serialized);
                get_crates!(serialized);
                mkinclude!(serialized);
                 
            }}
mkuse!{use std :: panic ;}
mkuse!{pub use dep_node :: { DepKind , DepKindStruct , DepNode , DepNodeParams , WorkProductId } ;}
mkuse!{pub (crate) use graph :: DepGraphData ;}
mkuse!{pub use graph :: { DepGraph , DepNodeIndex , TaskDepsRef , WorkProduct , WorkProductMap , hash_result } ;}
mkuse!{pub use query :: DepGraphQuery ;}
mkuse!{use rustc_data_structures :: profiling :: SelfProfilerRef ;}
mkuse!{use rustc_data_structures :: sync :: DynSync ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{pub use serialized :: { SerializedDepGraph , SerializedDepNodeIndex } ;}
mkuse!{use tracing :: instrument ;}
mkuse!{use self :: graph :: { MarkFrame , print_markframe_trace } ;}
mkuse!{use crate :: ich :: StableHashingContext ;}
mkitem!{mktrait!{pub trait DepContext : Copy { type Deps : Deps ; # [doc = " Create a hashing context for hashing new results."] fn with_stable_hashing_context < R > (self , f : impl FnOnce (StableHashingContext < '_ >) -> R) -> R ; # [doc = " Access the DepGraph."] fn dep_graph (& self) -> & DepGraph < Self :: Deps > ; # [doc = " Access the profiler."] fn profiler (& self) -> & SelfProfilerRef ; # [doc = " Access the compiler session."] fn sess (& self) -> & Session ; fn dep_kind_info (& self , dep_node : DepKind) -> & DepKindStruct < Self > ; # [inline (always)] fn fingerprint_style (self , kind : DepKind) -> FingerprintStyle { let data = self . dep_kind_info (kind) ; if data . is_anon { return FingerprintStyle :: Opaque ; } data . fingerprint_style } # [inline (always)] # [doc = " Return whether this kind always require evaluation."] fn is_eval_always (self , kind : DepKind) -> bool { self . dep_kind_info (kind) . is_eval_always } # [doc = " Try to force a dep node to execute and see if it's green."] # [doc = ""] # [doc = " Returns true if the query has actually been forced. It is valid that a query"] # [doc = " fails to be forced, e.g. when the query key cannot be reconstructed from the"] # [doc = " dep-node or when the query kind outright does not support it."] # [inline] # [instrument (skip (self , frame) , level = "debug")] fn try_force_from_dep_node (self , dep_node : DepNode , prev_index : SerializedDepNodeIndex , frame : Option < & MarkFrame < '_ > > ,) -> bool { let cb = self . dep_kind_info (dep_node . kind) ; if let Some (f) = cb . force_from_dep_node { match panic :: catch_unwind (panic :: AssertUnwindSafe (| | f (self , dep_node , prev_index))) { Err (value) => { if ! value . is :: < rustc_errors :: FatalErrorMarker > () { print_markframe_trace (self . dep_graph () , frame) ; } panic :: resume_unwind (value) } Ok (query_has_been_forced) => query_has_been_forced , } } else { false } } # [doc = " Load data from the on-disk cache."] fn try_load_from_on_disk_cache (self , dep_node : DepNode) { let cb = self . dep_kind_info (dep_node . kind) ; if let Some (f) = cb . try_load_from_on_disk_cache { f (self , dep_node) } } fn with_reduced_queries < T > (self , _ : impl FnOnce () -> T) -> T ; }}}
mkitem!{mktrait!{pub trait Deps : DynSync { # [doc = " Execute the operation with provided dependencies."] fn with_deps < OP , R > (deps : TaskDepsRef < '_ > , op : OP) -> R where OP : FnOnce () -> R ; # [doc = " Access dependencies from current implicit context."] fn read_deps < OP > (op : OP) where OP : for < 'a > FnOnce (TaskDepsRef < 'a >) ; fn name (& self , dep_kind : DepKind) -> & 'static str ; # [doc = " We use this for most things when incr. comp. is turned off."] const DEP_KIND_NULL : DepKind ; # [doc = " We use this to create a forever-red node."] const DEP_KIND_RED : DepKind ; # [doc = " We use this to create a side effect node."] const DEP_KIND_SIDE_EFFECT : DepKind ; # [doc = " We use this to create the anon node with zero dependencies."] const DEP_KIND_ANON_ZERO_DEPS : DepKind ; # [doc = " This is the highest value a `DepKind` can have. It's used during encoding to"] # [doc = " pack information into the unused bits."] const DEP_KIND_MAX : u16 ; }}}
mkitem!{mktrait!{pub trait HasDepContext : Copy { type Deps : self :: Deps ; type DepContext : self :: DepContext < Deps = Self :: Deps > ; fn dep_context (& self) -> & Self :: DepContext ; }}}
mkitem!{mkimpl!{impl < T : DepContext > HasDepContext for T { type Deps = T :: Deps ; type DepContext = Self ; fn dep_context (& self) -> & Self :: DepContext { self } }}}
mkitem!{mkimpl!{impl < T : HasDepContext , Q : Copy > HasDepContext for (T , Q) { type Deps = T :: Deps ; type DepContext = T :: DepContext ; fn dep_context (& self) -> & Self :: DepContext { self . 0 . dep_context () } }}}
mkitem!{mkenum!{# [doc = " Describes the contents of the fingerprint generated by a given query."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] pub enum FingerprintStyle { # [doc = " The fingerprint is actually a DefPathHash."] DefPathHash , # [doc = " The fingerprint is actually a HirId."] HirId , # [doc = " Query key was `()` or equivalent, so fingerprint is just zero."] Unit , # [doc = " Some opaque hash."] Opaque , }}}
mkitem!{mkimpl!{impl FingerprintStyle { # [inline] pub const fn reconstructible (self) -> bool { match self { FingerprintStyle :: DefPathHash | FingerprintStyle :: Unit | FingerprintStyle :: HirId => { true } FingerprintStyle :: Opaque => false , } } }}}