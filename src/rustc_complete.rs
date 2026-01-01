// Auto-generated complete rustc includes from symbol_map.json
// All rustc crates and submodules in dependency order

pub mod rustc_infer { pub use crate::*; }
pub mod rustc_trait_selection { pub use crate::*; }

pub mod ty {
    pub struct Ty<T>(pub T);
    pub struct TyCtxt<T>(pub T);
    pub struct TypeAndMut<T> { pub ty: T, pub mutbl: bool }
    pub struct Region;
    pub struct Predicate;
    pub struct TyKind;
    pub struct GenericArg;
    pub struct GenericArgs;
    pub struct ParamTy;
    pub struct EarlyBinder<T>(pub T);
    pub struct Binder<T>(pub T);
    pub struct TraitRef;
    pub struct PolyTraitRef;
    pub struct ExistentialTraitRef;
    pub struct TypeFoldable;
    pub struct TypeVisitable;
    pub mod layout {
        pub struct Layout;
        pub struct LayoutError;
        pub struct TyAndLayout<T> { pub ty: T, pub layout: Layout }
    }
}

pub mod def_id {
    pub struct DefId;
    pub struct LocalDefId;
    pub struct DefIndex;
    pub struct CrateNum;
    pub struct DefPathHash;
}

pub mod def {
    pub struct Def;
    pub struct DefKind;
}

pub mod mir {
    pub struct Body<T>(pub T);
    pub struct BasicBlock;
    pub struct Local;
    pub struct Place<T>(pub T);
    pub struct Operand<T>(pub T);
    pub struct Rvalue<T>(pub T);
}

pub fn bug() -> ! { panic!("bug") }
pub fn span_bug() -> ! { panic!("span_bug") }

pub struct Span;
pub struct Symbol;
pub struct Session;
pub struct ErrorGuaranteed;
pub struct LangItem;
pub const DUMMY_SP: Span = Span;

pub mod middle { pub struct Middle; }
pub mod query { pub struct Query; }
pub mod config { pub struct Config; }
pub mod attrs { pub struct Attrs; }
pub mod codes { pub struct Codes; }
pub mod source_map { pub struct SourceMap; }
pub mod sym { pub struct Sym; }
pub mod util { pub struct Util; }
pub mod token { pub struct Token; pub struct TokenKind; }
pub mod tokenstream { pub struct TokenStream; pub struct TokenTree; }

pub mod tests { pub struct Tests; }
pub mod undo_log { pub struct UndoLog; }
pub mod rustc_hash { pub struct RustcHash; }
pub mod fingerprint { pub struct Fingerprint; }
pub mod outline { pub struct Outline; }
pub mod errors { pub struct Errors; }
pub mod error_reporting { pub use crate::*; }
pub mod traits { pub use crate::*; }
pub mod stable_hasher { pub use crate::*; }

// Extern crate declarations moved to crate root
extern crate test;
extern crate self as rustc_span;
extern crate self as rustc_serialize;
extern crate self as rustc_middle;
extern crate self as rustc_type_ir;
extern crate self as rustc_hir;
extern crate smallvec;
extern crate tracing;
extern crate rustc_abi;
extern crate rustc_apfloat;
extern crate rustc_ast;
extern crate rustc_codegen_ssa;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_fluent_macro;
extern crate rustc_fs_util;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_macros;
extern crate rustc_session;
extern crate rustc_symbol_mangling;
extern crate rustc_target;
extern crate rustc_driver;
extern crate alloc;
extern crate rustc_incremental;
extern crate rustc_metadata;

// 1: rustc_macros (21 files)
pub mod included_rustc_macros {
}

// 2: rustc_data_structures (81 files)
pub mod included_rustc_data_structures {
    // Source: ../rust/compiler/rustc_data_structures/src/flock.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flock {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/parallel.rs
    pub mod rustc_data_structures_src_sync_parallel {
        include!("processed_rustc_data_structures_src_sync_parallel.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/union_find/tests.rs
    pub mod rustc_data_structures_src_union_find_tests {
        include!("processed_rustc_data_structures_src_union_find_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/vec.rs
    pub mod rustc_data_structures_src_sync_vec {
        include!("processed_rustc_data_structures_src_sync_vec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reversed.rs
    pub mod rustc_data_structures_src_graph_reversed {
        include!("processed_rustc_data_structures_src_graph_reversed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/owned_slice/tests.rs
    pub mod rustc_data_structures_src_owned_slice_tests {
        include!("processed_rustc_data_structures_src_owned_slice_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/intern/tests.rs
    pub mod rustc_data_structures_src_intern_tests {
        include!("processed_rustc_data_structures_src_intern_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/temp_dir.rs
    pub mod rustc_data_structures_rustc_data_structures_src_temp_dir {
        include!("processed_rustc_data_structures_rustc_data_structures_src_temp_dir.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flat_map_in_place.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flat_map_in_place {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flat_map_in_place.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/small_c_str/tests.rs
    pub mod rustc_data_structures_src_small_c_str_tests {
        include!("processed_rustc_data_structures_src_small_c_str_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/windows.rs
    pub mod rustc_data_structures_src_flock_windows {
        include!("processed_rustc_data_structures_src_flock_windows.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/profiling/tests.rs
    pub mod rustc_data_structures_src_profiling_tests {
        include!("processed_rustc_data_structures_src_profiling_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unsupported.rs
    pub mod rustc_data_structures_src_flock_unsupported {
        include!("processed_rustc_data_structures_src_flock_unsupported.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unhash.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unhash {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unhash.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/packed.rs
    pub mod rustc_data_structures_rustc_data_structures_src_packed {
        include!("processed_rustc_data_structures_rustc_data_structures_src_packed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/marker.rs
    pub mod rustc_data_structures_rustc_data_structures_src_marker {
        include!("processed_rustc_data_structures_rustc_data_structures_src_marker.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/vec_cache/tests.rs
    pub mod rustc_data_structures_src_vec_cache_tests {
        include!("processed_rustc_data_structures_src_vec_cache_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stack.rs
    pub mod rustc_data_structures_rustc_data_structures_src_stack {
        include!("processed_rustc_data_structures_rustc_data_structures_src_stack.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/frozen.rs
    pub mod rustc_data_structures_rustc_data_structures_src_frozen {
        include!("processed_rustc_data_structures_rustc_data_structures_src_frozen.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/set.rs
    pub mod rustc_data_structures_src_sso_set {
        include!("processed_rustc_data_structures_src_sso_set.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/binary_search_util/tests.rs
    pub mod rustc_data_structures_src_binary_search_util_tests {
        include!("processed_rustc_data_structures_src_binary_search_util_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thinvec.rs
    pub mod rustc_data_structures_rustc_data_structures_src_thinvec {
        include!("processed_rustc_data_structures_rustc_data_structures_src_thinvec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/vec_graph/tests.rs
    pub mod rustc_data_structures_graph_vec_graph_tests {
        include!("processed_rustc_data_structures_graph_vec_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sync {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sync.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/tests.rs
    pub mod rustc_data_structures_src_graph_tests {
        include!("processed_rustc_data_structures_src_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/worker_local.rs
    pub mod rustc_data_structures_src_sync_worker_local {
        include!("processed_rustc_data_structures_src_sync_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stable_hasher/tests.rs
    pub mod rustc_data_structures_src_stable_hasher_tests {
        include!("processed_rustc_data_structures_src_stable_hasher_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/work_queue.rs
    pub mod rustc_data_structures_rustc_data_structures_src_work_queue {
        include!("processed_rustc_data_structures_rustc_data_structures_src_work_queue.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fx.rs
    pub mod rustc_data_structures_rustc_data_structures_src_fx {
        include!("processed_rustc_data_structures_rustc_data_structures_src_fx.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/tagged_ptr/tests.rs
    pub mod rustc_data_structures_src_tagged_ptr_tests {
        include!("processed_rustc_data_structures_src_tagged_ptr_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/tests.rs
    pub mod rustc_data_structures_src_obligation_forest_tests {
        include!("processed_rustc_data_structures_src_obligation_forest_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/dominators/tests.rs
    pub mod rustc_data_structures_graph_dominators_tests {
        include!("processed_rustc_data_structures_graph_dominators_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/index_map.rs
    pub mod rustc_data_structures_src_sorted_map_index_map {
        include!("processed_rustc_data_structures_src_sorted_map_index_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unix.rs
    pub mod rustc_data_structures_src_flock_unix {
        include!("processed_rustc_data_structures_src_flock_unix.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/aligned.rs
    pub mod rustc_data_structures_rustc_data_structures_src_aligned {
        include!("processed_rustc_data_structures_rustc_data_structures_src_aligned.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/memmap.rs
    pub mod rustc_data_structures_rustc_data_structures_src_memmap {
        include!("processed_rustc_data_structures_rustc_data_structures_src_memmap.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/linux.rs
    pub mod rustc_data_structures_src_flock_linux {
        include!("processed_rustc_data_structures_src_flock_linux.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/snapshot_map/tests.rs
    pub mod rustc_data_structures_src_snapshot_map_tests {
        include!("processed_rustc_data_structures_src_snapshot_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/linked_graph/tests.rs
    pub mod rustc_data_structures_graph_linked_graph_tests {
        include!("processed_rustc_data_structures_graph_linked_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fingerprint/tests.rs
    pub mod rustc_data_structures_src_fingerprint_tests {
        include!("processed_rustc_data_structures_src_fingerprint_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unord.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unord {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unord.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/iterate/tests.rs
    pub mod rustc_data_structures_graph_iterate_tests {
        include!("processed_rustc_data_structures_graph_iterate_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/lib.rs
    pub mod rustc_data_structures_rustc_data_structures_src_lib {
        include!("processed_rustc_data_structures_rustc_data_structures_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/graphviz.rs
    pub mod rustc_data_structures_src_obligation_forest_graphviz {
        include!("processed_rustc_data_structures_src_obligation_forest_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/transitive_relation/tests.rs
    pub mod rustc_data_structures_src_transitive_relation_tests {
        include!("processed_rustc_data_structures_src_transitive_relation_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sharded.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sharded {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sharded.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thousands/tests.rs
    pub mod rustc_data_structures_src_thousands_tests {
        include!("processed_rustc_data_structures_src_thousands_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reference.rs
    pub mod rustc_data_structures_src_graph_reference {
        include!("processed_rustc_data_structures_src_graph_reference.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/base_n/tests.rs
    pub mod rustc_data_structures_src_base_n_tests {
        include!("processed_rustc_data_structures_src_base_n_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/mod.rs
    pub mod rustc_data_structures_src_sso_mod {
        include!("processed_rustc_data_structures_src_sso_mod.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/jobserver.rs
    pub mod rustc_data_structures_rustc_data_structures_src_jobserver {
        include!("processed_rustc_data_structures_rustc_data_structures_src_jobserver.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/freeze.rs
    pub mod rustc_data_structures_src_sync_freeze {
        include!("processed_rustc_data_structures_src_sync_freeze.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/steal.rs
    pub mod rustc_data_structures_rustc_data_structures_src_steal {
        include!("processed_rustc_data_structures_rustc_data_structures_src_steal.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/svh.rs
    pub mod rustc_data_structures_rustc_data_structures_src_svh {
        include!("processed_rustc_data_structures_rustc_data_structures_src_svh.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/scc/tests.rs
    pub mod rustc_data_structures_graph_scc_tests {
        include!("processed_rustc_data_structures_graph_scc_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/lock.rs
    pub mod rustc_data_structures_src_sync_lock {
        include!("processed_rustc_data_structures_src_sync_lock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/atomic_ref.rs
    pub mod rustc_data_structures_rustc_data_structures_src_atomic_ref {
        include!("processed_rustc_data_structures_rustc_data_structures_src_atomic_ref.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/map.rs
    pub mod rustc_data_structures_src_sso_map {
        include!("processed_rustc_data_structures_src_sso_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/tests.rs
    pub mod rustc_data_structures_src_sorted_map_tests {
        include!("processed_rustc_data_structures_src_sorted_map_tests.rs");
    }
}

// 3: rustc_index (9 files)
pub mod included_rustc_index {
    // Source: ../rust/compiler/rustc_index/src/lib.rs
    pub mod rustc_index_rustc_index_src_lib {
        include!("processed_rustc_index_rustc_index_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/slice.rs
    pub mod rustc_index_rustc_index_src_slice {
        include!("processed_rustc_index_rustc_index_src_slice.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/bit_set/tests.rs
    pub mod rustc_index_src_bit_set_tests {
        include!("processed_rustc_index_src_bit_set_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/interval/tests.rs
    pub mod rustc_index_src_interval_tests {
        include!("processed_rustc_index_src_interval_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/vec/tests.rs
    pub mod rustc_index_src_vec_tests {
        include!("processed_rustc_index_src_vec_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/idx.rs
    pub mod rustc_index_rustc_index_src_idx {
        include!("processed_rustc_index_rustc_index_src_idx.rs");
    }
}

// 4: rustc_span (17 files)
pub mod included_rustc_span {
    // Source: ../rust/compiler/rustc_span/src/profiling.rs
    pub mod rustc_span_rustc_span_src_profiling {
        include!("processed_rustc_span_rustc_span_src_profiling.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/symbol/tests.rs
    pub mod rustc_span_src_symbol_tests {
        include!("processed_rustc_span_src_symbol_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/caching_source_map_view.rs
    pub mod rustc_span_rustc_span_src_caching_source_map_view {
        include!("processed_rustc_span_rustc_span_src_caching_source_map_view.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/source_map/tests.rs
    pub mod rustc_span_src_source_map_tests {
        include!("processed_rustc_span_src_source_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edit_distance/tests.rs
    pub mod rustc_span_src_edit_distance_tests {
        include!("processed_rustc_span_src_edit_distance_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/analyze_source_file/tests.rs
    pub mod rustc_span_src_analyze_source_file_tests {
        include!("processed_rustc_span_src_analyze_source_file_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edition.rs
    pub mod rustc_span_rustc_span_src_edition {
        include!("processed_rustc_span_rustc_span_src_edition.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/hygiene.rs
    pub mod rustc_span_rustc_span_src_hygiene {
        include!("processed_rustc_span_rustc_span_src_hygiene.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/def_id.rs
    pub mod rustc_span_rustc_span_src_def_id {
        include!("processed_rustc_span_rustc_span_src_def_id.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/tests.rs
    pub mod rustc_span_rustc_span_src_tests {
        include!("processed_rustc_span_rustc_span_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/span_encoding.rs
    pub mod rustc_span_rustc_span_src_span_encoding {
        include!("processed_rustc_span_rustc_span_src_span_encoding.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/fatal_error.rs
    pub mod rustc_span_rustc_span_src_fatal_error {
        include!("processed_rustc_span_rustc_span_src_fatal_error.rs");
    }
}

// 5: rustc_fs_util (1 files)
pub mod included_rustc_fs_util {
    // Source: ../rust/compiler/rustc_fs_util/src/lib.rs
    pub mod rustc_fs_util_rustc_fs_util_src_lib {
        include!("processed_rustc_fs_util_rustc_fs_util_src_lib.rs");
    }
}

// 6: rustc_thread_pool (28 files)
pub mod included_rustc_thread_pool {
    // Source: ../rust/compiler/rustc_thread_pool/src/unwind.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_unwind {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_unwind.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/thread_pool/mod.rs
    pub mod rustc_thread_pool_src_thread_pool_mod {
        include!("processed_rustc_thread_pool_src_thread_pool_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/simple_panic.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_simple_panic {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_simple_panic.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/job.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_job {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_job.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/compile_fail/rc_return.rs
    pub mod rustc_thread_pool_src_compile_fail_rc_return {
        include!("processed_rustc_thread_pool_src_compile_fail_rc_return.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/scoped_threadpool.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_scoped_threadpool {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_scoped_threadpool.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/double_init_fail.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_double_init_fail {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_double_init_fail.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/init_zero_threads.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_init_zero_threads {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_init_zero_threads.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/spawn/tests.rs
    pub mod rustc_thread_pool_src_spawn_tests {
        include!("processed_rustc_thread_pool_src_spawn_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/join/tests.rs
    pub mod rustc_thread_pool_src_join_tests {
        include!("processed_rustc_thread_pool_src_join_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/registry.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_registry {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_registry.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/stack_overflow_crash.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_stack_overflow_crash {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_stack_overflow_crash.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/sleep/mod.rs
    pub mod rustc_thread_pool_src_sleep_mod {
        include!("processed_rustc_thread_pool_src_sleep_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/scope/tests.rs
    pub mod rustc_thread_pool_src_scope_tests {
        include!("processed_rustc_thread_pool_src_scope_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/worker_local.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_worker_local {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/scope_join.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_scope_join {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_scope_join.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/private.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_private {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_private.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/sleep/counters.rs
    pub mod rustc_thread_pool_src_sleep_counters {
        include!("processed_rustc_thread_pool_src_sleep_counters.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/tlv.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_tlv {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_tlv.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/lib.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_lib {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/latch.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_latch {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_latch.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/broadcast/mod.rs
    pub mod rustc_thread_pool_src_broadcast_mod {
        include!("processed_rustc_thread_pool_src_broadcast_mod.rs");
    }
}

// 7: rustc_query_impl (3 files)
pub mod included_rustc_query_impl {
    // Source: ../rust/compiler/rustc_query_impl/src/lib.rs
    pub mod rustc_query_impl_rustc_query_impl_src_lib {
        include!("processed_rustc_query_impl_rustc_query_impl_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_query_impl/src/profiling_support.rs
    pub mod rustc_query_impl_rustc_query_impl_src_profiling_support {
        include!("processed_rustc_query_impl_rustc_query_impl_src_profiling_support.rs");
    }
    // Source: ../rust/compiler/rustc_query_impl/src/plumbing.rs
    pub mod rustc_query_impl_rustc_query_impl_src_plumbing {
        include!("processed_rustc_query_impl_rustc_query_impl_src_plumbing.rs");
    }
}

// 8: rustc_hir_typeck (40 files)
pub mod included_rustc_hir_typeck {
    // Source: ../rust/compiler/rustc_hir_typeck/src/upvar.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_upvar {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_upvar.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/confirm.rs
    pub mod rustc_hir_typeck_src_method_confirm {
        include!("processed_rustc_hir_typeck_src_method_confirm.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs
    pub mod rustc_hir_typeck_src_fn_ctxt__impl {
        include!("processed_rustc_hir_typeck_src_fn_ctxt__impl.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/naked_functions.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_naked_functions {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_naked_functions.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/lib.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_lib {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/cast.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_cast {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_cast.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/coercion.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_coercion {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_coercion.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/suggest.rs
    pub mod rustc_hir_typeck_src_method_suggest {
        include!("processed_rustc_hir_typeck_src_method_suggest.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/demand.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_demand {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_demand.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/opaque_types.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_opaque_types {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/place_op.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_place_op {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_place_op.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/probe.rs
    pub mod rustc_hir_typeck_src_method_probe {
        include!("processed_rustc_hir_typeck_src_method_probe.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/rvalue_scopes.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_rvalue_scopes {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_rvalue_scopes.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/adjust_fulfillment_errors.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_adjust_fulfillment_errors {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_adjust_fulfillment_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/loops.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_loops {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_loops.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/_match.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src__match {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src__match.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fallback.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_fallback {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_fallback.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/expr.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_expr {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_expr.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/closure.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_closure {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_closure.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/intrinsicck.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_intrinsicck {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_intrinsicck.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/inline_asm.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_inline_asm {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_inline_asm.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/suggestions.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_suggestions {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_suggestions.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/mod.rs
    pub mod rustc_hir_typeck_src_method_mod {
        include!("processed_rustc_hir_typeck_src_method_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/errors.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_errors {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/expr_use_visitor.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_expr_use_visitor {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_expr_use_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/op.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_op {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_op.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/pat.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_pat {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_pat.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/arg_matrix.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_arg_matrix {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_arg_matrix.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/prelude_edition_lints.rs
    pub mod rustc_hir_typeck_src_method_prelude_edition_lints {
        include!("processed_rustc_hir_typeck_src_method_prelude_edition_lints.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/inspect_obligations.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_inspect_obligations {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_inspect_obligations.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/writeback.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_writeback {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_writeback.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_checks {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_checks.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/expectation.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_expectation {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_expectation.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/typeck_root_ctxt.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_typeck_root_ctxt {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_typeck_root_ctxt.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/diverges.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_diverges {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_diverges.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/callee.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_callee {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_callee.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/check.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_check {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/mod.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_mod {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/gather_locals.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_gather_locals {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_gather_locals.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/autoderef.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_autoderef {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_autoderef.rs");
    }
}

// 9: rustc_parse_format (2 files)
pub mod included_rustc_parse_format {
    // Source: ../rust/compiler/rustc_parse_format/src/tests.rs
    pub mod rustc_parse_format_rustc_parse_format_src_tests {
        include!("processed_rustc_parse_format_rustc_parse_format_src_tests.rs");
    }
}

// 10: rustc_monomorphize (9 files)
pub mod included_rustc_monomorphize {
    // Source: ../rust/compiler/rustc_monomorphize/src/errors.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_errors {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/partitioning.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_partitioning {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_partitioning.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/abi_check.rs
    pub mod rustc_monomorphize_src_mono_checks_abi_check {
        include!("processed_rustc_monomorphize_src_mono_checks_abi_check.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/mod.rs
    pub mod rustc_monomorphize_src_mono_checks_mod {
        include!("processed_rustc_monomorphize_src_mono_checks_mod.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/collector/autodiff.rs
    pub mod rustc_monomorphize_src_collector_autodiff {
        include!("processed_rustc_monomorphize_src_collector_autodiff.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/util.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_util {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_util.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/move_check.rs
    pub mod rustc_monomorphize_src_mono_checks_move_check {
        include!("processed_rustc_monomorphize_src_mono_checks_move_check.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/collector.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_collector {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_collector.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/lib.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_lib {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_lib.rs");
    }
}

// 11: rustc_fluent_macro (2 files)
pub mod included_rustc_fluent_macro {
    // Source: ../rust/compiler/rustc_fluent_macro/src/lib.rs
    pub mod rustc_fluent_macro_rustc_fluent_macro_src_lib {
        include!("processed_rustc_fluent_macro_rustc_fluent_macro_src_lib.rs");
    }
}

// 12: rustc_llvm (2 files)
pub mod included_rustc_llvm {
    // Source: ../rust/compiler/rustc_llvm/build.rs
    pub mod rustc_llvm_compiler_rustc_llvm_build {
        include!("processed_rustc_llvm_compiler_rustc_llvm_build.rs");
    }
    // Source: ../rust/compiler/rustc_llvm/src/lib.rs
    pub mod rustc_llvm_rustc_llvm_src_lib {
        include!("processed_rustc_llvm_rustc_llvm_src_lib.rs");
    }
}

// 13: rustc_mir_dataflow (24 files)
pub mod included_rustc_mir_dataflow {
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/fmt.rs
    pub mod rustc_mir_dataflow_src_framework_fmt {
        include!("processed_rustc_mir_dataflow_src_framework_fmt.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/borrowed_locals.rs
    pub mod rustc_mir_dataflow_src_impls_borrowed_locals {
        include!("processed_rustc_mir_dataflow_src_impls_borrowed_locals.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/un_derefer.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_un_derefer {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_un_derefer.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/points.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_points {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_points.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/move_paths/mod.rs
    pub mod rustc_mir_dataflow_src_move_paths_mod {
        include!("processed_rustc_mir_dataflow_src_move_paths_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/value_analysis.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_value_analysis {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_value_analysis.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/lib.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_lib {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/errors.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_errors {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/liveness.rs
    pub mod rustc_mir_dataflow_src_impls_liveness {
        include!("processed_rustc_mir_dataflow_src_impls_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/initialized.rs
    pub mod rustc_mir_dataflow_src_impls_initialized {
        include!("processed_rustc_mir_dataflow_src_impls_initialized.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/move_paths/builder.rs
    pub mod rustc_mir_dataflow_src_move_paths_builder {
        include!("processed_rustc_mir_dataflow_src_move_paths_builder.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/tests.rs
    pub mod rustc_mir_dataflow_src_framework_tests {
        include!("processed_rustc_mir_dataflow_src_framework_tests.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/mod.rs
    pub mod rustc_mir_dataflow_src_impls_mod {
        include!("processed_rustc_mir_dataflow_src_impls_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/graphviz.rs
    pub mod rustc_mir_dataflow_src_framework_graphviz {
        include!("processed_rustc_mir_dataflow_src_framework_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/storage_liveness.rs
    pub mod rustc_mir_dataflow_src_impls_storage_liveness {
        include!("processed_rustc_mir_dataflow_src_impls_storage_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/visitor.rs
    pub mod rustc_mir_dataflow_src_framework_visitor {
        include!("processed_rustc_mir_dataflow_src_framework_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/lattice.rs
    pub mod rustc_mir_dataflow_src_framework_lattice {
        include!("processed_rustc_mir_dataflow_src_framework_lattice.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/results.rs
    pub mod rustc_mir_dataflow_src_framework_results {
        include!("processed_rustc_mir_dataflow_src_framework_results.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/drop_flag_effects.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_drop_flag_effects {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_drop_flag_effects.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/direction.rs
    pub mod rustc_mir_dataflow_src_framework_direction {
        include!("processed_rustc_mir_dataflow_src_framework_direction.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/debuginfo.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_debuginfo {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/rustc_peek.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_rustc_peek {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_rustc_peek.rs");
    }
}

// 14: rustc_codegen_cranelift (74 files)
pub mod included_rustc_codegen_cranelift {
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm_x86.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm_x86 {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm_x86.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/aot.rs
    pub mod rustc_codegen_cranelift_src_driver_aot {
        include!("processed_rustc_codegen_cranelift_src_driver_aot.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/config.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_config {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_config.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/rustc_info.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_rustc_info {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_rustc_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/mod.rs
    pub mod rustc_codegen_cranelift_src_driver_mod {
        include!("processed_rustc_codegen_cranelift_src_driver_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/jit.rs
    pub mod rustc_codegen_cranelift_src_driver_jit {
        include!("processed_rustc_codegen_cranelift_src_driver_jit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/analyze.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_analyze {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_analyze.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/path.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_path {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_path.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/main_shim.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_main_shim {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_main_shim.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/mod.rs
    pub mod rustc_codegen_cranelift_src_abi_mod {
        include!("processed_rustc_codegen_cranelift_src_abi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/issue-59326.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue_59326 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue-59326.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/mini_core_hello_world.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core_hello_world {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core_hello_world.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/common.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_common {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/optimize/peephole.rs
    pub mod rustc_codegen_cranelift_src_optimize_peephole {
        include!("processed_rustc_codegen_cranelift_src_optimize_peephole.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/pretty_clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_pretty_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_pretty_clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/abi_cafe.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_abi_cafe {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_abi_cafe.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/concurrency_limiter.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_concurrency_limiter {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_concurrency_limiter.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/vtable.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_vtable {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/num.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_num {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_num.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/filter_profile.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_filter_profile {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_filter_profile.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/float-minmax-pass.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_float_minmax_pass {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_float-minmax-pass.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/bench.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_bench {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_bench.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/lib.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_lib {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/pass_mode.rs
    pub mod rustc_codegen_cranelift_src_abi_pass_mode {
        include!("processed_rustc_codegen_cranelift_src_abi_pass_mode.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/cargo-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_cargo_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_cargo-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/raw-dylib.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_raw_dylib {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_raw-dylib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/comments.rs
    pub mod rustc_codegen_cranelift_src_abi_comments {
        include!("processed_rustc_codegen_cranelift_src_abi_comments.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/main.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_main {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_main.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/rustc-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustc_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustc-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/allocator.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_allocator {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/neon.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_neon {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_neon.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/tests.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_tests {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/issue-72793.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue_72793 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue-72793.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm_aarch64.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm_aarch64 {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm_aarch64.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/build_sysroot.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_sysroot {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_sysroot.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/returning.rs
    pub mod rustc_codegen_cranelift_src_abi_returning {
        include!("processed_rustc_codegen_cranelift_src_abi_returning.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/dst-field-align.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_dst_field_align {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_dst-field-align.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/rustdoc-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustdoc_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustdoc-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/types.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_types {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_types.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/std_example.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_std_example {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_std_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/constant.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_constant {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_constant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/codegen_i128.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_i128 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_i128.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/global_asm.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_global_asm {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_global_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/linkage.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_linkage {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_linkage.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/mod.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_mod {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/subslice-patterns-const-eval.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_subslice_patterns_const_eval {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_subslice-patterns-const-eval.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/line_info.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_line_info {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_line_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/mod.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_mod {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/cast.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_cast {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_cast.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/build_backend.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_backend {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_backend.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/emit.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_emit {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_emit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/unwind_module.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_unwind_module {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_unwind_module.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/shared_utils.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_shared_utils {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_shared_utils.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/simd.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_simd {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_simd.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/utils.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_utils {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_utils.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/track-caller-attribute.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_track_caller_attribute {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_track-caller-attribute.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/unsize.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_unsize {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_unsize.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/base.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_base {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/config.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_config {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_config.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/arbitrary_self_types_pointers_and_wrappers.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_arbitrary_self_types_pointers_and_wrappers {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_arbitrary_self_types_pointers_and_wrappers.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/discriminant.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_discriminant {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_discriminant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/prepare.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_prepare {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_prepare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/value_and_place.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_value_and_place {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_value_and_place.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/unwind.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_unwind {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_unwind.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/codegen_f16_f128.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_f16_f128 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_f16_f128.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/toolchain.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_toolchain {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_toolchain.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/example.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_example {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/object.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_object {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_object.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/gen_block_iterate.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_gen_block_iterate {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_gen_block_iterate.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/compiler_builtins.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_compiler_builtins {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_compiler_builtins.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/mini_core.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/inline_asm.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_inline_asm {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_inline_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/pointer.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_pointer {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_pointer.rs");
    }
}

// 15: rustc_baked_icu_data (2 files)
pub mod included_rustc_baked_icu_data {
    // Source: ../rust/compiler/rustc_baked_icu_data/src/lib.rs
    pub mod rustc_baked_icu_data_rustc_baked_icu_data_src_lib {
        include!("processed_rustc_baked_icu_data_rustc_baked_icu_data_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_baked_icu_data/src/data/mod.rs
    pub mod rustc_baked_icu_data_src_data_mod {
        include!("processed_rustc_baked_icu_data_src_data_mod.rs");
    }
}

// 16: rustc_middle (113 files)
pub mod included_rustc_middle {
    // Source: ../rust/compiler/rustc_middle/src/middle/mod.rs
    pub mod rustc_middle_src_middle_mod {
        include!("processed_rustc_middle_src_middle_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/kind.rs
    pub mod rustc_middle_ty_consts_kind {
        include!("processed_rustc_middle_ty_consts_kind.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/arena_cached.rs
    pub mod rustc_middle_src_query_arena_cached {
        include!("processed_rustc_middle_src_query_arena_cached.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/mod.rs
    pub mod rustc_middle_src_hir_mod {
        include!("processed_rustc_middle_src_hir_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/privacy.rs
    pub mod rustc_middle_src_middle_privacy {
        include!("processed_rustc_middle_src_middle_privacy.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/init_mask/tests.rs
    pub mod rustc_middle_allocation_init_mask_tests {
        include!("processed_rustc_middle_allocation_init_mask_tests.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/lint.rs
    pub mod rustc_middle_rustc_middle_src_lint {
        include!("processed_rustc_middle_rustc_middle_src_lint.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/valtree.rs
    pub mod rustc_middle_ty_consts_valtree {
        include!("processed_rustc_middle_ty_consts_valtree.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/sty.rs
    pub mod rustc_middle_src_ty_sty {
        include!("processed_rustc_middle_src_ty_sty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/region.rs
    pub mod rustc_middle_src_ty_region {
        include!("processed_rustc_middle_src_ty_region.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/trait_def.rs
    pub mod rustc_middle_src_ty_trait_def {
        include!("processed_rustc_middle_src_ty_trait_def.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/util/mod.rs
    pub mod rustc_middle_src_util_mod {
        include!("processed_rustc_middle_src_util_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/adt.rs
    pub mod rustc_middle_src_ty_adt {
        include!("processed_rustc_middle_src_ty_adt.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/relate.rs
    pub mod rustc_middle_src_ty_relate {
        include!("processed_rustc_middle_src_ty_relate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/lang_items.rs
    pub mod rustc_middle_src_middle_lang_items {
        include!("processed_rustc_middle_src_middle_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/pretty.rs
    pub mod rustc_middle_src_mir_pretty {
        include!("processed_rustc_middle_src_mir_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/visit.rs
    pub mod rustc_middle_src_mir_visit {
        include!("processed_rustc_middle_src_mir_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/basic_blocks.rs
    pub mod rustc_middle_src_mir_basic_blocks {
        include!("processed_rustc_middle_src_mir_basic_blocks.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/codec.rs
    pub mod rustc_middle_src_ty_codec {
        include!("processed_rustc_middle_src_ty_codec.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/value.rs
    pub mod rustc_middle_mir_interpret_value {
        include!("processed_rustc_middle_mir_interpret_value.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/fast_reject.rs
    pub mod rustc_middle_src_ty_fast_reject {
        include!("processed_rustc_middle_src_ty_fast_reject.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/generics.rs
    pub mod rustc_middle_src_ty_generics {
        include!("processed_rustc_middle_src_ty_generics.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/impls_ty.rs
    pub mod rustc_middle_src_ty_impls_ty {
        include!("processed_rustc_middle_src_ty_impls_ty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/dep_graph/dep_node.rs
    pub mod rustc_middle_src_dep_graph_dep_node {
        include!("processed_rustc_middle_src_dep_graph_dep_node.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/coverage.rs
    pub mod rustc_middle_src_mir_coverage {
        include!("processed_rustc_middle_src_mir_coverage.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/place.rs
    pub mod rustc_middle_src_hir_place {
        include!("processed_rustc_middle_src_hir_place.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/metadata.rs
    pub mod rustc_middle_rustc_middle_src_metadata {
        include!("processed_rustc_middle_rustc_middle_src_metadata.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/int.rs
    pub mod rustc_middle_ty_consts_int {
        include!("processed_rustc_middle_ty_consts_int.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/mod.rs
    pub mod rustc_middle_src_ty_mod {
        include!("processed_rustc_middle_src_ty_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/erase.rs
    pub mod rustc_middle_src_query_erase {
        include!("processed_rustc_middle_src_query_erase.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/dep_graph/mod.rs
    pub mod rustc_middle_src_dep_graph_mod {
        include!("processed_rustc_middle_src_dep_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts.rs
    pub mod rustc_middle_src_ty_consts {
        include!("processed_rustc_middle_src_ty_consts.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/context/tls.rs
    pub mod rustc_middle_ty_context_tls {
        include!("processed_rustc_middle_ty_context_tls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/macros.rs
    pub mod rustc_middle_rustc_middle_src_macros {
        include!("processed_rustc_middle_rustc_middle_src_macros.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/typeck_results.rs
    pub mod rustc_middle_src_ty_typeck_results {
        include!("processed_rustc_middle_src_ty_typeck_results.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/statement.rs
    pub mod rustc_middle_src_mir_statement {
        include!("processed_rustc_middle_src_mir_statement.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/abstract_const.rs
    pub mod rustc_middle_src_ty_abstract_const {
        include!("processed_rustc_middle_src_ty_abstract_const.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/dependency_format.rs
    pub mod rustc_middle_src_middle_dependency_format {
        include!("processed_rustc_middle_src_middle_dependency_format.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/values.rs
    pub mod rustc_middle_rustc_middle_src_values {
        include!("processed_rustc_middle_rustc_middle_src_values.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/infer/canonical.rs
    pub mod rustc_middle_src_infer_canonical {
        include!("processed_rustc_middle_src_infer_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/error.rs
    pub mod rustc_middle_rustc_middle_src_error {
        include!("processed_rustc_middle_rustc_middle_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/provenance_map.rs
    pub mod rustc_middle_interpret_allocation_provenance_map {
        include!("processed_rustc_middle_interpret_allocation_provenance_map.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/specialization_graph.rs
    pub mod rustc_middle_src_traits_specialization_graph {
        include!("processed_rustc_middle_src_traits_specialization_graph.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/keys.rs
    pub mod rustc_middle_src_query_keys {
        include!("processed_rustc_middle_src_query_keys.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/error.rs
    pub mod rustc_middle_src_ty_error {
        include!("processed_rustc_middle_src_ty_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/fold.rs
    pub mod rustc_middle_src_ty_fold {
        include!("processed_rustc_middle_src_ty_fold.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/query.rs
    pub mod rustc_middle_src_traits_query {
        include!("processed_rustc_middle_src_traits_query.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/debugger_visualizer.rs
    pub mod rustc_middle_src_middle_debugger_visualizer {
        include!("processed_rustc_middle_src_middle_debugger_visualizer.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/thir/visit.rs
    pub mod rustc_middle_src_thir_visit {
        include!("processed_rustc_middle_src_thir_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/elaborate_impl.rs
    pub mod rustc_middle_src_ty_elaborate_impl {
        include!("processed_rustc_middle_src_ty_elaborate_impl.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs
    pub mod rustc_middle_src_ty_normalize_erasing_regions {
        include!("processed_rustc_middle_src_ty_normalize_erasing_regions.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/intrinsic.rs
    pub mod rustc_middle_src_ty_intrinsic {
        include!("processed_rustc_middle_src_ty_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/query.rs
    pub mod rustc_middle_src_mir_query {
        include!("processed_rustc_middle_src_mir_query.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/consts.rs
    pub mod rustc_middle_src_mir_consts {
        include!("processed_rustc_middle_src_mir_consts.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/map.rs
    pub mod rustc_middle_src_hir_map {
        include!("processed_rustc_middle_src_hir_map.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/pointer.rs
    pub mod rustc_middle_mir_interpret_pointer {
        include!("processed_rustc_middle_mir_interpret_pointer.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/list.rs
    pub mod rustc_middle_src_ty_list {
        include!("processed_rustc_middle_src_ty_list.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/instance.rs
    pub mod rustc_middle_src_ty_instance {
        include!("processed_rustc_middle_src_ty_instance.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/region.rs
    pub mod rustc_middle_src_middle_region {
        include!("processed_rustc_middle_src_middle_region.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/pattern.rs
    pub mod rustc_middle_src_ty_pattern {
        include!("processed_rustc_middle_src_ty_pattern.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/tests.rs
    pub mod rustc_middle_rustc_middle_src_tests {
        include!("processed_rustc_middle_rustc_middle_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/visit.rs
    pub mod rustc_middle_src_ty_visit {
        include!("processed_rustc_middle_src_ty_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/mod.rs
    pub mod rustc_middle_src_traits_mod {
        include!("processed_rustc_middle_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/cast.rs
    pub mod rustc_middle_src_ty_cast {
        include!("processed_rustc_middle_src_ty_cast.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/on_disk_cache.rs
    pub mod rustc_middle_src_query_on_disk_cache {
        include!("processed_rustc_middle_src_query_on_disk_cache.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/rvalue_scopes.rs
    pub mod rustc_middle_src_ty_rvalue_scopes {
        include!("processed_rustc_middle_src_ty_rvalue_scopes.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/generic_args.rs
    pub mod rustc_middle_src_ty_generic_args {
        include!("processed_rustc_middle_src_ty_generic_args.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation.rs
    pub mod rustc_middle_mir_interpret_allocation {
        include!("processed_rustc_middle_mir_interpret_allocation.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/opaque_types.rs
    pub mod rustc_middle_src_ty_opaque_types {
        include!("processed_rustc_middle_src_ty_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/erase_regions.rs
    pub mod rustc_middle_src_ty_erase_regions {
        include!("processed_rustc_middle_src_ty_erase_regions.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/mod.rs
    pub mod rustc_middle_mir_interpret_mod {
        include!("processed_rustc_middle_mir_interpret_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/nested_filter.rs
    pub mod rustc_middle_src_hir_nested_filter {
        include!("processed_rustc_middle_src_hir_nested_filter.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/codegen_fn_attrs.rs
    pub mod rustc_middle_src_middle_codegen_fn_attrs {
        include!("processed_rustc_middle_src_middle_codegen_fn_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/closure.rs
    pub mod rustc_middle_src_ty_closure {
        include!("processed_rustc_middle_src_ty_closure.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/graphviz.rs
    pub mod rustc_middle_src_mir_graphviz {
        include!("processed_rustc_middle_src_mir_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/diagnostics.rs
    pub mod rustc_middle_src_ty_diagnostics {
        include!("processed_rustc_middle_src_ty_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/print/pretty.rs
    pub mod rustc_middle_ty_print_pretty {
        include!("processed_rustc_middle_ty_print_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/traversal.rs
    pub mod rustc_middle_src_mir_traversal {
        include!("processed_rustc_middle_src_mir_traversal.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/util.rs
    pub mod rustc_middle_src_ty_util {
        include!("processed_rustc_middle_src_ty_util.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/thir.rs
    pub mod rustc_middle_rustc_middle_src_thir {
        include!("processed_rustc_middle_rustc_middle_src_thir.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/mono.rs
    pub mod rustc_middle_src_mir_mono {
        include!("processed_rustc_middle_src_mir_mono.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/loops.rs
    pub mod rustc_middle_src_mir_loops {
        include!("processed_rustc_middle_src_mir_loops.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/generic_graph.rs
    pub mod rustc_middle_src_mir_generic_graph {
        include!("processed_rustc_middle_src_mir_generic_graph.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/inhabitedness/inhabited_predicate.rs
    pub mod rustc_middle_ty_inhabitedness_inhabited_predicate {
        include!("processed_rustc_middle_ty_inhabitedness_inhabited_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/layout.rs
    pub mod rustc_middle_src_ty_layout {
        include!("processed_rustc_middle_src_ty_layout.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/mod.rs
    pub mod rustc_middle_src_query_mod {
        include!("processed_rustc_middle_src_query_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/vtable.rs
    pub mod rustc_middle_src_ty_vtable {
        include!("processed_rustc_middle_src_ty_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/significant_drop_order.rs
    pub mod rustc_middle_src_ty_significant_drop_order {
        include!("processed_rustc_middle_src_ty_significant_drop_order.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/predicate.rs
    pub mod rustc_middle_src_ty_predicate {
        include!("processed_rustc_middle_src_ty_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/structural_impls.rs
    pub mod rustc_middle_src_ty_structural_impls {
        include!("processed_rustc_middle_src_ty_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/resolve_bound_vars.rs
    pub mod rustc_middle_src_middle_resolve_bound_vars {
        include!("processed_rustc_middle_src_middle_resolve_bound_vars.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/adjustment.rs
    pub mod rustc_middle_src_ty_adjustment {
        include!("processed_rustc_middle_src_ty_adjustment.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/generic_graphviz.rs
    pub mod rustc_middle_src_mir_generic_graphviz {
        include!("processed_rustc_middle_src_mir_generic_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/structural_impls.rs
    pub mod rustc_middle_src_traits_structural_impls {
        include!("processed_rustc_middle_src_traits_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/inhabitedness/mod.rs
    pub mod rustc_middle_ty_inhabitedness_mod {
        include!("processed_rustc_middle_ty_inhabitedness_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hooks/mod.rs
    pub mod rustc_middle_src_hooks_mod {
        include!("processed_rustc_middle_src_hooks_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/solve.rs
    pub mod rustc_middle_src_traits_solve {
        include!("processed_rustc_middle_src_traits_solve.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/terminator.rs
    pub mod rustc_middle_src_mir_terminator {
        include!("processed_rustc_middle_src_mir_terminator.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/context.rs
    pub mod rustc_middle_src_ty_context {
        include!("processed_rustc_middle_src_ty_context.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/print/mod.rs
    pub mod rustc_middle_ty_print_mod {
        include!("processed_rustc_middle_ty_print_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/syntax.rs
    pub mod rustc_middle_src_mir_syntax {
        include!("processed_rustc_middle_src_mir_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/assoc.rs
    pub mod rustc_middle_src_ty_assoc {
        include!("processed_rustc_middle_src_ty_assoc.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/error.rs
    pub mod rustc_middle_mir_interpret_error {
        include!("processed_rustc_middle_mir_interpret_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/plumbing.rs
    pub mod rustc_middle_src_query_plumbing {
        include!("processed_rustc_middle_src_query_plumbing.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/exported_symbols.rs
    pub mod rustc_middle_src_middle_exported_symbols {
        include!("processed_rustc_middle_src_middle_exported_symbols.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/stability.rs
    pub mod rustc_middle_src_middle_stability {
        include!("processed_rustc_middle_src_middle_stability.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/util/bug.rs
    pub mod rustc_middle_src_util_bug {
        include!("processed_rustc_middle_src_util_bug.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/select.rs
    pub mod rustc_middle_src_traits_select {
        include!("processed_rustc_middle_src_traits_select.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/queries.rs
    pub mod rustc_middle_mir_interpret_queries {
        include!("processed_rustc_middle_mir_interpret_queries.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/arena.rs
    pub mod rustc_middle_rustc_middle_src_arena {
        include!("processed_rustc_middle_rustc_middle_src_arena.rs");
    }
}

// 17: rustc_trait_selection (73 files)
pub mod included_rustc_trait_selection {
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/call_kind.rs
    pub mod rustc_trait_selection_error_reporting_traits_call_kind {
        include!("processed_rustc_trait_selection_error_reporting_traits_call_kind.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/auto_trait.rs
    pub mod rustc_trait_selection_src_traits_auto_trait {
        include!("processed_rustc_trait_selection_src_traits_auto_trait.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/select/confirmation.rs
    pub mod rustc_trait_selection_traits_select_confirmation {
        include!("processed_rustc_trait_selection_traits_select_confirmation.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs
    pub mod rustc_trait_selection_traits_query_evaluate_obligation {
        include!("processed_rustc_trait_selection_traits_query_evaluate_obligation.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/fulfill.rs
    pub mod rustc_trait_selection_src_solve_fulfill {
        include!("processed_rustc_trait_selection_src_solve_fulfill.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/on_unimplemented_format.rs
    pub mod rustc_trait_selection_error_reporting_traits_on_unimplemented_format {
        include!("processed_rustc_trait_selection_error_reporting_traits_on_unimplemented_format.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/engine.rs
    pub mod rustc_trait_selection_src_traits_engine {
        include!("processed_rustc_trait_selection_src_traits_engine.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/specialize/mod.rs
    pub mod rustc_trait_selection_traits_specialize_mod {
        include!("processed_rustc_trait_selection_traits_specialize_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs
    pub mod rustc_trait_selection_traits_query_method_autoderef {
        include!("processed_rustc_trait_selection_traits_query_method_autoderef.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/lib.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_lib {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/select/mod.rs
    pub mod rustc_trait_selection_traits_select_mod {
        include!("processed_rustc_trait_selection_traits_select_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/find_anon_type.rs
    pub mod rustc_trait_selection_infer_nice_region_error_find_anon_type {
        include!("processed_rustc_trait_selection_infer_nice_region_error_find_anon_type.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/custom.rs
    pub mod rustc_trait_selection_query_type_op_custom {
        include!("processed_rustc_trait_selection_query_type_op_custom.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/fulfill.rs
    pub mod rustc_trait_selection_src_traits_fulfill {
        include!("processed_rustc_trait_selection_src_traits_fulfill.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/mod.rs
    pub mod rustc_trait_selection_error_reporting_traits_mod {
        include!("processed_rustc_trait_selection_error_reporting_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/opaque_types.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_opaque_types {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/util.rs
    pub mod rustc_trait_selection_infer_nice_region_error_util {
        include!("processed_rustc_trait_selection_infer_nice_region_error_util.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/infer.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_infer {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_infer.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/select.rs
    pub mod rustc_trait_selection_src_solve_select {
        include!("processed_rustc_trait_selection_src_solve_select.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/vtable.rs
    pub mod rustc_trait_selection_src_traits_vtable {
        include!("processed_rustc_trait_selection_src_traits_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/ascribe_user_type.rs
    pub mod rustc_trait_selection_query_type_op_ascribe_user_type {
        include!("processed_rustc_trait_selection_query_type_op_ascribe_user_type.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/suggest.rs
    pub mod rustc_trait_selection_error_reporting_infer_suggest {
        include!("processed_rustc_trait_selection_error_reporting_infer_suggest.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/prove_predicate.rs
    pub mod rustc_trait_selection_query_type_op_prove_predicate {
        include!("processed_rustc_trait_selection_query_type_op_prove_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/normalize.rs
    pub mod rustc_trait_selection_query_type_op_normalize {
        include!("processed_rustc_trait_selection_query_type_op_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/on_unimplemented_condition.rs
    pub mod rustc_trait_selection_error_reporting_traits_on_unimplemented_condition {
        include!("processed_rustc_trait_selection_error_reporting_traits_on_unimplemented_condition.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/coherence.rs
    pub mod rustc_trait_selection_src_traits_coherence {
        include!("processed_rustc_trait_selection_src_traits_coherence.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs
    pub mod rustc_trait_selection_traits_specialize_specialization_graph {
        include!("processed_rustc_trait_selection_traits_specialize_specialization_graph.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/placeholder_error.rs
    pub mod rustc_trait_selection_infer_nice_region_error_placeholder_error {
        include!("processed_rustc_trait_selection_infer_nice_region_error_placeholder_error.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/mismatched_static_lifetime.rs
    pub mod rustc_trait_selection_infer_nice_region_error_mismatched_static_lifetime {
        include!("processed_rustc_trait_selection_infer_nice_region_error_mismatched_static_lifetime.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/select/_match.rs
    pub mod rustc_trait_selection_traits_select__match {
        include!("processed_rustc_trait_selection_traits_select__match.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/placeholder_relation.rs
    pub mod rustc_trait_selection_infer_nice_region_error_placeholder_relation {
        include!("processed_rustc_trait_selection_infer_nice_region_error_placeholder_relation.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/ambiguity.rs
    pub mod rustc_trait_selection_error_reporting_traits_ambiguity {
        include!("processed_rustc_trait_selection_error_reporting_traits_ambiguity.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/effects.rs
    pub mod rustc_trait_selection_src_traits_effects {
        include!("processed_rustc_trait_selection_src_traits_effects.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/errors.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_errors {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/inspect.rs
    pub mod rustc_trait_selection_src_solve_inspect {
        include!("processed_rustc_trait_selection_src_solve_inspect.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/implied_outlives_bounds.rs
    pub mod rustc_trait_selection_query_type_op_implied_outlives_bounds {
        include!("processed_rustc_trait_selection_query_type_op_implied_outlives_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/mod.rs
    pub mod rustc_trait_selection_query_type_op_mod {
        include!("processed_rustc_trait_selection_query_type_op_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs
    pub mod rustc_trait_selection_traits_select_candidate_assembly {
        include!("processed_rustc_trait_selection_traits_select_candidate_assembly.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/structural_normalize.rs
    pub mod rustc_trait_selection_src_traits_structural_normalize {
        include!("processed_rustc_trait_selection_src_traits_structural_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/inspect/analyse.rs
    pub mod rustc_trait_selection_solve_inspect_analyse {
        include!("processed_rustc_trait_selection_solve_inspect_analyse.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/on_unimplemented.rs
    pub mod rustc_trait_selection_error_reporting_traits_on_unimplemented {
        include!("processed_rustc_trait_selection_error_reporting_traits_on_unimplemented.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/delegate.rs
    pub mod rustc_trait_selection_src_solve_delegate {
        include!("processed_rustc_trait_selection_src_solve_delegate.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/normalize.rs
    pub mod rustc_trait_selection_src_solve_normalize {
        include!("processed_rustc_trait_selection_src_solve_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/outlives.rs
    pub mod rustc_trait_selection_query_type_op_outlives {
        include!("processed_rustc_trait_selection_query_type_op_outlives.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/different_lifetimes.rs
    pub mod rustc_trait_selection_infer_nice_region_error_different_lifetimes {
        include!("processed_rustc_trait_selection_infer_nice_region_error_different_lifetimes.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/util.rs
    pub mod rustc_trait_selection_src_traits_util {
        include!("processed_rustc_trait_selection_src_traits_util.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/region.rs
    pub mod rustc_trait_selection_error_reporting_infer_region {
        include!("processed_rustc_trait_selection_error_reporting_infer_region.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/mod.rs
    pub mod rustc_trait_selection_src_traits_mod {
        include!("processed_rustc_trait_selection_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/named_anon_conflict.rs
    pub mod rustc_trait_selection_infer_nice_region_error_named_anon_conflict {
        include!("processed_rustc_trait_selection_infer_nice_region_error_named_anon_conflict.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/fulfillment_errors.rs
    pub mod rustc_trait_selection_error_reporting_traits_fulfillment_errors {
        include!("processed_rustc_trait_selection_error_reporting_traits_fulfillment_errors.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/dyn_compatibility.rs
    pub mod rustc_trait_selection_src_traits_dyn_compatibility {
        include!("processed_rustc_trait_selection_src_traits_dyn_compatibility.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/static_impl_trait.rs
    pub mod rustc_trait_selection_infer_nice_region_error_static_impl_trait {
        include!("processed_rustc_trait_selection_infer_nice_region_error_static_impl_trait.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/suggestions.rs
    pub mod rustc_trait_selection_error_reporting_traits_suggestions {
        include!("processed_rustc_trait_selection_error_reporting_traits_suggestions.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs
    pub mod rustc_trait_selection_src_traits_const_evaluatable {
        include!("processed_rustc_trait_selection_src_traits_const_evaluatable.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs
    pub mod rustc_trait_selection_traits_query_dropck_outlives {
        include!("processed_rustc_trait_selection_traits_query_dropck_outlives.rs");
    }
