/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_USE_0001
/* FP:mod.rs-0002 */ use crate :: rustc_data_structures :: profiling :: SelfProfilerRef ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_USE_0002
/* FP:mod.rs-0004 */ use rustc_query_system :: ich :: StableHashingContext ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: rustc_complete :: Session ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_USE_0004
/* FP:mod.rs-0008 */ use crate :: ty :: print :: with_reduced_queries ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_USE_0005
/* FP:mod.rs-0010 */ use crate :: ty :: { self , TyCtxt } ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_MOD_0006
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_USE_0007
/* FP:mod.rs-0014 */ pub use dep_node :: { DepKind , DepNode , DepNodeExt , dep_kinds , label_strs } ;
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_USE_0008
/* FP:mod.rs-0016 */ pub (crate) use dep_node :: { make_compile_codegen_unit , make_compile_mono_item , make_metadata } ;
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_USE_0009
/* FP:mod.rs-0018 */ pub use rustc_query_system :: dep_graph :: debug :: { DepNodeFilter , EdgeFilter } ;
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_USE_0010
/* FP:mod.rs-0020 */ pub use rustc_query_system :: dep_graph :: { DepContext , DepGraphQuery , DepNodeIndex , Deps , SerializedDepGraph , SerializedDepNodeIndex , TaskDepsRef , WorkProduct , WorkProductId , WorkProductMap , hash_result , } ;
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_TYPE_0011
/* FP:mod.rs-0022 */ pub type DepGraph = rustc_query_system :: dep_graph :: DepGraph < DepsType > ;
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_TYPE_0012
/* FP:mod.rs-0024 */ pub type DepKindStruct < 'tcx > = rustc_query_system :: dep_graph :: DepKindStruct < TyCtxt < 'tcx > > ;
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_STRUCT_0013
/* FP:mod.rs-0026 */ # [derive (Clone)] pub struct DepsType { pub dep_names : Vec < & 'static str > , }
/* FP:mod.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_IMPL_0014
/* FP:mod.rs-0028 */ impl Deps for DepsType { fn with_deps < OP , R > (task_deps : TaskDepsRef < '_ > , op : OP) -> R where OP : FnOnce () -> R , { ty :: tls :: with_context (| icx | { let icx = ty :: tls :: ImplicitCtxt { task_deps , .. icx . clone () } ; ty :: tls :: enter_context (& icx , op) }) } fn read_deps < OP > (op : OP) where OP : for < 'a > FnOnce (TaskDepsRef < 'a >) , { ty :: tls :: with_context_opt (| icx | { let Some (icx) = icx else { return } ; op (icx . task_deps) }) } fn name (& self , dep_kind : DepKind) -> & 'static str { self . dep_names [dep_kind . as_usize ()] } const DEP_KIND_NULL : DepKind = dep_kinds :: Null ; const DEP_KIND_RED : DepKind = dep_kinds :: Red ; const DEP_KIND_SIDE_EFFECT : DepKind = dep_kinds :: SideEffect ; const DEP_KIND_ANON_ZERO_DEPS : DepKind = dep_kinds :: AnonZeroDeps ; const DEP_KIND_MAX : u16 = dep_node :: DEP_KIND_VARIANTS - 1 ; }
/* FP:mod.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_dep_graph_mod_IMPL_0015
/* FP:mod.rs-0030 */ impl < 'tcx > DepContext for TyCtxt < 'tcx > { type Deps = DepsType ; # [inline] fn with_stable_hashing_context < R > (self , f : impl FnOnce (StableHashingContext < '_ >) -> R) -> R { TyCtxt :: with_stable_hashing_context (self , f) } # [inline] fn dep_graph (& self) -> & DepGraph { & self . dep_graph } # [inline (always)] fn profiler (& self) -> & SelfProfilerRef { & self . prof } # [inline (always)] fn sess (& self) -> & Session { self . sess } # [inline] fn dep_kind_info (& self , dk : DepKind) -> & DepKindStruct < 'tcx > { & self . query_kinds [dk . as_usize ()] } fn with_reduced_queries < T > (self , f : impl FnOnce () -> T) -> T { with_reduced_queries ! (f ()) } }