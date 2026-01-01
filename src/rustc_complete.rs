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
    // Source: ../rust/compiler/rustc_data_structures/src/marker.rs
    pub mod rustc_data_structures_rustc_data_structures_src_marker {
        include!("processed_rustc_data_structures_rustc_data_structures_src_marker.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fx.rs
    pub mod rustc_data_structures_rustc_data_structures_src_fx {
        include!("processed_rustc_data_structures_rustc_data_structures_src_fx.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thinvec.rs
    pub mod rustc_data_structures_rustc_data_structures_src_thinvec {
        include!("processed_rustc_data_structures_rustc_data_structures_src_thinvec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/aligned.rs
    pub mod rustc_data_structures_rustc_data_structures_src_aligned {
        include!("processed_rustc_data_structures_rustc_data_structures_src_aligned.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/atomic_ref.rs
    pub mod rustc_data_structures_rustc_data_structures_src_atomic_ref {
        include!("processed_rustc_data_structures_rustc_data_structures_src_atomic_ref.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reversed.rs
    pub mod rustc_data_structures_src_graph_reversed {
        include!("processed_rustc_data_structures_src_graph_reversed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/intern/tests.rs
    pub mod rustc_data_structures_src_intern_tests {
        include!("processed_rustc_data_structures_src_intern_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/lib.rs
    pub mod rustc_data_structures_rustc_data_structures_src_lib {
        include!("processed_rustc_data_structures_rustc_data_structures_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/map.rs
    pub mod rustc_data_structures_src_sso_map {
        include!("processed_rustc_data_structures_src_sso_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/set.rs
    pub mod rustc_data_structures_src_sso_set {
        include!("processed_rustc_data_structures_src_sso_set.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/iterate/tests.rs
    pub mod rustc_data_structures_graph_iterate_tests {
        include!("processed_rustc_data_structures_graph_iterate_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/tests.rs
    pub mod rustc_data_structures_src_obligation_forest_tests {
        include!("processed_rustc_data_structures_src_obligation_forest_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/mod.rs
    pub mod rustc_data_structures_src_sso_mod {
        include!("processed_rustc_data_structures_src_sso_mod.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/graphviz.rs
    pub mod rustc_data_structures_src_obligation_forest_graphviz {
        include!("processed_rustc_data_structures_src_obligation_forest_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/linux.rs
    pub mod rustc_data_structures_src_flock_linux {
        include!("processed_rustc_data_structures_src_flock_linux.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sharded.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sharded {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sharded.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/steal.rs
    pub mod rustc_data_structures_rustc_data_structures_src_steal {
        include!("processed_rustc_data_structures_rustc_data_structures_src_steal.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unix.rs
    pub mod rustc_data_structures_src_flock_unix {
        include!("processed_rustc_data_structures_src_flock_unix.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/memmap.rs
    pub mod rustc_data_structures_rustc_data_structures_src_memmap {
        include!("processed_rustc_data_structures_rustc_data_structures_src_memmap.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/freeze.rs
    pub mod rustc_data_structures_src_sync_freeze {
        include!("processed_rustc_data_structures_src_sync_freeze.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/jobserver.rs
    pub mod rustc_data_structures_rustc_data_structures_src_jobserver {
        include!("processed_rustc_data_structures_rustc_data_structures_src_jobserver.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/dominators/tests.rs
    pub mod rustc_data_structures_graph_dominators_tests {
        include!("processed_rustc_data_structures_graph_dominators_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/transitive_relation/tests.rs
    pub mod rustc_data_structures_src_transitive_relation_tests {
        include!("processed_rustc_data_structures_src_transitive_relation_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/svh.rs
    pub mod rustc_data_structures_rustc_data_structures_src_svh {
        include!("processed_rustc_data_structures_rustc_data_structures_src_svh.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/windows.rs
    pub mod rustc_data_structures_src_flock_windows {
        include!("processed_rustc_data_structures_src_flock_windows.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/frozen.rs
    pub mod rustc_data_structures_rustc_data_structures_src_frozen {
        include!("processed_rustc_data_structures_rustc_data_structures_src_frozen.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/snapshot_map/tests.rs
    pub mod rustc_data_structures_src_snapshot_map_tests {
        include!("processed_rustc_data_structures_src_snapshot_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stable_hasher/tests.rs
    pub mod rustc_data_structures_src_stable_hasher_tests {
        include!("processed_rustc_data_structures_src_stable_hasher_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/tagged_ptr/tests.rs
    pub mod rustc_data_structures_src_tagged_ptr_tests {
        include!("processed_rustc_data_structures_src_tagged_ptr_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/tests.rs
    pub mod rustc_data_structures_src_graph_tests {
        include!("processed_rustc_data_structures_src_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/linked_graph/tests.rs
    pub mod rustc_data_structures_graph_linked_graph_tests {
        include!("processed_rustc_data_structures_graph_linked_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/vec_cache/tests.rs
    pub mod rustc_data_structures_src_vec_cache_tests {
        include!("processed_rustc_data_structures_src_vec_cache_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/union_find/tests.rs
    pub mod rustc_data_structures_src_union_find_tests {
        include!("processed_rustc_data_structures_src_union_find_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/small_c_str/tests.rs
    pub mod rustc_data_structures_src_small_c_str_tests {
        include!("processed_rustc_data_structures_src_small_c_str_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/temp_dir.rs
    pub mod rustc_data_structures_rustc_data_structures_src_temp_dir {
        include!("processed_rustc_data_structures_rustc_data_structures_src_temp_dir.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/profiling/tests.rs
    pub mod rustc_data_structures_src_profiling_tests {
        include!("processed_rustc_data_structures_src_profiling_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/owned_slice/tests.rs
    pub mod rustc_data_structures_src_owned_slice_tests {
        include!("processed_rustc_data_structures_src_owned_slice_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fingerprint/tests.rs
    pub mod rustc_data_structures_src_fingerprint_tests {
        include!("processed_rustc_data_structures_src_fingerprint_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/vec.rs
    pub mod rustc_data_structures_src_sync_vec {
        include!("processed_rustc_data_structures_src_sync_vec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thousands/tests.rs
    pub mod rustc_data_structures_src_thousands_tests {
        include!("processed_rustc_data_structures_src_thousands_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/vec_graph/tests.rs
    pub mod rustc_data_structures_graph_vec_graph_tests {
        include!("processed_rustc_data_structures_graph_vec_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/packed.rs
    pub mod rustc_data_structures_rustc_data_structures_src_packed {
        include!("processed_rustc_data_structures_rustc_data_structures_src_packed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/binary_search_util/tests.rs
    pub mod rustc_data_structures_src_binary_search_util_tests {
        include!("processed_rustc_data_structures_src_binary_search_util_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unord.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unord {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unord.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unsupported.rs
    pub mod rustc_data_structures_src_flock_unsupported {
        include!("processed_rustc_data_structures_src_flock_unsupported.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unhash.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unhash {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unhash.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stack.rs
    pub mod rustc_data_structures_rustc_data_structures_src_stack {
        include!("processed_rustc_data_structures_rustc_data_structures_src_stack.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reference.rs
    pub mod rustc_data_structures_src_graph_reference {
        include!("processed_rustc_data_structures_src_graph_reference.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/parallel.rs
    pub mod rustc_data_structures_src_sync_parallel {
        include!("processed_rustc_data_structures_src_sync_parallel.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sync {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sync.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/base_n/tests.rs
    pub mod rustc_data_structures_src_base_n_tests {
        include!("processed_rustc_data_structures_src_base_n_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/scc/tests.rs
    pub mod rustc_data_structures_graph_scc_tests {
        include!("processed_rustc_data_structures_graph_scc_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/index_map.rs
    pub mod rustc_data_structures_src_sorted_map_index_map {
        include!("processed_rustc_data_structures_src_sorted_map_index_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/worker_local.rs
    pub mod rustc_data_structures_src_sync_worker_local {
        include!("processed_rustc_data_structures_src_sync_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/work_queue.rs
    pub mod rustc_data_structures_rustc_data_structures_src_work_queue {
        include!("processed_rustc_data_structures_rustc_data_structures_src_work_queue.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flat_map_in_place.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flat_map_in_place {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flat_map_in_place.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/tests.rs
    pub mod rustc_data_structures_src_sorted_map_tests {
        include!("processed_rustc_data_structures_src_sorted_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/lock.rs
    pub mod rustc_data_structures_src_sync_lock {
        include!("processed_rustc_data_structures_src_sync_lock.rs");
    }
}

// 3: rustc_index (9 files)
pub mod included_rustc_index {
    // Source: ../rust/compiler/rustc_index/src/bit_set/tests.rs
    pub mod rustc_index_src_bit_set_tests {
        include!("processed_rustc_index_src_bit_set_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/lib.rs
    pub mod rustc_index_rustc_index_src_lib {
        include!("processed_rustc_index_rustc_index_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/idx.rs
    pub mod rustc_index_rustc_index_src_idx {
        include!("processed_rustc_index_rustc_index_src_idx.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/slice.rs
    pub mod rustc_index_rustc_index_src_slice {
        include!("processed_rustc_index_rustc_index_src_slice.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/interval/tests.rs
    pub mod rustc_index_src_interval_tests {
        include!("processed_rustc_index_src_interval_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/vec/tests.rs
    pub mod rustc_index_src_vec_tests {
        include!("processed_rustc_index_src_vec_tests.rs");
    }
}

// 4: rustc_span (17 files)
pub mod included_rustc_span {
    // Source: ../rust/compiler/rustc_span/src/def_id.rs
    pub mod rustc_span_rustc_span_src_def_id {
        include!("processed_rustc_span_rustc_span_src_def_id.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/profiling.rs
    pub mod rustc_span_rustc_span_src_profiling {
        include!("processed_rustc_span_rustc_span_src_profiling.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edit_distance/tests.rs
    pub mod rustc_span_src_edit_distance_tests {
        include!("processed_rustc_span_src_edit_distance_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/source_map/tests.rs
    pub mod rustc_span_src_source_map_tests {
        include!("processed_rustc_span_src_source_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/symbol/tests.rs
    pub mod rustc_span_src_symbol_tests {
        include!("processed_rustc_span_src_symbol_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/fatal_error.rs
    pub mod rustc_span_rustc_span_src_fatal_error {
        include!("processed_rustc_span_rustc_span_src_fatal_error.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/tests.rs
    pub mod rustc_span_rustc_span_src_tests {
        include!("processed_rustc_span_rustc_span_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/hygiene.rs
    pub mod rustc_span_rustc_span_src_hygiene {
        include!("processed_rustc_span_rustc_span_src_hygiene.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/caching_source_map_view.rs
    pub mod rustc_span_rustc_span_src_caching_source_map_view {
        include!("processed_rustc_span_rustc_span_src_caching_source_map_view.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/analyze_source_file/tests.rs
    pub mod rustc_span_src_analyze_source_file_tests {
        include!("processed_rustc_span_src_analyze_source_file_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edition.rs
    pub mod rustc_span_rustc_span_src_edition {
        include!("processed_rustc_span_rustc_span_src_edition.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/span_encoding.rs
    pub mod rustc_span_rustc_span_src_span_encoding {
        include!("processed_rustc_span_rustc_span_src_span_encoding.rs");
    }
}

// 5: rustc_thread_pool (28 files)
pub mod included_rustc_thread_pool {
    // Source: ../rust/compiler/rustc_thread_pool/tests/stack_overflow_crash.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_stack_overflow_crash {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_stack_overflow_crash.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/join/tests.rs
    pub mod rustc_thread_pool_src_join_tests {
        include!("processed_rustc_thread_pool_src_join_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/double_init_fail.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_double_init_fail {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_double_init_fail.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/thread_pool/mod.rs
    pub mod rustc_thread_pool_src_thread_pool_mod {
        include!("processed_rustc_thread_pool_src_thread_pool_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/scoped_threadpool.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_scoped_threadpool {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_scoped_threadpool.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/lib.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_lib {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/worker_local.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_worker_local {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/unwind.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_unwind {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_unwind.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/latch.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_latch {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_latch.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/simple_panic.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_simple_panic {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_simple_panic.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/scope/tests.rs
    pub mod rustc_thread_pool_src_scope_tests {
        include!("processed_rustc_thread_pool_src_scope_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/sleep/counters.rs
    pub mod rustc_thread_pool_src_sleep_counters {
        include!("processed_rustc_thread_pool_src_sleep_counters.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/broadcast/mod.rs
    pub mod rustc_thread_pool_src_broadcast_mod {
        include!("processed_rustc_thread_pool_src_broadcast_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/registry.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_registry {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_registry.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/private.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_private {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_private.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/job.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_job {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_job.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/compile_fail/rc_return.rs
    pub mod rustc_thread_pool_src_compile_fail_rc_return {
        include!("processed_rustc_thread_pool_src_compile_fail_rc_return.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/spawn/tests.rs
    pub mod rustc_thread_pool_src_spawn_tests {
        include!("processed_rustc_thread_pool_src_spawn_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/tlv.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_tlv {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_tlv.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/sleep/mod.rs
    pub mod rustc_thread_pool_src_sleep_mod {
        include!("processed_rustc_thread_pool_src_sleep_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/init_zero_threads.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_init_zero_threads {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_init_zero_threads.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/scope_join.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_scope_join {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_scope_join.rs");
    }
}

// 6: rustc_feature (6 files)
pub mod included_rustc_feature {
    // Source: ../rust/compiler/rustc_feature/src/unstable.rs
    pub mod rustc_feature_rustc_feature_src_unstable {
        include!("processed_rustc_feature_rustc_feature_src_unstable.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/tests.rs
    pub mod rustc_feature_rustc_feature_src_tests {
        include!("processed_rustc_feature_rustc_feature_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/removed.rs
    pub mod rustc_feature_rustc_feature_src_removed {
        include!("processed_rustc_feature_rustc_feature_src_removed.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/builtin_attrs.rs
    pub mod rustc_feature_rustc_feature_src_builtin_attrs {
        include!("processed_rustc_feature_rustc_feature_src_builtin_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/accepted.rs
    pub mod rustc_feature_rustc_feature_src_accepted {
        include!("processed_rustc_feature_rustc_feature_src_accepted.rs");
    }
}

// 7: rustc_resolve (14 files)
pub mod included_rustc_resolve {
    // Source: ../rust/compiler/rustc_resolve/src/rustdoc/tests.rs
    pub mod rustc_resolve_src_rustdoc_tests {
        include!("processed_rustc_resolve_src_rustdoc_tests.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/build_reduced_graph.rs
    pub mod rustc_resolve_rustc_resolve_src_build_reduced_graph {
        include!("processed_rustc_resolve_rustc_resolve_src_build_reduced_graph.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/lib.rs
    pub mod rustc_resolve_rustc_resolve_src_lib {
        include!("processed_rustc_resolve_rustc_resolve_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/late.rs
    pub mod rustc_resolve_rustc_resolve_src_late {
        include!("processed_rustc_resolve_rustc_resolve_src_late.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/macros.rs
    pub mod rustc_resolve_rustc_resolve_src_macros {
        include!("processed_rustc_resolve_rustc_resolve_src_macros.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/def_collector.rs
    pub mod rustc_resolve_rustc_resolve_src_def_collector {
        include!("processed_rustc_resolve_rustc_resolve_src_def_collector.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/errors.rs
    pub mod rustc_resolve_rustc_resolve_src_errors {
        include!("processed_rustc_resolve_rustc_resolve_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/imports.rs
    pub mod rustc_resolve_rustc_resolve_src_imports {
        include!("processed_rustc_resolve_rustc_resolve_src_imports.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/diagnostics.rs
    pub mod rustc_resolve_rustc_resolve_src_diagnostics {
        include!("processed_rustc_resolve_rustc_resolve_src_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/ident.rs
    pub mod rustc_resolve_rustc_resolve_src_ident {
        include!("processed_rustc_resolve_rustc_resolve_src_ident.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/late/diagnostics.rs
    pub mod rustc_resolve_src_late_diagnostics {
        include!("processed_rustc_resolve_src_late_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/effective_visibilities.rs
    pub mod rustc_resolve_rustc_resolve_src_effective_visibilities {
        include!("processed_rustc_resolve_rustc_resolve_src_effective_visibilities.rs");
    }
}

// 8: rustc_sanitizers (8 files)
pub mod included_rustc_sanitizers {
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/mod.rs
    pub mod rustc_sanitizers_typeid_itanium_cxx_abi_mod {
        include!("processed_rustc_sanitizers_typeid_itanium_cxx_abi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/lib.rs
    pub mod rustc_sanitizers_rustc_sanitizers_src_lib {
        include!("processed_rustc_sanitizers_rustc_sanitizers_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/mod.rs
    pub mod rustc_sanitizers_cfi_typeid_mod {
        include!("processed_rustc_sanitizers_cfi_typeid_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/encode.rs
    pub mod rustc_sanitizers_typeid_itanium_cxx_abi_encode {
        include!("processed_rustc_sanitizers_typeid_itanium_cxx_abi_encode.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/transform.rs
    pub mod rustc_sanitizers_typeid_itanium_cxx_abi_transform {
        include!("processed_rustc_sanitizers_typeid_itanium_cxx_abi_transform.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/kcfi/mod.rs
    pub mod rustc_sanitizers_src_kcfi_mod {
        include!("processed_rustc_sanitizers_src_kcfi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/kcfi/typeid/mod.rs
    pub mod rustc_sanitizers_kcfi_typeid_mod {
        include!("processed_rustc_sanitizers_kcfi_typeid_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/mod.rs
    pub mod rustc_sanitizers_src_cfi_mod {
        include!("processed_rustc_sanitizers_src_cfi_mod.rs");
    }
}

// 9: rustc (2 files)
pub mod included_rustc {
    // Source: ../rust/compiler/rustc/src/main.rs
    pub mod rustc_rustc_src_main {
        include!("processed_rustc_rustc_src_main.rs");
    }
    // Source: ../rust/compiler/rustc/build.rs
    pub mod rustc_compiler_rustc_build {
        include!("processed_rustc_compiler_rustc_build.rs");
    }
}

// 10: rustc_lexer (3 files)
pub mod included_rustc_lexer {
    // Source: ../rust/compiler/rustc_lexer/src/cursor.rs
    pub mod rustc_lexer_rustc_lexer_src_cursor {
        include!("processed_rustc_lexer_rustc_lexer_src_cursor.rs");
    }
    // Source: ../rust/compiler/rustc_lexer/src/tests.rs
    pub mod rustc_lexer_rustc_lexer_src_tests {
        include!("processed_rustc_lexer_rustc_lexer_src_tests.rs");
    }
}

// 11: rustc_baked_icu_data (2 files)
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

// 12: rustc_mir_transform (93 files)
pub mod included_rustc_mir_transform {
    // Source: ../rust/compiler/rustc_mir_transform/src/lower_slice_len.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lower_slice_len {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lower_slice_len.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/unreachable_enum_branching.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_unreachable_enum_branching {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_unreachable_enum_branching.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/counters/balanced_flow.rs
    pub mod rustc_mir_transform_coverage_counters_balanced_flow {
        include!("processed_rustc_mir_transform_coverage_counters_balanced_flow.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/lint.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lint {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lint.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_inline_always_target_features.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_inline_always_target_features {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_inline_always_target_features.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/add_moves_for_packed_drops.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_add_moves_for_packed_drops {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_add_moves_for_packed_drops.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/pass_manager.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_pass_manager {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_pass_manager.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/unexpand.rs
    pub mod rustc_mir_transform_src_coverage_unexpand {
        include!("processed_rustc_mir_transform_src_coverage_unexpand.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/elaborate_drops.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_elaborate_drops {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_elaborate_drops.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/dead_store_elimination.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_dead_store_elimination {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_dead_store_elimination.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_const_item_mutation.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_const_item_mutation {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_const_item_mutation.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/dataflow_const_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_dataflow_const_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_dataflow_const_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/function_item_references.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_function_item_references {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_function_item_references.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/abort_unwinding_calls.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_abort_unwinding_calls {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_abort_unwinding_calls.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/expansion.rs
    pub mod rustc_mir_transform_src_coverage_expansion {
        include!("processed_rustc_mir_transform_src_coverage_expansion.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/promote_consts.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_promote_consts {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_promote_consts.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/mappings.rs
    pub mod rustc_mir_transform_src_coverage_mappings {
        include!("processed_rustc_mir_transform_src_coverage_mappings.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/instsimplify.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_instsimplify {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_instsimplify.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/unreachable_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_unreachable_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_unreachable_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/graph.rs
    pub mod rustc_mir_transform_src_coverage_graph {
        include!("processed_rustc_mir_transform_src_coverage_graph.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/strip_debuginfo.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_strip_debuginfo {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_strip_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_call_recursion.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_call_recursion {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_call_recursion.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_place_mention.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_place_mention {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_place_mention.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/shim.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_shim {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_shim.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/ssa.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_ssa {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_ssa.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/dump_mir.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_dump_mir {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_dump_mir.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/shim/async_destructor_ctor.rs
    pub mod rustc_mir_transform_src_shim_async_destructor_ctor {
        include!("processed_rustc_mir_transform_src_shim_async_destructor_ctor.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_zsts.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_zsts {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_zsts.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_enums.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_enums {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_enums.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/cleanup_post_borrowck.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_cleanup_post_borrowck {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_cleanup_post_borrowck.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/cost_checker.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_cost_checker {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_cost_checker.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/impossible_predicates.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_impossible_predicates {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_impossible_predicates.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/query.rs
    pub mod rustc_mir_transform_src_coverage_query {
        include!("processed_rustc_mir_transform_src_coverage_query.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coroutine.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_coroutine {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_coroutine.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/mentioned_items.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_mentioned_items {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_mentioned_items.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/lib.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lib {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_alignment.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_alignment {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_alignment.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_storage_markers.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_storage_markers {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_storage_markers.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/errors.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_errors {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/simplify_branches.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_simplify_branches {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_simplify_branches.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/counters.rs
    pub mod rustc_mir_transform_src_coverage_counters {
        include!("processed_rustc_mir_transform_src_coverage_counters.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/sroa.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_sroa {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_sroa.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/inline.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_inline {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_inline.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/ctfe_limit.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_ctfe_limit {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_ctfe_limit.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_packed_ref.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_packed_ref {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_packed_ref.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/gvn.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_gvn {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_gvn.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_unneeded_drops.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_unneeded_drops {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_unneeded_drops.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/elaborate_drop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_elaborate_drop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_elaborate_drop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/post_drop_elaboration.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_post_drop_elaboration {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_post_drop_elaboration.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/ffi_unwind_calls.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_ffi_unwind_calls {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_ffi_unwind_calls.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/known_panics_lint.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_known_panics_lint {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_known_panics_lint.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/required_consts.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_required_consts {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_required_consts.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_inline.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_inline {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_inline.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/add_call_guards.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_add_call_guards {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_add_call_guards.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/hir_info.rs
    pub mod rustc_mir_transform_src_coverage_hir_info {
        include!("processed_rustc_mir_transform_src_coverage_hir_info.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/validate.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_validate {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_validate.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/post_analysis_normalize.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_post_analysis_normalize {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_post_analysis_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_pointers.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_pointers {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_pointers.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/elaborate_box_derefs.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_elaborate_box_derefs {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_elaborate_box_derefs.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coroutine/by_move_body.rs
    pub mod rustc_mir_transform_src_coroutine_by_move_body {
        include!("processed_rustc_mir_transform_src_coroutine_by_move_body.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_uninit_drops.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_uninit_drops {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_uninit_drops.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/add_retag.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_add_retag {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_add_retag.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/prettify.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_prettify {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_prettify.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/ref_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_ref_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_ref_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/early_otherwise_branch.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_early_otherwise_branch {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_early_otherwise_branch.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/nrvo.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_nrvo {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_nrvo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/patch.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_patch {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_patch.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/add_subtyping_projections.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_add_subtyping_projections {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_add_subtyping_projections.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/multiple_return_terminators.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_multiple_return_terminators {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_multiple_return_terminators.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/lower_intrinsics.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lower_intrinsics {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lower_intrinsics.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/counters/node_flow/tests.rs
    pub mod rustc_mir_transform_counters_node_flow_tests {
        include!("processed_rustc_mir_transform_counters_node_flow_tests.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/tests.rs
    pub mod rustc_mir_transform_src_coverage_tests {
        include!("processed_rustc_mir_transform_src_coverage_tests.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/cross_crate_inline.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_cross_crate_inline {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_cross_crate_inline.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/simplify_comparison_integral.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_simplify_comparison_integral {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_simplify_comparison_integral.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/deduce_param_attrs.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_deduce_param_attrs {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_deduce_param_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/single_use_consts.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_single_use_consts {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_single_use_consts.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/dest_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_dest_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_dest_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_null.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_null {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_null.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/deref_separator.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_deref_separator {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_deref_separator.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/large_enums.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_large_enums {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_large_enums.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/spans.rs
    pub mod rustc_mir_transform_src_coverage_spans {
        include!("processed_rustc_mir_transform_src_coverage_spans.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/lint_tail_expr_drop_order.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lint_tail_expr_drop_order {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lint_tail_expr_drop_order.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/simplify.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_simplify {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_simplify.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/match_branches.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_match_branches {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_match_branches.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/jump_threading.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_jump_threading {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_jump_threading.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/inline/cycle.rs
    pub mod rustc_mir_transform_src_inline_cycle {
        include!("processed_rustc_mir_transform_src_inline_cycle.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_noop_landing_pads.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_noop_landing_pads {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_noop_landing_pads.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/spans/from_mir.rs
    pub mod rustc_mir_transform_coverage_spans_from_mir {
        include!("processed_rustc_mir_transform_coverage_spans_from_mir.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/copy_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_copy_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_copy_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/sanity_check.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_sanity_check {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_sanity_check.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coroutine/drop.rs
    pub mod rustc_mir_transform_src_coroutine_drop {
        include!("processed_rustc_mir_transform_src_coroutine_drop.rs");
    }
}

// 13: rustc_graphviz (2 files)
pub mod included_rustc_graphviz {
    // Source: ../rust/compiler/rustc_graphviz/src/tests.rs
    pub mod rustc_graphviz_rustc_graphviz_src_tests {
        include!("processed_rustc_graphviz_rustc_graphviz_src_tests.rs");
    }
}

// 14: rustc_query_system (19 files)
pub mod included_rustc_query_system {
    // Source: ../rust/compiler/rustc_query_system/src/values.rs
    pub mod rustc_query_system_rustc_query_system_src_values {
        include!("processed_rustc_query_system_rustc_query_system_src_values.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/mod.rs
    pub mod rustc_query_system_src_dep_graph_mod {
        include!("processed_rustc_query_system_src_dep_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/cache.rs
    pub mod rustc_query_system_rustc_query_system_src_cache {
        include!("processed_rustc_query_system_rustc_query_system_src_cache.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/impls_syntax.rs
    pub mod rustc_query_system_src_ich_impls_syntax {
        include!("processed_rustc_query_system_src_ich_impls_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/debug.rs
    pub mod rustc_query_system_src_dep_graph_debug {
        include!("processed_rustc_query_system_src_dep_graph_debug.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/query.rs
    pub mod rustc_query_system_src_dep_graph_query {
        include!("processed_rustc_query_system_src_dep_graph_query.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/dep_node.rs
    pub mod rustc_query_system_src_dep_graph_dep_node {
        include!("processed_rustc_query_system_src_dep_graph_dep_node.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/mod.rs
    pub mod rustc_query_system_src_ich_mod {
        include!("processed_rustc_query_system_src_ich_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/graph.rs
    pub mod rustc_query_system_src_dep_graph_graph {
        include!("processed_rustc_query_system_src_dep_graph_graph.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/config.rs
    pub mod rustc_query_system_src_query_config {
        include!("processed_rustc_query_system_src_query_config.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/error.rs
    pub mod rustc_query_system_rustc_query_system_src_error {
        include!("processed_rustc_query_system_rustc_query_system_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/plumbing.rs
    pub mod rustc_query_system_src_query_plumbing {
        include!("processed_rustc_query_system_src_query_plumbing.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/lib.rs
    pub mod rustc_query_system_rustc_query_system_src_lib {
        include!("processed_rustc_query_system_rustc_query_system_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/edges.rs
    pub mod rustc_query_system_src_dep_graph_edges {
        include!("processed_rustc_query_system_src_dep_graph_edges.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/mod.rs
    pub mod rustc_query_system_src_query_mod {
        include!("processed_rustc_query_system_src_query_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/hcx.rs
    pub mod rustc_query_system_src_ich_hcx {
        include!("processed_rustc_query_system_src_ich_hcx.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/serialized.rs
    pub mod rustc_query_system_src_dep_graph_serialized {
        include!("processed_rustc_query_system_src_dep_graph_serialized.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/job.rs
    pub mod rustc_query_system_src_query_job {
        include!("processed_rustc_query_system_src_query_job.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/caches.rs
    pub mod rustc_query_system_src_query_caches {
        include!("processed_rustc_query_system_src_query_caches.rs");
    }
}

// 15: rustc_type_ir_macros (1 files)
pub mod included_rustc_type_ir_macros {
    // Source: ../rust/compiler/rustc_type_ir_macros/src/lib.rs
    pub mod rustc_type_ir_macros_rustc_type_ir_macros_src_lib {
        include!("processed_rustc_type_ir_macros_rustc_type_ir_macros_src_lib.rs");
    }
}

// 16: rustc_hir_typeck (40 files)
pub mod included_rustc_hir_typeck {
    // Source: ../rust/compiler/rustc_hir_typeck/src/op.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_op {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_op.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/mod.rs
    pub mod rustc_hir_typeck_src_method_mod {
        include!("processed_rustc_hir_typeck_src_method_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/confirm.rs
    pub mod rustc_hir_typeck_src_method_confirm {
        include!("processed_rustc_hir_typeck_src_method_confirm.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/_match.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src__match {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src__match.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/typeck_root_ctxt.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_typeck_root_ctxt {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_typeck_root_ctxt.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/upvar.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_upvar {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_upvar.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/naked_functions.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_naked_functions {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_naked_functions.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/opaque_types.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_opaque_types {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/intrinsicck.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_intrinsicck {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_intrinsicck.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/closure.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_closure {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_closure.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/errors.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_errors {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/prelude_edition_lints.rs
    pub mod rustc_hir_typeck_src_method_prelude_edition_lints {
        include!("processed_rustc_hir_typeck_src_method_prelude_edition_lints.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/mod.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_mod {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/pat.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_pat {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_pat.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/loops.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_loops {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_loops.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/cast.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_cast {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_cast.rs");
    }
