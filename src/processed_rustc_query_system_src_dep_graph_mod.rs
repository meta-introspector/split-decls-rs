/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_MOD_0001
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_MOD_0002
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_MOD_0003
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_MOD_0004
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_MOD_0005
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_MOD_0006
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0007
/* FP:mod.rs-0014 */ use std :: panic ;
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0008
/* FP:mod.rs-0016 */ pub use dep_node :: { DepKind , DepKindStruct , DepNode , DepNodeParams , WorkProductId } ;
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0009
/* FP:mod.rs-0018 */ pub (crate) use graph :: DepGraphData ;
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0010
/* FP:mod.rs-0020 */ pub use graph :: { DepGraph , DepNodeIndex , TaskDepsRef , WorkProduct , WorkProductMap , hash_result } ;
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0011
/* FP:mod.rs-0022 */ pub use query :: DepGraphQuery ;
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0012
/* FP:mod.rs-0024 */ use crate :: rustc_data_structures :: profiling :: SelfProfilerRef ;
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0013
/* FP:mod.rs-0026 */ use crate :: rustc_data_structures :: sync :: DynSync ;
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0014
/* FP:mod.rs-0028 */ use crate :: rustc_complete :: Session ;
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0015
/* FP:mod.rs-0030 */ pub use serialized :: { SerializedDepGraph , SerializedDepNodeIndex } ;
/* FP:mod.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0016
/* FP:mod.rs-0032 */ use tracing :: instrument ;
/* FP:mod.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0017
/* FP:mod.rs-0034 */ use self :: graph :: { MarkFrame , print_markframe_trace } ;
/* FP:mod.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_USE_0018
/* FP:mod.rs-0036 */ use crate :: ich :: StableHashingContext ;
/* FP:mod.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_TRAIT_0019
/* FP:mod.rs-0038 */ pub trait DepContext : Copy { type Deps : Deps ; # [doc = " Create a hashing context for hashing new results."] fn with_stable_hashing_context < R > (self , f : impl FnOnce (StableHashingContext < '_ >) -> R) -> R ; # [doc = " Access the DepGraph."] fn dep_graph (& self) -> & DepGraph < Self :: Deps > ; # [doc = " Access the profiler."] fn profiler (& self) -> & SelfProfilerRef ; # [doc = " Access the compiler session."] fn sess (& self) -> & Session ; fn dep_kind_info (& self , dep_node : DepKind) -> & DepKindStruct < Self > ; # [inline (always)] fn fingerprint_style (self , kind : DepKind) -> FingerprintStyle { let data = self . dep_kind_info (kind) ; if data . is_anon { return FingerprintStyle :: Opaque ; } data . fingerprint_style } # [inline (always)] # [doc = " Return whether this kind always require evaluation."] fn is_eval_always (self , kind : DepKind) -> bool { self . dep_kind_info (kind) . is_eval_always } # [doc = " Try to force a dep node to execute and see if it's green."] # [doc = ""] # [doc = " Returns true if the query has actually been forced. It is valid that a query"] # [doc = " fails to be forced, e.g. when the query key cannot be reconstructed from the"] # [doc = " dep-node or when the query kind outright does not support it."] # [inline] # [instrument (skip (self , frame) , level = "debug")] fn try_force_from_dep_node (self , dep_node : DepNode , prev_index : SerializedDepNodeIndex , frame : Option < & MarkFrame < '_ > > ,) -> bool { let cb = self . dep_kind_info (dep_node . kind) ; if let Some (f) = cb . force_from_dep_node { match panic :: catch_unwind (panic :: AssertUnwindSafe (| | f (self , dep_node , prev_index))) { Err (value) => { if ! value . is :: < crate :: rustc_errors :: FatalErrorMarker > () { print_markframe_trace (self . dep_graph () , frame) ; } panic :: resume_unwind (value) } Ok (query_has_been_forced) => query_has_been_forced , } } else { false } } # [doc = " Load data from the on-disk cache."] fn try_load_from_on_disk_cache (self , dep_node : DepNode) { let cb = self . dep_kind_info (dep_node . kind) ; if let Some (f) = cb . try_load_from_on_disk_cache { f (self , dep_node) } } fn with_reduced_queries < T > (self , _ : impl FnOnce () -> T) -> T ; }
/* FP:mod.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_TRAIT_0020
/* FP:mod.rs-0040 */ pub trait Deps : DynSync { # [doc = " Execute the operation with provided dependencies."] fn with_deps < OP , R > (deps : TaskDepsRef < '_ > , op : OP) -> R where OP : FnOnce () -> R ; # [doc = " Access dependencies from current implicit context."] fn read_deps < OP > (op : OP) where OP : for < 'a > FnOnce (TaskDepsRef < 'a >) ; fn name (& self , dep_kind : DepKind) -> & 'static str ; # [doc = " We use this for most things when incr. comp. is turned off."] const DEP_KIND_NULL : DepKind ; # [doc = " We use this to create a forever-red node."] const DEP_KIND_RED : DepKind ; # [doc = " We use this to create a side effect node."] const DEP_KIND_SIDE_EFFECT : DepKind ; # [doc = " We use this to create the anon node with zero dependencies."] const DEP_KIND_ANON_ZERO_DEPS : DepKind ; # [doc = " This is the highest value a `DepKind` can have. It's used during encoding to"] # [doc = " pack information into the unused bits."] const DEP_KIND_MAX : u16 ; }
/* FP:mod.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_TRAIT_0021
/* FP:mod.rs-0042 */ pub trait HasDepContext : Copy { type Deps : self :: Deps ; type DepContext : self :: DepContext < Deps = Self :: Deps > ; fn dep_context (& self) -> & Self :: DepContext ; }
/* FP:mod.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_IMPL_0022
/* FP:mod.rs-0044 */ impl < T : DepContext > HasDepContext for T { type Deps = T :: Deps ; type DepContext = Self ; fn dep_context (& self) -> & Self :: DepContext { self } }
/* FP:mod.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_IMPL_0023
/* FP:mod.rs-0046 */ impl < T : HasDepContext , Q : Copy > HasDepContext for (T , Q) { type Deps = T :: Deps ; type DepContext = T :: DepContext ; fn dep_context (& self) -> & Self :: DepContext { self . 0 . dep_context () } }
/* FP:mod.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_ENUM_0024
/* FP:mod.rs-0048 */ # [doc = " Describes the contents of the fingerprint generated by a given query."] # [derive (Debug , PartialEq , Eq , Copy , Clone)] pub enum FingerprintStyle { # [doc = " The fingerprint is actually a DefPathHash."] DefPathHash , # [doc = " The fingerprint is actually a HirId."] HirId , # [doc = " Query key was `()` or equivalent, so fingerprint is just zero."] Unit , # [doc = " Some opaque hash."] Opaque , }
/* FP:mod.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_query_system_src_dep_graph_mod_IMPL_0025
/* FP:mod.rs-0050 */ impl FingerprintStyle { # [inline] pub const fn reconstructible (self) -> bool { match self { FingerprintStyle :: DefPathHash | FingerprintStyle :: Unit | FingerprintStyle :: HirId => { true } FingerprintStyle :: Opaque => false , } } }