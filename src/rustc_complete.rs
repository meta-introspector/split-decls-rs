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
    // Source: ../rust/compiler/rustc_data_structures/src/sso/map.rs
    pub mod rustc_data_structures_src_sso_map {
        include!("processed_rustc_data_structures_src_sso_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/svh.rs
    pub mod rustc_data_structures_rustc_data_structures_src_svh {
        include!("processed_rustc_data_structures_rustc_data_structures_src_svh.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/marker.rs
    pub mod rustc_data_structures_rustc_data_structures_src_marker {
        include!("processed_rustc_data_structures_rustc_data_structures_src_marker.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/memmap.rs
    pub mod rustc_data_structures_rustc_data_structures_src_memmap {
        include!("processed_rustc_data_structures_rustc_data_structures_src_memmap.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/profiling/tests.rs
    pub mod rustc_data_structures_src_profiling_tests {
        include!("processed_rustc_data_structures_src_profiling_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/base_n/tests.rs
    pub mod rustc_data_structures_src_base_n_tests {
        include!("processed_rustc_data_structures_src_base_n_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/binary_search_util/tests.rs
    pub mod rustc_data_structures_src_binary_search_util_tests {
        include!("processed_rustc_data_structures_src_binary_search_util_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/lib.rs
    pub mod rustc_data_structures_rustc_data_structures_src_lib {
        include!("processed_rustc_data_structures_rustc_data_structures_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thousands/tests.rs
    pub mod rustc_data_structures_src_thousands_tests {
        include!("processed_rustc_data_structures_src_thousands_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/work_queue.rs
    pub mod rustc_data_structures_rustc_data_structures_src_work_queue {
        include!("processed_rustc_data_structures_rustc_data_structures_src_work_queue.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sync {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sync.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/owned_slice/tests.rs
    pub mod rustc_data_structures_src_owned_slice_tests {
        include!("processed_rustc_data_structures_src_owned_slice_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/parallel.rs
    pub mod rustc_data_structures_src_sync_parallel {
        include!("processed_rustc_data_structures_src_sync_parallel.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/packed.rs
    pub mod rustc_data_structures_rustc_data_structures_src_packed {
        include!("processed_rustc_data_structures_rustc_data_structures_src_packed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reference.rs
    pub mod rustc_data_structures_src_graph_reference {
        include!("processed_rustc_data_structures_src_graph_reference.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unord.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unord {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unord.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thinvec.rs
    pub mod rustc_data_structures_rustc_data_structures_src_thinvec {
        include!("processed_rustc_data_structures_rustc_data_structures_src_thinvec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unsupported.rs
    pub mod rustc_data_structures_src_flock_unsupported {
        include!("processed_rustc_data_structures_src_flock_unsupported.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/frozen.rs
    pub mod rustc_data_structures_rustc_data_structures_src_frozen {
        include!("processed_rustc_data_structures_rustc_data_structures_src_frozen.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/lock.rs
    pub mod rustc_data_structures_src_sync_lock {
        include!("processed_rustc_data_structures_src_sync_lock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/steal.rs
    pub mod rustc_data_structures_rustc_data_structures_src_steal {
        include!("processed_rustc_data_structures_rustc_data_structures_src_steal.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/linux.rs
    pub mod rustc_data_structures_src_flock_linux {
        include!("processed_rustc_data_structures_src_flock_linux.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/jobserver.rs
    pub mod rustc_data_structures_rustc_data_structures_src_jobserver {
        include!("processed_rustc_data_structures_rustc_data_structures_src_jobserver.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/set.rs
    pub mod rustc_data_structures_src_sso_set {
        include!("processed_rustc_data_structures_src_sso_set.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/freeze.rs
    pub mod rustc_data_structures_src_sync_freeze {
        include!("processed_rustc_data_structures_src_sync_freeze.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/snapshot_map/tests.rs
    pub mod rustc_data_structures_src_snapshot_map_tests {
        include!("processed_rustc_data_structures_src_snapshot_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/union_find/tests.rs
    pub mod rustc_data_structures_src_union_find_tests {
        include!("processed_rustc_data_structures_src_union_find_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/tests.rs
    pub mod rustc_data_structures_src_graph_tests {
        include!("processed_rustc_data_structures_src_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/mod.rs
    pub mod rustc_data_structures_src_sso_mod {
        include!("processed_rustc_data_structures_src_sso_mod.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/tests.rs
    pub mod rustc_data_structures_src_sorted_map_tests {
        include!("processed_rustc_data_structures_src_sorted_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/dominators/tests.rs
    pub mod rustc_data_structures_graph_dominators_tests {
        include!("processed_rustc_data_structures_graph_dominators_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/tests.rs
    pub mod rustc_data_structures_src_obligation_forest_tests {
        include!("processed_rustc_data_structures_src_obligation_forest_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/vec_graph/tests.rs
    pub mod rustc_data_structures_graph_vec_graph_tests {
        include!("processed_rustc_data_structures_graph_vec_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fingerprint/tests.rs
    pub mod rustc_data_structures_src_fingerprint_tests {
        include!("processed_rustc_data_structures_src_fingerprint_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/worker_local.rs
    pub mod rustc_data_structures_src_sync_worker_local {
        include!("processed_rustc_data_structures_src_sync_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/tagged_ptr/tests.rs
    pub mod rustc_data_structures_src_tagged_ptr_tests {
        include!("processed_rustc_data_structures_src_tagged_ptr_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/index_map.rs
    pub mod rustc_data_structures_src_sorted_map_index_map {
        include!("processed_rustc_data_structures_src_sorted_map_index_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/transitive_relation/tests.rs
    pub mod rustc_data_structures_src_transitive_relation_tests {
        include!("processed_rustc_data_structures_src_transitive_relation_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/vec_cache/tests.rs
    pub mod rustc_data_structures_src_vec_cache_tests {
        include!("processed_rustc_data_structures_src_vec_cache_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/intern/tests.rs
    pub mod rustc_data_structures_src_intern_tests {
        include!("processed_rustc_data_structures_src_intern_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flat_map_in_place.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flat_map_in_place {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flat_map_in_place.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reversed.rs
    pub mod rustc_data_structures_src_graph_reversed {
        include!("processed_rustc_data_structures_src_graph_reversed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stack.rs
    pub mod rustc_data_structures_rustc_data_structures_src_stack {
        include!("processed_rustc_data_structures_rustc_data_structures_src_stack.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/linked_graph/tests.rs
    pub mod rustc_data_structures_graph_linked_graph_tests {
        include!("processed_rustc_data_structures_graph_linked_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unix.rs
    pub mod rustc_data_structures_src_flock_unix {
        include!("processed_rustc_data_structures_src_flock_unix.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/graphviz.rs
    pub mod rustc_data_structures_src_obligation_forest_graphviz {
        include!("processed_rustc_data_structures_src_obligation_forest_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/scc/tests.rs
    pub mod rustc_data_structures_graph_scc_tests {
        include!("processed_rustc_data_structures_graph_scc_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stable_hasher/tests.rs
    pub mod rustc_data_structures_src_stable_hasher_tests {
        include!("processed_rustc_data_structures_src_stable_hasher_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/small_c_str/tests.rs
    pub mod rustc_data_structures_src_small_c_str_tests {
        include!("processed_rustc_data_structures_src_small_c_str_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/windows.rs
    pub mod rustc_data_structures_src_flock_windows {
        include!("processed_rustc_data_structures_src_flock_windows.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fx.rs
    pub mod rustc_data_structures_rustc_data_structures_src_fx {
        include!("processed_rustc_data_structures_rustc_data_structures_src_fx.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/iterate/tests.rs
    pub mod rustc_data_structures_graph_iterate_tests {
        include!("processed_rustc_data_structures_graph_iterate_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/aligned.rs
    pub mod rustc_data_structures_rustc_data_structures_src_aligned {
        include!("processed_rustc_data_structures_rustc_data_structures_src_aligned.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flock {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/vec.rs
    pub mod rustc_data_structures_src_sync_vec {
        include!("processed_rustc_data_structures_src_sync_vec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unhash.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unhash {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unhash.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/atomic_ref.rs
    pub mod rustc_data_structures_rustc_data_structures_src_atomic_ref {
        include!("processed_rustc_data_structures_rustc_data_structures_src_atomic_ref.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/temp_dir.rs
    pub mod rustc_data_structures_rustc_data_structures_src_temp_dir {
        include!("processed_rustc_data_structures_rustc_data_structures_src_temp_dir.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sharded.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sharded {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sharded.rs");
    }
}

// 3: rustc_index (9 files)
pub mod included_rustc_index {
    // Source: ../rust/compiler/rustc_index/src/idx.rs
    pub mod rustc_index_rustc_index_src_idx {
        include!("processed_rustc_index_rustc_index_src_idx.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/vec/tests.rs
    pub mod rustc_index_src_vec_tests {
        include!("processed_rustc_index_src_vec_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/slice.rs
    pub mod rustc_index_rustc_index_src_slice {
        include!("processed_rustc_index_rustc_index_src_slice.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/interval/tests.rs
    pub mod rustc_index_src_interval_tests {
        include!("processed_rustc_index_src_interval_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/bit_set/tests.rs
    pub mod rustc_index_src_bit_set_tests {
        include!("processed_rustc_index_src_bit_set_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/lib.rs
    pub mod rustc_index_rustc_index_src_lib {
        include!("processed_rustc_index_rustc_index_src_lib.rs");
    }
}

// 4: rustc_span (17 files)
pub mod included_rustc_span {
    // Source: ../rust/compiler/rustc_span/src/span_encoding.rs
    pub mod rustc_span_rustc_span_src_span_encoding {
        include!("processed_rustc_span_rustc_span_src_span_encoding.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/analyze_source_file/tests.rs
    pub mod rustc_span_src_analyze_source_file_tests {
        include!("processed_rustc_span_src_analyze_source_file_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/hygiene.rs
    pub mod rustc_span_rustc_span_src_hygiene {
        include!("processed_rustc_span_rustc_span_src_hygiene.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/source_map/tests.rs
    pub mod rustc_span_src_source_map_tests {
        include!("processed_rustc_span_src_source_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/profiling.rs
    pub mod rustc_span_rustc_span_src_profiling {
        include!("processed_rustc_span_rustc_span_src_profiling.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/caching_source_map_view.rs
    pub mod rustc_span_rustc_span_src_caching_source_map_view {
        include!("processed_rustc_span_rustc_span_src_caching_source_map_view.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edition.rs
    pub mod rustc_span_rustc_span_src_edition {
        include!("processed_rustc_span_rustc_span_src_edition.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/tests.rs
    pub mod rustc_span_rustc_span_src_tests {
        include!("processed_rustc_span_rustc_span_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edit_distance/tests.rs
    pub mod rustc_span_src_edit_distance_tests {
        include!("processed_rustc_span_src_edit_distance_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/symbol/tests.rs
    pub mod rustc_span_src_symbol_tests {
        include!("processed_rustc_span_src_symbol_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/def_id.rs
    pub mod rustc_span_rustc_span_src_def_id {
        include!("processed_rustc_span_rustc_span_src_def_id.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/fatal_error.rs
    pub mod rustc_span_rustc_span_src_fatal_error {
        include!("processed_rustc_span_rustc_span_src_fatal_error.rs");
    }
}

// 5: rustc_traits (9 files)
pub mod included_rustc_traits {
    // Source: ../rust/compiler/rustc_traits/src/normalize_projection_ty.rs
    pub mod rustc_traits_rustc_traits_src_normalize_projection_ty {
        include!("processed_rustc_traits_rustc_traits_src_normalize_projection_ty.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/codegen.rs
    pub mod rustc_traits_rustc_traits_src_codegen {
        include!("processed_rustc_traits_rustc_traits_src_codegen.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/lib.rs
    pub mod rustc_traits_rustc_traits_src_lib {
        include!("processed_rustc_traits_rustc_traits_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/type_op.rs
    pub mod rustc_traits_rustc_traits_src_type_op {
        include!("processed_rustc_traits_rustc_traits_src_type_op.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/implied_outlives_bounds.rs
    pub mod rustc_traits_rustc_traits_src_implied_outlives_bounds {
        include!("processed_rustc_traits_rustc_traits_src_implied_outlives_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/normalize_erasing_regions.rs
    pub mod rustc_traits_rustc_traits_src_normalize_erasing_regions {
        include!("processed_rustc_traits_rustc_traits_src_normalize_erasing_regions.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/coroutine_witnesses.rs
    pub mod rustc_traits_rustc_traits_src_coroutine_witnesses {
        include!("processed_rustc_traits_rustc_traits_src_coroutine_witnesses.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/dropck_outlives.rs
    pub mod rustc_traits_rustc_traits_src_dropck_outlives {
        include!("processed_rustc_traits_rustc_traits_src_dropck_outlives.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/evaluate_obligation.rs
    pub mod rustc_traits_rustc_traits_src_evaluate_obligation {
        include!("processed_rustc_traits_rustc_traits_src_evaluate_obligation.rs");
    }
}

// 6: rustc_incremental (12 files)
pub mod included_rustc_incremental {
    // Source: ../rust/compiler/rustc_incremental/src/errors.rs
    pub mod rustc_incremental_rustc_incremental_src_errors {
        include!("processed_rustc_incremental_rustc_incremental_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/save.rs
    pub mod rustc_incremental_src_persist_save {
        include!("processed_rustc_incremental_src_persist_save.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/dirty_clean.rs
    pub mod rustc_incremental_src_persist_dirty_clean {
        include!("processed_rustc_incremental_src_persist_dirty_clean.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/file_format.rs
    pub mod rustc_incremental_src_persist_file_format {
        include!("processed_rustc_incremental_src_persist_file_format.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/lib.rs
    pub mod rustc_incremental_rustc_incremental_src_lib {
        include!("processed_rustc_incremental_rustc_incremental_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/fs/tests.rs
    pub mod rustc_incremental_persist_fs_tests {
        include!("processed_rustc_incremental_persist_fs_tests.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/data.rs
    pub mod rustc_incremental_src_persist_data {
        include!("processed_rustc_incremental_src_persist_data.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/work_product.rs
    pub mod rustc_incremental_src_persist_work_product {
        include!("processed_rustc_incremental_src_persist_work_product.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/mod.rs
    pub mod rustc_incremental_src_persist_mod {
        include!("processed_rustc_incremental_src_persist_mod.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/assert_dep_graph.rs
    pub mod rustc_incremental_rustc_incremental_src_assert_dep_graph {
        include!("processed_rustc_incremental_rustc_incremental_src_assert_dep_graph.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/load.rs
    pub mod rustc_incremental_src_persist_load {
        include!("processed_rustc_incremental_src_persist_load.rs");
    }
}

// 7: rustc_attr_parsing (38 files)
pub mod included_rustc_attr_parsing {
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/link_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_link_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_link_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/allow_unstable.rs
    pub mod rustc_attr_parsing_src_attributes_allow_unstable {
        include!("processed_rustc_attr_parsing_src_attributes_allow_unstable.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/no_implicit_prelude.rs
    pub mod rustc_attr_parsing_src_attributes_no_implicit_prelude {
        include!("processed_rustc_attr_parsing_src_attributes_no_implicit_prelude.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/proc_macro_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_proc_macro_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_proc_macro_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/cfg_old.rs
    pub mod rustc_attr_parsing_src_attributes_cfg_old {
        include!("processed_rustc_attr_parsing_src_attributes_cfg_old.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/crate_level.rs
    pub mod rustc_attr_parsing_src_attributes_crate_level {
        include!("processed_rustc_attr_parsing_src_attributes_crate_level.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/context.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_context {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_context.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/codegen_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_codegen_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_codegen_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/lib.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_lib {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/prototype.rs
    pub mod rustc_attr_parsing_src_attributes_prototype {
        include!("processed_rustc_attr_parsing_src_attributes_prototype.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/lints.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_lints {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_lints.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/transparency.rs
    pub mod rustc_attr_parsing_src_attributes_transparency {
        include!("processed_rustc_attr_parsing_src_attributes_transparency.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/semantics.rs
    pub mod rustc_attr_parsing_src_attributes_semantics {
        include!("processed_rustc_attr_parsing_src_attributes_semantics.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/macro_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_macro_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_macro_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/loop_match.rs
    pub mod rustc_attr_parsing_src_attributes_loop_match {
        include!("processed_rustc_attr_parsing_src_attributes_loop_match.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/body.rs
    pub mod rustc_attr_parsing_src_attributes_body {
        include!("processed_rustc_attr_parsing_src_attributes_body.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/must_use.rs
    pub mod rustc_attr_parsing_src_attributes_must_use {
        include!("processed_rustc_attr_parsing_src_attributes_must_use.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/path.rs
    pub mod rustc_attr_parsing_src_attributes_path {
        include!("processed_rustc_attr_parsing_src_attributes_path.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/mod.rs
    pub mod rustc_attr_parsing_src_attributes_mod {
        include!("processed_rustc_attr_parsing_src_attributes_mod.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/confusables.rs
    pub mod rustc_attr_parsing_src_attributes_confusables {
        include!("processed_rustc_attr_parsing_src_attributes_confusables.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/test_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_test_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_test_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/session_diagnostics.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_session_diagnostics {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_session_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/util.rs
    pub mod rustc_attr_parsing_src_attributes_util {
        include!("processed_rustc_attr_parsing_src_attributes_util.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/non_exhaustive.rs
    pub mod rustc_attr_parsing_src_attributes_non_exhaustive {
        include!("processed_rustc_attr_parsing_src_attributes_non_exhaustive.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/lint_helpers.rs
    pub mod rustc_attr_parsing_src_attributes_lint_helpers {
        include!("processed_rustc_attr_parsing_src_attributes_lint_helpers.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/cfg.rs
    pub mod rustc_attr_parsing_src_attributes_cfg {
        include!("processed_rustc_attr_parsing_src_attributes_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/stability.rs
    pub mod rustc_attr_parsing_src_attributes_stability {
        include!("processed_rustc_attr_parsing_src_attributes_stability.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/prelude.rs
    pub mod rustc_attr_parsing_src_attributes_prelude {
        include!("processed_rustc_attr_parsing_src_attributes_prelude.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/traits.rs
    pub mod rustc_attr_parsing_src_attributes_traits {
        include!("processed_rustc_attr_parsing_src_attributes_traits.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/deprecation.rs
    pub mod rustc_attr_parsing_src_attributes_deprecation {
        include!("processed_rustc_attr_parsing_src_attributes_deprecation.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/rustc_internal.rs
    pub mod rustc_attr_parsing_src_attributes_rustc_internal {
        include!("processed_rustc_attr_parsing_src_attributes_rustc_internal.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/parser.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_parser {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_parser.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/dummy.rs
    pub mod rustc_attr_parsing_src_attributes_dummy {
        include!("processed_rustc_attr_parsing_src_attributes_dummy.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/inline.rs
    pub mod rustc_attr_parsing_src_attributes_inline {
        include!("processed_rustc_attr_parsing_src_attributes_inline.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/repr.rs
    pub mod rustc_attr_parsing_src_attributes_repr {
        include!("processed_rustc_attr_parsing_src_attributes_repr.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/interface.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_interface {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_interface.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/target_checking.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_target_checking {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_target_checking.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/validate_attr.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_validate_attr {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_validate_attr.rs");
    }
}

// 8: rustc_hir (23 files)
pub mod included_rustc_hir {
    // Source: ../rust/compiler/rustc_hir/src/weak_lang_items.rs
    pub mod rustc_hir_rustc_hir_src_weak_lang_items {
        include!("processed_rustc_hir_rustc_hir_src_weak_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/diagnostic_items.rs
    pub mod rustc_hir_rustc_hir_src_diagnostic_items {
        include!("processed_rustc_hir_rustc_hir_src_diagnostic_items.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/lints.rs
    pub mod rustc_hir_rustc_hir_src_lints {
        include!("processed_rustc_hir_rustc_hir_src_lints.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/target.rs
    pub mod rustc_hir_rustc_hir_src_target {
        include!("processed_rustc_hir_rustc_hir_src_target.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/intravisit.rs
    pub mod rustc_hir_rustc_hir_src_intravisit {
        include!("processed_rustc_hir_rustc_hir_src_intravisit.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/limit.rs
    pub mod rustc_hir_rustc_hir_src_limit {
        include!("processed_rustc_hir_rustc_hir_src_limit.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/pat_util.rs
    pub mod rustc_hir_rustc_hir_src_pat_util {
        include!("processed_rustc_hir_rustc_hir_src_pat_util.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/attrs/encode_cross_crate.rs
    pub mod rustc_hir_src_attrs_encode_cross_crate {
        include!("processed_rustc_hir_src_attrs_encode_cross_crate.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/hir/tests.rs
    pub mod rustc_hir_src_hir_tests {
        include!("processed_rustc_hir_src_hir_tests.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/definitions.rs
    pub mod rustc_hir_rustc_hir_src_definitions {
        include!("processed_rustc_hir_rustc_hir_src_definitions.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/stable_hash_impls.rs
    pub mod rustc_hir_rustc_hir_src_stable_hash_impls {
        include!("processed_rustc_hir_rustc_hir_src_stable_hash_impls.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/attrs/pretty_printing.rs
    pub mod rustc_hir_src_attrs_pretty_printing {
        include!("processed_rustc_hir_src_attrs_pretty_printing.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/tests.rs
    pub mod rustc_hir_rustc_hir_src_tests {
        include!("processed_rustc_hir_rustc_hir_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/lang_items.rs
    pub mod rustc_hir_rustc_hir_src_lang_items {
        include!("processed_rustc_hir_rustc_hir_src_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/stability.rs
    pub mod rustc_hir_rustc_hir_src_stability {
        include!("processed_rustc_hir_rustc_hir_src_stability.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/attrs/data_structures.rs
    pub mod rustc_hir_src_attrs_data_structures {
        include!("processed_rustc_hir_src_attrs_data_structures.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/version.rs
    pub mod rustc_hir_rustc_hir_src_version {
        include!("processed_rustc_hir_rustc_hir_src_version.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/arena.rs
    pub mod rustc_hir_rustc_hir_src_arena {
        include!("processed_rustc_hir_rustc_hir_src_arena.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/attrs/mod.rs
    pub mod rustc_hir_src_attrs_mod {
        include!("processed_rustc_hir_src_attrs_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/def.rs
    pub mod rustc_hir_rustc_hir_src_def {
        include!("processed_rustc_hir_rustc_hir_src_def.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/def_path_hash_map.rs
    pub mod rustc_hir_rustc_hir_src_def_path_hash_map {
        include!("processed_rustc_hir_rustc_hir_src_def_path_hash_map.rs");
    }
}

// 9: rustc_error_codes (1 files)
pub mod included_rustc_error_codes {
    // Source: ../rust/compiler/rustc_error_codes/src/lib.rs
    pub mod rustc_error_codes_rustc_error_codes_src_lib {
        include!("processed_rustc_error_codes_rustc_error_codes_src_lib.rs");
    }
}

// 10: rustc_hir_id (1 files)
pub mod included_rustc_hir_id {
    // Source: ../rust/compiler/rustc_hir_id/src/lib.rs
    pub mod rustc_hir_id_rustc_hir_id_src_lib {
        include!("processed_rustc_hir_id_rustc_hir_id_src_lib.rs");
    }
}

// 11: rustc_ty_utils (17 files)
pub mod included_rustc_ty_utils {
    // Source: ../rust/compiler/rustc_ty_utils/src/representability.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_representability {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_representability.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/common_traits.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_common_traits {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_common_traits.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/instance.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_instance {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_instance.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/sig_types.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_sig_types {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_sig_types.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/nested_bodies.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_nested_bodies {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_nested_bodies.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/assoc.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_assoc {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_assoc.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/structural_match.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_structural_match {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_structural_match.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/opaque_types.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_opaque_types {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/lib.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_lib {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/implied_bounds.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_implied_bounds {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_implied_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/errors.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_errors {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/consts.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_consts {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_consts.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/layout/invariant.rs
    pub mod rustc_ty_utils_src_layout_invariant {
        include!("processed_rustc_ty_utils_src_layout_invariant.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/layout.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_layout {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_layout.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/abi.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_abi {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_abi.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/needs_drop.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_needs_drop {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_needs_drop.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/ty.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_ty {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_ty.rs");
    }
}

// 12: rustc_feature (6 files)
pub mod included_rustc_feature {
    // Source: ../rust/compiler/rustc_feature/src/unstable.rs
    pub mod rustc_feature_rustc_feature_src_unstable {
        include!("processed_rustc_feature_rustc_feature_src_unstable.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/builtin_attrs.rs
    pub mod rustc_feature_rustc_feature_src_builtin_attrs {
        include!("processed_rustc_feature_rustc_feature_src_builtin_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/accepted.rs
    pub mod rustc_feature_rustc_feature_src_accepted {
        include!("processed_rustc_feature_rustc_feature_src_accepted.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/tests.rs
    pub mod rustc_feature_rustc_feature_src_tests {
        include!("processed_rustc_feature_rustc_feature_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/removed.rs
    pub mod rustc_feature_rustc_feature_src_removed {
        include!("processed_rustc_feature_rustc_feature_src_removed.rs");
    }
}

// 13: rustc_infer (39 files)
pub mod included_rustc_infer {
    // Source: ../rust/compiler/rustc_infer/src/infer/region_constraints/leak_check.rs
    pub mod rustc_infer_infer_region_constraints_leak_check {
        include!("processed_rustc_infer_infer_region_constraints_leak_check.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/lib.rs
    pub mod rustc_infer_rustc_infer_src_lib {
        include!("processed_rustc_infer_rustc_infer_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/context.rs
    pub mod rustc_infer_src_infer_context {
        include!("processed_rustc_infer_src_infer_context.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/projection.rs
    pub mod rustc_infer_src_infer_projection {
        include!("processed_rustc_infer_src_infer_projection.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/mod.rs
    pub mod rustc_infer_src_traits_mod {
        include!("processed_rustc_infer_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/mod.rs
    pub mod rustc_infer_infer_relate_mod {
        include!("processed_rustc_infer_infer_relate_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/lattice.rs
    pub mod rustc_infer_infer_relate_lattice {
        include!("processed_rustc_infer_infer_relate_lattice.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/test_type_match.rs
    pub mod rustc_infer_infer_outlives_test_type_match {
        include!("processed_rustc_infer_infer_outlives_test_type_match.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/at.rs
    pub mod rustc_infer_src_infer_at {
        include!("processed_rustc_infer_src_infer_at.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/project.rs
    pub mod rustc_infer_src_traits_project {
        include!("processed_rustc_infer_src_traits_project.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/errors.rs
    pub mod rustc_infer_rustc_infer_src_errors {
        include!("processed_rustc_infer_rustc_infer_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/freshen.rs
    pub mod rustc_infer_src_infer_freshen {
        include!("processed_rustc_infer_src_infer_freshen.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/instantiate.rs
    pub mod rustc_infer_infer_canonical_instantiate {
        include!("processed_rustc_infer_infer_canonical_instantiate.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs
    pub mod rustc_infer_infer_lexical_region_resolve_mod {
        include!("processed_rustc_infer_infer_lexical_region_resolve_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/region_constraints/mod.rs
    pub mod rustc_infer_infer_region_constraints_mod {
        include!("processed_rustc_infer_infer_region_constraints_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/for_liveness.rs
    pub mod rustc_infer_infer_outlives_for_liveness {
        include!("processed_rustc_infer_infer_outlives_for_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/engine.rs
    pub mod rustc_infer_src_traits_engine {
        include!("processed_rustc_infer_src_traits_engine.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/unify_key.rs
    pub mod rustc_infer_src_infer_unify_key {
        include!("processed_rustc_infer_src_infer_unify_key.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/structural_impls.rs
    pub mod rustc_infer_src_traits_structural_impls {
        include!("processed_rustc_infer_src_traits_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/mod.rs
    pub mod rustc_infer_src_infer_mod {
        include!("processed_rustc_infer_src_infer_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/canonicalizer.rs
    pub mod rustc_infer_infer_canonical_canonicalizer {
        include!("processed_rustc_infer_infer_canonical_canonicalizer.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/env.rs
    pub mod rustc_infer_infer_outlives_env {
        include!("processed_rustc_infer_infer_outlives_env.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/verify.rs
    pub mod rustc_infer_infer_outlives_verify {
        include!("processed_rustc_infer_infer_outlives_verify.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/mod.rs
    pub mod rustc_infer_infer_outlives_mod {
        include!("processed_rustc_infer_infer_outlives_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/resolve.rs
    pub mod rustc_infer_src_infer_resolve {
        include!("processed_rustc_infer_src_infer_resolve.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/generalize.rs
    pub mod rustc_infer_infer_relate_generalize {
        include!("processed_rustc_infer_infer_relate_generalize.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/query_response.rs
    pub mod rustc_infer_infer_canonical_query_response {
        include!("processed_rustc_infer_infer_canonical_query_response.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/opaque_types/table.rs
    pub mod rustc_infer_infer_opaque_types_table {
        include!("processed_rustc_infer_infer_opaque_types_table.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/higher_ranked.rs
    pub mod rustc_infer_infer_relate_higher_ranked {
        include!("processed_rustc_infer_infer_relate_higher_ranked.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/free_regions.rs
    pub mod rustc_infer_src_infer_free_regions {
        include!("processed_rustc_infer_src_infer_free_regions.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/snapshot/fudge.rs
    pub mod rustc_infer_infer_snapshot_fudge {
        include!("processed_rustc_infer_infer_snapshot_fudge.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/opaque_types/mod.rs
    pub mod rustc_infer_infer_opaque_types_mod {
        include!("processed_rustc_infer_infer_opaque_types_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/type_relating.rs
    pub mod rustc_infer_infer_relate_type_relating {
        include!("processed_rustc_infer_infer_relate_type_relating.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/snapshot/undo_log.rs
    pub mod rustc_infer_infer_snapshot_undo_log {
        include!("processed_rustc_infer_infer_snapshot_undo_log.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/obligations.rs
    pub mod rustc_infer_infer_outlives_obligations {
        include!("processed_rustc_infer_infer_outlives_obligations.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/snapshot/mod.rs
    pub mod rustc_infer_infer_snapshot_mod {
        include!("processed_rustc_infer_infer_snapshot_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/util.rs
    pub mod rustc_infer_src_traits_util {
        include!("processed_rustc_infer_src_traits_util.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/type_variable.rs
    pub mod rustc_infer_src_infer_type_variable {
        include!("processed_rustc_infer_src_infer_type_variable.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/mod.rs
    pub mod rustc_infer_infer_canonical_mod {
        include!("processed_rustc_infer_infer_canonical_mod.rs");
    }
}

// 14: rustc_middle (113 files)
pub mod included_rustc_middle {
    // Source: ../rust/compiler/rustc_middle/src/hir/place.rs
    pub mod rustc_middle_src_hir_place {
        include!("processed_rustc_middle_src_hir_place.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/structural_impls.rs
    pub mod rustc_middle_src_traits_structural_impls {
        include!("processed_rustc_middle_src_traits_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/layout.rs
    pub mod rustc_middle_src_ty_layout {
        include!("processed_rustc_middle_src_ty_layout.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/impls_ty.rs
    pub mod rustc_middle_src_ty_impls_ty {
        include!("processed_rustc_middle_src_ty_impls_ty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/codegen_fn_attrs.rs
    pub mod rustc_middle_src_middle_codegen_fn_attrs {
        include!("processed_rustc_middle_src_middle_codegen_fn_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/map.rs
    pub mod rustc_middle_src_hir_map {
        include!("processed_rustc_middle_src_hir_map.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/value.rs
    pub mod rustc_middle_mir_interpret_value {
        include!("processed_rustc_middle_mir_interpret_value.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/terminator.rs
    pub mod rustc_middle_src_mir_terminator {
        include!("processed_rustc_middle_src_mir_terminator.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/generic_graph.rs
    pub mod rustc_middle_src_mir_generic_graph {
        include!("processed_rustc_middle_src_mir_generic_graph.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/coverage.rs
    pub mod rustc_middle_src_mir_coverage {
        include!("processed_rustc_middle_src_mir_coverage.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/assoc.rs
    pub mod rustc_middle_src_ty_assoc {
        include!("processed_rustc_middle_src_ty_assoc.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/trait_def.rs
    pub mod rustc_middle_src_ty_trait_def {
        include!("processed_rustc_middle_src_ty_trait_def.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/visit.rs
    pub mod rustc_middle_src_mir_visit {
        include!("processed_rustc_middle_src_mir_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/abstract_const.rs
    pub mod rustc_middle_src_ty_abstract_const {
        include!("processed_rustc_middle_src_ty_abstract_const.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/mod.rs
    pub mod rustc_middle_src_middle_mod {
        include!("processed_rustc_middle_src_middle_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/mono.rs
    pub mod rustc_middle_src_mir_mono {
        include!("processed_rustc_middle_src_mir_mono.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/stability.rs
    pub mod rustc_middle_src_middle_stability {
        include!("processed_rustc_middle_src_middle_stability.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/context/tls.rs
    pub mod rustc_middle_ty_context_tls {
        include!("processed_rustc_middle_ty_context_tls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hooks/mod.rs
    pub mod rustc_middle_src_hooks_mod {
        include!("processed_rustc_middle_src_hooks_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/region.rs
    pub mod rustc_middle_src_middle_region {
        include!("processed_rustc_middle_src_middle_region.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/error.rs
    pub mod rustc_middle_mir_interpret_error {
        include!("processed_rustc_middle_mir_interpret_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/error.rs
    pub mod rustc_middle_src_ty_error {
        include!("processed_rustc_middle_src_ty_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/erase_regions.rs
    pub mod rustc_middle_src_ty_erase_regions {
        include!("processed_rustc_middle_src_ty_erase_regions.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/significant_drop_order.rs
    pub mod rustc_middle_src_ty_significant_drop_order {
        include!("processed_rustc_middle_src_ty_significant_drop_order.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/consts.rs
    pub mod rustc_middle_src_mir_consts {
        include!("processed_rustc_middle_src_mir_consts.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/statement.rs
    pub mod rustc_middle_src_mir_statement {
        include!("processed_rustc_middle_src_mir_statement.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/print/pretty.rs
    pub mod rustc_middle_ty_print_pretty {
        include!("processed_rustc_middle_ty_print_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/fold.rs
    pub mod rustc_middle_src_ty_fold {
        include!("processed_rustc_middle_src_ty_fold.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/mod.rs
    pub mod rustc_middle_src_ty_mod {
        include!("processed_rustc_middle_src_ty_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/vtable.rs
    pub mod rustc_middle_src_ty_vtable {
        include!("processed_rustc_middle_src_ty_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/rvalue_scopes.rs
    pub mod rustc_middle_src_ty_rvalue_scopes {
        include!("processed_rustc_middle_src_ty_rvalue_scopes.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/adt.rs
    pub mod rustc_middle_src_ty_adt {
        include!("processed_rustc_middle_src_ty_adt.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/pointer.rs
    pub mod rustc_middle_mir_interpret_pointer {
        include!("processed_rustc_middle_mir_interpret_pointer.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/dep_graph/dep_node.rs
    pub mod rustc_middle_src_dep_graph_dep_node {
        include!("processed_rustc_middle_src_dep_graph_dep_node.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/visit.rs
    pub mod rustc_middle_src_ty_visit {
        include!("processed_rustc_middle_src_ty_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/thir.rs
    pub mod rustc_middle_rustc_middle_src_thir {
        include!("processed_rustc_middle_rustc_middle_src_thir.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs
    pub mod rustc_middle_src_ty_normalize_erasing_regions {
        include!("processed_rustc_middle_src_ty_normalize_erasing_regions.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/arena.rs
    pub mod rustc_middle_rustc_middle_src_arena {
        include!("processed_rustc_middle_rustc_middle_src_arena.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/util/mod.rs
    pub mod rustc_middle_src_util_mod {
        include!("processed_rustc_middle_src_util_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/exported_symbols.rs
    pub mod rustc_middle_src_middle_exported_symbols {
        include!("processed_rustc_middle_src_middle_exported_symbols.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/fast_reject.rs
    pub mod rustc_middle_src_ty_fast_reject {
        include!("processed_rustc_middle_src_ty_fast_reject.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/util.rs
    pub mod rustc_middle_src_ty_util {
        include!("processed_rustc_middle_src_ty_util.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/pattern.rs
    pub mod rustc_middle_src_ty_pattern {
        include!("processed_rustc_middle_src_ty_pattern.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/init_mask/tests.rs
    pub mod rustc_middle_allocation_init_mask_tests {
        include!("processed_rustc_middle_allocation_init_mask_tests.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/generic_args.rs
    pub mod rustc_middle_src_ty_generic_args {
        include!("processed_rustc_middle_src_ty_generic_args.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/print/mod.rs
    pub mod rustc_middle_ty_print_mod {
        include!("processed_rustc_middle_ty_print_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/resolve_bound_vars.rs
    pub mod rustc_middle_src_middle_resolve_bound_vars {
        include!("processed_rustc_middle_src_middle_resolve_bound_vars.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/thir/visit.rs
    pub mod rustc_middle_src_thir_visit {
        include!("processed_rustc_middle_src_thir_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/intrinsic.rs
    pub mod rustc_middle_src_ty_intrinsic {
        include!("processed_rustc_middle_src_ty_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/arena_cached.rs
    pub mod rustc_middle_src_query_arena_cached {
        include!("processed_rustc_middle_src_query_arena_cached.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/generics.rs
    pub mod rustc_middle_src_ty_generics {
        include!("processed_rustc_middle_src_ty_generics.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/structural_impls.rs
    pub mod rustc_middle_src_ty_structural_impls {
        include!("processed_rustc_middle_src_ty_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/closure.rs
    pub mod rustc_middle_src_ty_closure {
        include!("processed_rustc_middle_src_ty_closure.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/select.rs
    pub mod rustc_middle_src_traits_select {
        include!("processed_rustc_middle_src_traits_select.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/codec.rs
    pub mod rustc_middle_src_ty_codec {
        include!("processed_rustc_middle_src_ty_codec.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/typeck_results.rs
    pub mod rustc_middle_src_ty_typeck_results {
        include!("processed_rustc_middle_src_ty_typeck_results.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/query.rs
    pub mod rustc_middle_src_mir_query {
        include!("processed_rustc_middle_src_mir_query.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/lang_items.rs
    pub mod rustc_middle_src_middle_lang_items {
        include!("processed_rustc_middle_src_middle_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/debugger_visualizer.rs
    pub mod rustc_middle_src_middle_debugger_visualizer {
        include!("processed_rustc_middle_src_middle_debugger_visualizer.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/adjustment.rs
    pub mod rustc_middle_src_ty_adjustment {
        include!("processed_rustc_middle_src_ty_adjustment.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/tests.rs
    pub mod rustc_middle_rustc_middle_src_tests {
        include!("processed_rustc_middle_rustc_middle_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/privacy.rs
    pub mod rustc_middle_src_middle_privacy {
        include!("processed_rustc_middle_src_middle_privacy.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/diagnostics.rs
    pub mod rustc_middle_src_ty_diagnostics {
        include!("processed_rustc_middle_src_ty_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/graphviz.rs
    pub mod rustc_middle_src_mir_graphviz {
        include!("processed_rustc_middle_src_mir_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/metadata.rs
    pub mod rustc_middle_rustc_middle_src_metadata {
        include!("processed_rustc_middle_rustc_middle_src_metadata.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/values.rs
    pub mod rustc_middle_rustc_middle_src_values {
        include!("processed_rustc_middle_rustc_middle_src_values.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/infer/canonical.rs
    pub mod rustc_middle_src_infer_canonical {
        include!("processed_rustc_middle_src_infer_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/solve.rs
    pub mod rustc_middle_src_traits_solve {
        include!("processed_rustc_middle_src_traits_solve.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/sty.rs
    pub mod rustc_middle_src_ty_sty {
        include!("processed_rustc_middle_src_ty_sty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/list.rs
    pub mod rustc_middle_src_ty_list {
        include!("processed_rustc_middle_src_ty_list.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/generic_graphviz.rs
    pub mod rustc_middle_src_mir_generic_graphviz {
        include!("processed_rustc_middle_src_mir_generic_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/queries.rs
    pub mod rustc_middle_mir_interpret_queries {
        include!("processed_rustc_middle_mir_interpret_queries.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/region.rs
    pub mod rustc_middle_src_ty_region {
        include!("processed_rustc_middle_src_ty_region.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/inhabitedness/mod.rs
    pub mod rustc_middle_ty_inhabitedness_mod {
        include!("processed_rustc_middle_ty_inhabitedness_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/error.rs
    pub mod rustc_middle_rustc_middle_src_error {
        include!("processed_rustc_middle_rustc_middle_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/syntax.rs
    pub mod rustc_middle_src_mir_syntax {
        include!("processed_rustc_middle_src_mir_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/mod.rs
    pub mod rustc_middle_src_hir_mod {
        include!("processed_rustc_middle_src_hir_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/valtree.rs
    pub mod rustc_middle_ty_consts_valtree {
        include!("processed_rustc_middle_ty_consts_valtree.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation.rs
    pub mod rustc_middle_mir_interpret_allocation {
        include!("processed_rustc_middle_mir_interpret_allocation.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/elaborate_impl.rs
    pub mod rustc_middle_src_ty_elaborate_impl {
        include!("processed_rustc_middle_src_ty_elaborate_impl.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/instance.rs
    pub mod rustc_middle_src_ty_instance {
        include!("processed_rustc_middle_src_ty_instance.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/relate.rs
    pub mod rustc_middle_src_ty_relate {
        include!("processed_rustc_middle_src_ty_relate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/lint.rs
    pub mod rustc_middle_rustc_middle_src_lint {
        include!("processed_rustc_middle_rustc_middle_src_lint.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/on_disk_cache.rs
    pub mod rustc_middle_src_query_on_disk_cache {
        include!("processed_rustc_middle_src_query_on_disk_cache.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/provenance_map.rs
    pub mod rustc_middle_interpret_allocation_provenance_map {
        include!("processed_rustc_middle_interpret_allocation_provenance_map.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/mod.rs
    pub mod rustc_middle_src_traits_mod {
        include!("processed_rustc_middle_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/inhabitedness/inhabited_predicate.rs
    pub mod rustc_middle_ty_inhabitedness_inhabited_predicate {
        include!("processed_rustc_middle_ty_inhabitedness_inhabited_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/basic_blocks.rs
    pub mod rustc_middle_src_mir_basic_blocks {
        include!("processed_rustc_middle_src_mir_basic_blocks.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/pretty.rs
    pub mod rustc_middle_src_mir_pretty {
        include!("processed_rustc_middle_src_mir_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/dep_graph/mod.rs
    pub mod rustc_middle_src_dep_graph_mod {
        include!("processed_rustc_middle_src_dep_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/query.rs
    pub mod rustc_middle_src_traits_query {
        include!("processed_rustc_middle_src_traits_query.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/context.rs
    pub mod rustc_middle_src_ty_context {
        include!("processed_rustc_middle_src_ty_context.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/predicate.rs
    pub mod rustc_middle_src_ty_predicate {
        include!("processed_rustc_middle_src_ty_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/traversal.rs
    pub mod rustc_middle_src_mir_traversal {
        include!("processed_rustc_middle_src_mir_traversal.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/kind.rs
    pub mod rustc_middle_ty_consts_kind {
        include!("processed_rustc_middle_ty_consts_kind.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/nested_filter.rs
    pub mod rustc_middle_src_hir_nested_filter {
        include!("processed_rustc_middle_src_hir_nested_filter.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/opaque_types.rs
    pub mod rustc_middle_src_ty_opaque_types {
        include!("processed_rustc_middle_src_ty_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/specialization_graph.rs
    pub mod rustc_middle_src_traits_specialization_graph {
        include!("processed_rustc_middle_src_traits_specialization_graph.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/mod.rs
    pub mod rustc_middle_mir_interpret_mod {
        include!("processed_rustc_middle_mir_interpret_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/dependency_format.rs
    pub mod rustc_middle_src_middle_dependency_format {
        include!("processed_rustc_middle_src_middle_dependency_format.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/loops.rs
    pub mod rustc_middle_src_mir_loops {
        include!("processed_rustc_middle_src_mir_loops.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/int.rs
    pub mod rustc_middle_ty_consts_int {
        include!("processed_rustc_middle_ty_consts_int.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/plumbing.rs
    pub mod rustc_middle_src_query_plumbing {
        include!("processed_rustc_middle_src_query_plumbing.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts.rs
    pub mod rustc_middle_src_ty_consts {
        include!("processed_rustc_middle_src_ty_consts.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/macros.rs
    pub mod rustc_middle_rustc_middle_src_macros {
        include!("processed_rustc_middle_rustc_middle_src_macros.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/erase.rs
    pub mod rustc_middle_src_query_erase {
        include!("processed_rustc_middle_src_query_erase.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/cast.rs
    pub mod rustc_middle_src_ty_cast {
        include!("processed_rustc_middle_src_ty_cast.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/util/bug.rs
    pub mod rustc_middle_src_util_bug {
        include!("processed_rustc_middle_src_util_bug.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/mod.rs
    pub mod rustc_middle_src_query_mod {
        include!("processed_rustc_middle_src_query_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/keys.rs
    pub mod rustc_middle_src_query_keys {
        include!("processed_rustc_middle_src_query_keys.rs");
    }
}

// 15: rustc_driver_impl (6 files)
pub mod included_rustc_driver_impl {
    // Source: ../rust/compiler/rustc_driver_impl/src/signal_handler.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_signal_handler {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_signal_handler.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/lib.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_lib {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/args.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_args {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_args.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/print.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_print {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_print.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/pretty.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_pretty {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/session_diagnostics.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_session_diagnostics {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_session_diagnostics.rs");
    }
}

// 16: rustc_index_macros (2 files)
pub mod included_rustc_index_macros {
    // Source: ../rust/compiler/rustc_index_macros/src/newtype.rs
    pub mod rustc_index_macros_rustc_index_macros_src_newtype {
        include!("processed_rustc_index_macros_rustc_index_macros_src_newtype.rs");
    }
    // Source: ../rust/compiler/rustc_index_macros/src/lib.rs
    pub mod rustc_index_macros_rustc_index_macros_src_lib {
        include!("processed_rustc_index_macros_rustc_index_macros_src_lib.rs");
    }
}

// 17: rustc_hashes (1 files)
pub mod included_rustc_hashes {
    // Source: ../rust/compiler/rustc_hashes/src/lib.rs
    pub mod rustc_hashes_rustc_hashes_src_lib {
        include!("processed_rustc_hashes_rustc_hashes_src_lib.rs");
    }
}

// 18: rustc_pattern_analysis (14 files)
pub mod included_rustc_pattern_analysis {
    // Source: ../rust/compiler/rustc_pattern_analysis/tests/exhaustiveness.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_tests_exhaustiveness {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_tests_exhaustiveness.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/lib.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_lib {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/constructor.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_constructor {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_constructor.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/usefulness.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_usefulness {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_usefulness.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/tests/complexity.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_tests_complexity {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_tests_complexity.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/rustc.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_rustc {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_rustc.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/lints.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_lints {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_lints.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/tests/common/mod.rs
    pub mod rustc_pattern_analysis_tests_common_mod {
        include!("processed_rustc_pattern_analysis_tests_common_mod.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/errors.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_errors {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/pat.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_pat {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_pat.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/checks.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_checks {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_checks.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/rustc/print.rs
    pub mod rustc_pattern_analysis_src_rustc_print {
        include!("processed_rustc_pattern_analysis_src_rustc_print.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/tests/intersection.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_tests_intersection {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_tests_intersection.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/pat_column.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_pat_column {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_pat_column.rs");
    }
}

// 19: rustc_type_ir_macros (1 files)
pub mod included_rustc_type_ir_macros {
    // Source: ../rust/compiler/rustc_type_ir_macros/src/lib.rs
    pub mod rustc_type_ir_macros_rustc_type_ir_macros_src_lib {
        include!("processed_rustc_type_ir_macros_rustc_type_ir_macros_src_lib.rs");
    }
}

// 20: rustc_ast_passes (4 files)
pub mod included_rustc_ast_passes {
    // Source: ../rust/compiler/rustc_ast_passes/src/ast_validation.rs
    pub mod rustc_ast_passes_rustc_ast_passes_src_ast_validation {
        include!("processed_rustc_ast_passes_rustc_ast_passes_src_ast_validation.rs");
    }
    // Source: ../rust/compiler/rustc_ast_passes/src/lib.rs
    pub mod rustc_ast_passes_rustc_ast_passes_src_lib {
        include!("processed_rustc_ast_passes_rustc_ast_passes_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ast_passes/src/feature_gate.rs
    pub mod rustc_ast_passes_rustc_ast_passes_src_feature_gate {
        include!("processed_rustc_ast_passes_rustc_ast_passes_src_feature_gate.rs");
    }
    // Source: ../rust/compiler/rustc_ast_passes/src/errors.rs
    pub mod rustc_ast_passes_rustc_ast_passes_src_errors {
        include!("processed_rustc_ast_passes_rustc_ast_passes_src_errors.rs");
    }
}

// 21: rustc_ast_ir (2 files)
pub mod included_rustc_ast_ir {
    // Source: ../rust/compiler/rustc_ast_ir/src/visit.rs
    pub mod rustc_ast_ir_rustc_ast_ir_src_visit {
        include!("processed_rustc_ast_ir_rustc_ast_ir_src_visit.rs");
    }
    // Source: ../rust/compiler/rustc_ast_ir/src/lib.rs
    pub mod rustc_ast_ir_rustc_ast_ir_src_lib {
        include!("processed_rustc_ast_ir_rustc_ast_ir_src_lib.rs");
    }
}

// 22: rustc_driver (1 files)
pub mod included_rustc_driver {
    // Source: ../rust/compiler/rustc_driver/src/lib.rs
    pub mod rustc_driver_rustc_driver_src_lib {
        include!("processed_rustc_driver_rustc_driver_src_lib.rs");
    }
}

// 23: rustc_ast_pretty (11 files)
pub mod included_rustc_ast_pretty {
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/state/item.rs
    pub mod rustc_ast_pretty_pprust_state_item {
        include!("processed_rustc_ast_pretty_pprust_state_item.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pp.rs
    pub mod rustc_ast_pretty_rustc_ast_pretty_src_pp {
        include!("processed_rustc_ast_pretty_rustc_ast_pretty_src_pp.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/tests.rs
    pub mod rustc_ast_pretty_src_pprust_tests {
        include!("processed_rustc_ast_pretty_src_pprust_tests.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/state/expr.rs
    pub mod rustc_ast_pretty_pprust_state_expr {
        include!("processed_rustc_ast_pretty_pprust_state_expr.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pp/convenience.rs
    pub mod rustc_ast_pretty_src_pp_convenience {
        include!("processed_rustc_ast_pretty_src_pp_convenience.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/lib.rs
    pub mod rustc_ast_pretty_rustc_ast_pretty_src_lib {
        include!("processed_rustc_ast_pretty_rustc_ast_pretty_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/state/fixup.rs
    pub mod rustc_ast_pretty_pprust_state_fixup {
        include!("processed_rustc_ast_pretty_pprust_state_fixup.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pp/ring.rs
    pub mod rustc_ast_pretty_src_pp_ring {
        include!("processed_rustc_ast_pretty_src_pp_ring.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/state.rs
    pub mod rustc_ast_pretty_src_pprust_state {
        include!("processed_rustc_ast_pretty_src_pprust_state.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/helpers.rs
    pub mod rustc_ast_pretty_rustc_ast_pretty_src_helpers {
        include!("processed_rustc_ast_pretty_rustc_ast_pretty_src_helpers.rs");
    }
}

// 24: rustc_llvm (2 files)
pub mod included_rustc_llvm {
    // Source: ../rust/compiler/rustc_llvm/src/lib.rs
    pub mod rustc_llvm_rustc_llvm_src_lib {
        include!("processed_rustc_llvm_rustc_llvm_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_llvm/build.rs
    pub mod rustc_llvm_compiler_rustc_llvm_build {
        include!("processed_rustc_llvm_compiler_rustc_llvm_build.rs");
    }
}

// 25: rustc_parse_format (2 files)
pub mod included_rustc_parse_format {
    // Source: ../rust/compiler/rustc_parse_format/src/tests.rs
    pub mod rustc_parse_format_rustc_parse_format_src_tests {
        include!("processed_rustc_parse_format_rustc_parse_format_src_tests.rs");
    }
}

// 26: rustc_passes (20 files)
pub mod included_rustc_passes {
    // Source: ../rust/compiler/rustc_passes/src/layout_test.rs
    pub mod rustc_passes_rustc_passes_src_layout_test {
        include!("processed_rustc_passes_rustc_passes_src_layout_test.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/input_stats.rs
    pub mod rustc_passes_rustc_passes_src_input_stats {
        include!("processed_rustc_passes_rustc_passes_src_input_stats.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/check_export.rs
    pub mod rustc_passes_rustc_passes_src_check_export {
        include!("processed_rustc_passes_rustc_passes_src_check_export.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/hir_id_validator.rs
    pub mod rustc_passes_rustc_passes_src_hir_id_validator {
        include!("processed_rustc_passes_rustc_passes_src_hir_id_validator.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/lib.rs
    pub mod rustc_passes_rustc_passes_src_lib {
        include!("processed_rustc_passes_rustc_passes_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/liveness.rs
    pub mod rustc_passes_rustc_passes_src_liveness {
        include!("processed_rustc_passes_rustc_passes_src_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/lib_features.rs
    pub mod rustc_passes_rustc_passes_src_lib_features {
        include!("processed_rustc_passes_rustc_passes_src_lib_features.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/errors.rs
    pub mod rustc_passes_rustc_passes_src_errors {
        include!("processed_rustc_passes_rustc_passes_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/abi_test.rs
    pub mod rustc_passes_rustc_passes_src_abi_test {
        include!("processed_rustc_passes_rustc_passes_src_abi_test.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/check_attr.rs
    pub mod rustc_passes_rustc_passes_src_check_attr {
        include!("processed_rustc_passes_rustc_passes_src_check_attr.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/lang_items.rs
    pub mod rustc_passes_rustc_passes_src_lang_items {
        include!("processed_rustc_passes_rustc_passes_src_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/reachable.rs
    pub mod rustc_passes_rustc_passes_src_reachable {
        include!("processed_rustc_passes_rustc_passes_src_reachable.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/weak_lang_items.rs
    pub mod rustc_passes_rustc_passes_src_weak_lang_items {
        include!("processed_rustc_passes_rustc_passes_src_weak_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/liveness/rwu_table.rs
    pub mod rustc_passes_src_liveness_rwu_table {
        include!("processed_rustc_passes_src_liveness_rwu_table.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/upvars.rs
    pub mod rustc_passes_rustc_passes_src_upvars {
        include!("processed_rustc_passes_rustc_passes_src_upvars.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/stability.rs
    pub mod rustc_passes_rustc_passes_src_stability {
        include!("processed_rustc_passes_rustc_passes_src_stability.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/entry.rs
    pub mod rustc_passes_rustc_passes_src_entry {
        include!("processed_rustc_passes_rustc_passes_src_entry.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/dead.rs
    pub mod rustc_passes_rustc_passes_src_dead {
        include!("processed_rustc_passes_rustc_passes_src_dead.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/debugger_visualizer.rs
    pub mod rustc_passes_rustc_passes_src_debugger_visualizer {
        include!("processed_rustc_passes_rustc_passes_src_debugger_visualizer.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/diagnostic_items.rs
    pub mod rustc_passes_rustc_passes_src_diagnostic_items {
        include!("processed_rustc_passes_rustc_passes_src_diagnostic_items.rs");
    }
}

// 27: rustc_ast (22 files)
pub mod included_rustc_ast {
    // Source: ../rust/compiler/rustc_ast/src/lib.rs
    pub mod rustc_ast_rustc_ast_src_lib {
        include!("processed_rustc_ast_rustc_ast_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/comments/tests.rs
    pub mod rustc_ast_util_comments_tests {
        include!("processed_rustc_ast_util_comments_tests.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/parser.rs
    pub mod rustc_ast_src_util_parser {
        include!("processed_rustc_ast_src_util_parser.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/unicode.rs
    pub mod rustc_ast_src_util_unicode {
        include!("processed_rustc_ast_src_util_unicode.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/attr/mod.rs
    pub mod rustc_ast_src_attr_mod {
        include!("processed_rustc_ast_src_attr_mod.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/mut_visit.rs
    pub mod rustc_ast_rustc_ast_src_mut_visit {
        include!("processed_rustc_ast_rustc_ast_src_mut_visit.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/literal.rs
    pub mod rustc_ast_src_util_literal {
        include!("processed_rustc_ast_src_util_literal.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/classify.rs
    pub mod rustc_ast_src_util_classify {
        include!("processed_rustc_ast_src_util_classify.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/token.rs
    pub mod rustc_ast_rustc_ast_src_token {
        include!("processed_rustc_ast_rustc_ast_src_token.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/mod.rs
    pub mod rustc_ast_src_expand_mod {
        include!("processed_rustc_ast_src_expand_mod.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/allocator.rs
    pub mod rustc_ast_src_expand_allocator {
        include!("processed_rustc_ast_src_expand_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/autodiff_attrs.rs
    pub mod rustc_ast_src_expand_autodiff_attrs {
        include!("processed_rustc_ast_src_expand_autodiff_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/tokenstream.rs
    pub mod rustc_ast_rustc_ast_src_tokenstream {
        include!("processed_rustc_ast_rustc_ast_src_tokenstream.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/format.rs
    pub mod rustc_ast_rustc_ast_src_format {
        include!("processed_rustc_ast_rustc_ast_src_format.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/ast.rs
    pub mod rustc_ast_rustc_ast_src_ast {
        include!("processed_rustc_ast_rustc_ast_src_ast.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/visit.rs
    pub mod rustc_ast_rustc_ast_src_visit {
        include!("processed_rustc_ast_rustc_ast_src_visit.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/entry.rs
    pub mod rustc_ast_rustc_ast_src_entry {
        include!("processed_rustc_ast_rustc_ast_src_entry.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/case.rs
    pub mod rustc_ast_src_util_case {
        include!("processed_rustc_ast_src_util_case.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/typetree.rs
    pub mod rustc_ast_src_expand_typetree {
        include!("processed_rustc_ast_src_expand_typetree.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/ast_traits.rs
    pub mod rustc_ast_rustc_ast_src_ast_traits {
        include!("processed_rustc_ast_rustc_ast_src_ast_traits.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/node_id.rs
    pub mod rustc_ast_rustc_ast_src_node_id {
        include!("processed_rustc_ast_rustc_ast_src_node_id.rs");
    }
}

// 28: rustc_mir_build (36 files)
pub mod included_rustc_mir_build {
    // Source: ../rust/compiler/rustc_mir_build/src/thir/pattern/check_match.rs
    pub mod rustc_mir_build_thir_pattern_check_match {
        include!("processed_rustc_mir_build_thir_pattern_check_match.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/category.rs
    pub mod rustc_mir_build_builder_expr_category {
        include!("processed_rustc_mir_build_builder_expr_category.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/lib.rs
    pub mod rustc_mir_build_rustc_mir_build_src_lib {
        include!("processed_rustc_mir_build_rustc_mir_build_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/pattern/const_to_pat.rs
    pub mod rustc_mir_build_thir_pattern_const_to_pat {
        include!("processed_rustc_mir_build_thir_pattern_const_to_pat.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/util.rs
    pub mod rustc_mir_build_builder_matches_util {
        include!("processed_rustc_mir_build_builder_matches_util.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/pattern/migration.rs
    pub mod rustc_mir_build_thir_pattern_migration {
        include!("processed_rustc_mir_build_thir_pattern_migration.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/mod.rs
    pub mod rustc_mir_build_src_builder_mod {
        include!("processed_rustc_mir_build_src_builder_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_place.rs
    pub mod rustc_mir_build_builder_expr_as_place {
        include!("processed_rustc_mir_build_builder_expr_as_place.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/pattern/mod.rs
    pub mod rustc_mir_build_thir_pattern_mod {
        include!("processed_rustc_mir_build_thir_pattern_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/cx/mod.rs
    pub mod rustc_mir_build_thir_cx_mod {
        include!("processed_rustc_mir_build_thir_cx_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/cx/expr.rs
    pub mod rustc_mir_build_thir_cx_expr {
        include!("processed_rustc_mir_build_thir_cx_expr.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/custom/mod.rs
    pub mod rustc_mir_build_builder_custom_mod {
        include!("processed_rustc_mir_build_builder_custom_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_temp.rs
    pub mod rustc_mir_build_builder_expr_as_temp {
        include!("processed_rustc_mir_build_builder_expr_as_temp.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/misc.rs
    pub mod rustc_mir_build_src_builder_misc {
        include!("processed_rustc_mir_build_src_builder_misc.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/print.rs
    pub mod rustc_mir_build_src_thir_print {
        include!("processed_rustc_mir_build_src_thir_print.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_rvalue.rs
    pub mod rustc_mir_build_builder_expr_as_rvalue {
        include!("processed_rustc_mir_build_builder_expr_as_rvalue.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/mod.rs
    pub mod rustc_mir_build_builder_matches_mod {
        include!("processed_rustc_mir_build_builder_matches_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/util.rs
    pub mod rustc_mir_build_src_thir_util {
        include!("processed_rustc_mir_build_src_thir_util.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/cfg.rs
    pub mod rustc_mir_build_src_builder_cfg {
        include!("processed_rustc_mir_build_src_builder_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/check_unsafety.rs
    pub mod rustc_mir_build_rustc_mir_build_src_check_unsafety {
        include!("processed_rustc_mir_build_rustc_mir_build_src_check_unsafety.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/match_pair.rs
    pub mod rustc_mir_build_builder_matches_match_pair {
        include!("processed_rustc_mir_build_builder_matches_match_pair.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/test.rs
    pub mod rustc_mir_build_builder_matches_test {
        include!("processed_rustc_mir_build_builder_matches_test.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/into.rs
    pub mod rustc_mir_build_builder_expr_into {
        include!("processed_rustc_mir_build_builder_expr_into.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/custom/parse.rs
    pub mod rustc_mir_build_builder_custom_parse {
        include!("processed_rustc_mir_build_builder_custom_parse.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/coverageinfo.rs
    pub mod rustc_mir_build_src_builder_coverageinfo {
        include!("processed_rustc_mir_build_src_builder_coverageinfo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/user_ty.rs
    pub mod rustc_mir_build_builder_matches_user_ty {
        include!("processed_rustc_mir_build_builder_matches_user_ty.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/errors.rs
    pub mod rustc_mir_build_rustc_mir_build_src_errors {
        include!("processed_rustc_mir_build_rustc_mir_build_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/cx/block.rs
    pub mod rustc_mir_build_thir_cx_block {
        include!("processed_rustc_mir_build_thir_cx_block.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/stmt.rs
    pub mod rustc_mir_build_builder_expr_stmt {
        include!("processed_rustc_mir_build_builder_expr_stmt.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/custom/parse/instruction.rs
    pub mod rustc_mir_build_custom_parse_instruction {
        include!("processed_rustc_mir_build_custom_parse_instruction.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/scope.rs
    pub mod rustc_mir_build_src_builder_scope {
        include!("processed_rustc_mir_build_src_builder_scope.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/block.rs
    pub mod rustc_mir_build_src_builder_block {
        include!("processed_rustc_mir_build_src_builder_block.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/check_tail_calls.rs
    pub mod rustc_mir_build_rustc_mir_build_src_check_tail_calls {
        include!("processed_rustc_mir_build_rustc_mir_build_src_check_tail_calls.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/constant.rs
    pub mod rustc_mir_build_src_thir_constant {
        include!("processed_rustc_mir_build_src_thir_constant.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_operand.rs
    pub mod rustc_mir_build_builder_expr_as_operand {
        include!("processed_rustc_mir_build_builder_expr_as_operand.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_constant.rs
    pub mod rustc_mir_build_builder_expr_as_constant {
        include!("processed_rustc_mir_build_builder_expr_as_constant.rs");
    }
}

// 29: rustc_symbol_mangling (7 files)
pub mod included_rustc_symbol_mangling {
    // Source: ../rust/compiler/rustc_symbol_mangling/src/test.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_test {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_test.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/errors.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_errors {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/v0.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_v0 {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_v0.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/legacy.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_legacy {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_legacy.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/hashed.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_hashed {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_hashed.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/lib.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_lib {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/export.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_export {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_export.rs");
    }
}

// 30: rustc_fs_util (1 files)
pub mod included_rustc_fs_util {
    // Source: ../rust/compiler/rustc_fs_util/src/lib.rs
    pub mod rustc_fs_util_rustc_fs_util_src_lib {
        include!("processed_rustc_fs_util_rustc_fs_util_src_lib.rs");
    }
}

// 31: rustc_sanitizers (8 files)
pub mod included_rustc_sanitizers {
    // Source: ../rust/compiler/rustc_sanitizers/src/lib.rs
    pub mod rustc_sanitizers_rustc_sanitizers_src_lib {
        include!("processed_rustc_sanitizers_rustc_sanitizers_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/kcfi/mod.rs
    pub mod rustc_sanitizers_src_kcfi_mod {
        include!("processed_rustc_sanitizers_src_kcfi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/mod.rs
    pub mod rustc_sanitizers_typeid_itanium_cxx_abi_mod {
        include!("processed_rustc_sanitizers_typeid_itanium_cxx_abi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/mod.rs
    pub mod rustc_sanitizers_src_cfi_mod {
        include!("processed_rustc_sanitizers_src_cfi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/kcfi/typeid/mod.rs
    pub mod rustc_sanitizers_kcfi_typeid_mod {
        include!("processed_rustc_sanitizers_kcfi_typeid_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/mod.rs
    pub mod rustc_sanitizers_cfi_typeid_mod {
        include!("processed_rustc_sanitizers_cfi_typeid_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/transform.rs
    pub mod rustc_sanitizers_typeid_itanium_cxx_abi_transform {
        include!("processed_rustc_sanitizers_typeid_itanium_cxx_abi_transform.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/encode.rs
    pub mod rustc_sanitizers_typeid_itanium_cxx_abi_encode {
        include!("processed_rustc_sanitizers_typeid_itanium_cxx_abi_encode.rs");
    }
}

// 32: rustc_transmute (8 files)
pub mod included_rustc_transmute {
    // Source: ../rust/compiler/rustc_transmute/src/maybe_transmutable/tests.rs
    pub mod rustc_transmute_src_maybe_transmutable_tests {
        include!("processed_rustc_transmute_src_maybe_transmutable_tests.rs");
    }
    // Source: ../rust/compiler/rustc_transmute/src/lib.rs
    pub mod rustc_transmute_rustc_transmute_src_lib {
        include!("processed_rustc_transmute_rustc_transmute_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_transmute/src/layout/tree/tests.rs
    pub mod rustc_transmute_layout_tree_tests {
        include!("processed_rustc_transmute_layout_tree_tests.rs");
    }
}

// 33: rustc_expand (19 files)
pub mod included_rustc_expand {
    // Source: ../rust/compiler/rustc_expand/src/stats.rs
    pub mod rustc_expand_rustc_expand_src_stats {
        include!("processed_rustc_expand_rustc_expand_src_stats.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/macro_check.rs
    pub mod rustc_expand_src_mbe_macro_check {
        include!("processed_rustc_expand_src_mbe_macro_check.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/diagnostics.rs
    pub mod rustc_expand_src_mbe_diagnostics {
        include!("processed_rustc_expand_src_mbe_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/transcribe.rs
    pub mod rustc_expand_src_mbe_transcribe {
        include!("processed_rustc_expand_src_mbe_transcribe.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/module.rs
    pub mod rustc_expand_rustc_expand_src_module {
        include!("processed_rustc_expand_rustc_expand_src_module.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/config.rs
    pub mod rustc_expand_rustc_expand_src_config {
        include!("processed_rustc_expand_rustc_expand_src_config.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/metavar_expr.rs
    pub mod rustc_expand_src_mbe_metavar_expr {
        include!("processed_rustc_expand_src_mbe_metavar_expr.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/expand.rs
    pub mod rustc_expand_rustc_expand_src_expand {
        include!("processed_rustc_expand_rustc_expand_src_expand.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/proc_macro.rs
    pub mod rustc_expand_rustc_expand_src_proc_macro {
        include!("processed_rustc_expand_rustc_expand_src_proc_macro.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/quoted.rs
    pub mod rustc_expand_src_mbe_quoted {
        include!("processed_rustc_expand_src_mbe_quoted.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/placeholders.rs
    pub mod rustc_expand_rustc_expand_src_placeholders {
        include!("processed_rustc_expand_rustc_expand_src_placeholders.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/base.rs
    pub mod rustc_expand_rustc_expand_src_base {
        include!("processed_rustc_expand_rustc_expand_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/build.rs
    pub mod rustc_expand_rustc_expand_src_build {
        include!("processed_rustc_expand_rustc_expand_src_build.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe.rs
    pub mod rustc_expand_rustc_expand_src_mbe {
        include!("processed_rustc_expand_rustc_expand_src_mbe.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/lib.rs
    pub mod rustc_expand_rustc_expand_src_lib {
        include!("processed_rustc_expand_rustc_expand_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/proc_macro_server.rs
    pub mod rustc_expand_rustc_expand_src_proc_macro_server {
        include!("processed_rustc_expand_rustc_expand_src_proc_macro_server.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/macro_parser.rs
    pub mod rustc_expand_src_mbe_macro_parser {
        include!("processed_rustc_expand_src_mbe_macro_parser.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/errors.rs
    pub mod rustc_expand_rustc_expand_src_errors {
        include!("processed_rustc_expand_rustc_expand_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/macro_rules.rs
    pub mod rustc_expand_src_mbe_macro_rules {
        include!("processed_rustc_expand_src_mbe_macro_rules.rs");
    }
}

// 34: rustc_serialize (8 files)
pub mod included_rustc_serialize {
    // Source: ../rust/compiler/rustc_serialize/src/serialize.rs
    pub mod rustc_serialize_rustc_serialize_src_serialize {
        include!("processed_rustc_serialize_rustc_serialize_src_serialize.rs");
    }
    // Source: ../rust/compiler/rustc_serialize/src/opaque/mem_encoder.rs
    pub mod rustc_serialize_src_opaque_mem_encoder {
        include!("processed_rustc_serialize_src_opaque_mem_encoder.rs");
    }
    // Source: ../rust/compiler/rustc_serialize/src/opaque/tests.rs
    pub mod rustc_serialize_src_opaque_tests {
        include!("processed_rustc_serialize_src_opaque_tests.rs");
    }
    // Source: ../rust/compiler/rustc_serialize/src/leb128/tests.rs
    pub mod rustc_serialize_src_leb128_tests {
        include!("processed_rustc_serialize_src_leb128_tests.rs");
    }
    // Source: ../rust/compiler/rustc_serialize/src/int_overflow.rs
    pub mod rustc_serialize_rustc_serialize_src_int_overflow {
        include!("processed_rustc_serialize_rustc_serialize_src_int_overflow.rs");
    }
}

// 35: rustc_fluent_macro (2 files)
pub mod included_rustc_fluent_macro {
    // Source: ../rust/compiler/rustc_fluent_macro/src/lib.rs
    pub mod rustc_fluent_macro_rustc_fluent_macro_src_lib {
        include!("processed_rustc_fluent_macro_rustc_fluent_macro_src_lib.rs");
    }
}

// 36: rustc_hir_pretty (1 files)
pub mod included_rustc_hir_pretty {
    // Source: ../rust/compiler/rustc_hir_pretty/src/lib.rs
    pub mod rustc_hir_pretty_rustc_hir_pretty_src_lib {
        include!("processed_rustc_hir_pretty_rustc_hir_pretty_src_lib.rs");
    }
}

// 37: rustc_baked_icu_data (2 files)
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

// 38: rustc_mir_dataflow (24 files)
pub mod included_rustc_mir_dataflow {
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/fmt.rs
    pub mod rustc_mir_dataflow_src_framework_fmt {
        include!("processed_rustc_mir_dataflow_src_framework_fmt.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/tests.rs
    pub mod rustc_mir_dataflow_src_framework_tests {
        include!("processed_rustc_mir_dataflow_src_framework_tests.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/move_paths/mod.rs
    pub mod rustc_mir_dataflow_src_move_paths_mod {
        include!("processed_rustc_mir_dataflow_src_move_paths_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/value_analysis.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_value_analysis {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_value_analysis.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/borrowed_locals.rs
    pub mod rustc_mir_dataflow_src_impls_borrowed_locals {
        include!("processed_rustc_mir_dataflow_src_impls_borrowed_locals.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/visitor.rs
    pub mod rustc_mir_dataflow_src_framework_visitor {
        include!("processed_rustc_mir_dataflow_src_framework_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/debuginfo.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_debuginfo {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/storage_liveness.rs
    pub mod rustc_mir_dataflow_src_impls_storage_liveness {
        include!("processed_rustc_mir_dataflow_src_impls_storage_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/un_derefer.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_un_derefer {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_un_derefer.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/graphviz.rs
    pub mod rustc_mir_dataflow_src_framework_graphviz {
        include!("processed_rustc_mir_dataflow_src_framework_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/liveness.rs
    pub mod rustc_mir_dataflow_src_impls_liveness {
        include!("processed_rustc_mir_dataflow_src_impls_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/rustc_peek.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_rustc_peek {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_rustc_peek.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/mod.rs
    pub mod rustc_mir_dataflow_src_impls_mod {
        include!("processed_rustc_mir_dataflow_src_impls_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/lattice.rs
    pub mod rustc_mir_dataflow_src_framework_lattice {
        include!("processed_rustc_mir_dataflow_src_framework_lattice.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/drop_flag_effects.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_drop_flag_effects {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_drop_flag_effects.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/errors.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_errors {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/results.rs
    pub mod rustc_mir_dataflow_src_framework_results {
        include!("processed_rustc_mir_dataflow_src_framework_results.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/move_paths/builder.rs
    pub mod rustc_mir_dataflow_src_move_paths_builder {
        include!("processed_rustc_mir_dataflow_src_move_paths_builder.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/lib.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_lib {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/points.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_points {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_points.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/initialized.rs
    pub mod rustc_mir_dataflow_src_impls_initialized {
        include!("processed_rustc_mir_dataflow_src_impls_initialized.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/direction.rs
    pub mod rustc_mir_dataflow_src_framework_direction {
        include!("processed_rustc_mir_dataflow_src_framework_direction.rs");
    }
}

// 39: rustc (2 files)
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

// 40: rustc_arena (2 files)
pub mod included_rustc_arena {
    // Source: ../rust/compiler/rustc_arena/src/tests.rs
    pub mod rustc_arena_rustc_arena_src_tests {
        include!("processed_rustc_arena_rustc_arena_src_tests.rs");
    }
}

// 41: rustc_log (1 files)
pub mod included_rustc_log {
    // Source: ../rust/compiler/rustc_log/src/lib.rs
    pub mod rustc_log_rustc_log_src_lib {
        include!("processed_rustc_log_rustc_log_src_lib.rs");
    }
}

// 42: rustc_query_system (19 files)
pub mod included_rustc_query_system {
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/debug.rs
    pub mod rustc_query_system_src_dep_graph_debug {
        include!("processed_rustc_query_system_src_dep_graph_debug.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/mod.rs
    pub mod rustc_query_system_src_dep_graph_mod {
        include!("processed_rustc_query_system_src_dep_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/plumbing.rs
    pub mod rustc_query_system_src_query_plumbing {
        include!("processed_rustc_query_system_src_query_plumbing.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/cache.rs
    pub mod rustc_query_system_rustc_query_system_src_cache {
        include!("processed_rustc_query_system_rustc_query_system_src_cache.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/values.rs
    pub mod rustc_query_system_rustc_query_system_src_values {
        include!("processed_rustc_query_system_rustc_query_system_src_values.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/mod.rs
    pub mod rustc_query_system_src_ich_mod {
        include!("processed_rustc_query_system_src_ich_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/lib.rs
    pub mod rustc_query_system_rustc_query_system_src_lib {
        include!("processed_rustc_query_system_rustc_query_system_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/edges.rs
    pub mod rustc_query_system_src_dep_graph_edges {
        include!("processed_rustc_query_system_src_dep_graph_edges.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/caches.rs
    pub mod rustc_query_system_src_query_caches {
        include!("processed_rustc_query_system_src_query_caches.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/dep_node.rs
    pub mod rustc_query_system_src_dep_graph_dep_node {
        include!("processed_rustc_query_system_src_dep_graph_dep_node.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/config.rs
    pub mod rustc_query_system_src_query_config {
        include!("processed_rustc_query_system_src_query_config.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/impls_syntax.rs
    pub mod rustc_query_system_src_ich_impls_syntax {
        include!("processed_rustc_query_system_src_ich_impls_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/serialized.rs
    pub mod rustc_query_system_src_dep_graph_serialized {
        include!("processed_rustc_query_system_src_dep_graph_serialized.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/job.rs
    pub mod rustc_query_system_src_query_job {
        include!("processed_rustc_query_system_src_query_job.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/mod.rs
    pub mod rustc_query_system_src_query_mod {
        include!("processed_rustc_query_system_src_query_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/hcx.rs
    pub mod rustc_query_system_src_ich_hcx {
        include!("processed_rustc_query_system_src_ich_hcx.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/graph.rs
    pub mod rustc_query_system_src_dep_graph_graph {
        include!("processed_rustc_query_system_src_dep_graph_graph.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/error.rs
    pub mod rustc_query_system_rustc_query_system_src_error {
        include!("processed_rustc_query_system_rustc_query_system_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/query.rs
    pub mod rustc_query_system_src_dep_graph_query {
        include!("processed_rustc_query_system_src_dep_graph_query.rs");
    }
}

// 43: rustc_abi (11 files)
pub mod included_rustc_abi {
    // Source: ../rust/compiler/rustc_abi/src/tests.rs
    pub mod rustc_abi_rustc_abi_src_tests {
        include!("processed_rustc_abi_rustc_abi_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/layout/simple.rs
    pub mod rustc_abi_src_layout_simple {
        include!("processed_rustc_abi_src_layout_simple.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/callconv/reg.rs
    pub mod rustc_abi_src_callconv_reg {
        include!("processed_rustc_abi_src_callconv_reg.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/layout.rs
    pub mod rustc_abi_rustc_abi_src_layout {
        include!("processed_rustc_abi_rustc_abi_src_layout.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/callconv.rs
    pub mod rustc_abi_rustc_abi_src_callconv {
        include!("processed_rustc_abi_rustc_abi_src_callconv.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/layout/coroutine.rs
    pub mod rustc_abi_src_layout_coroutine {
        include!("processed_rustc_abi_src_layout_coroutine.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/canon_abi.rs
    pub mod rustc_abi_rustc_abi_src_canon_abi {
        include!("processed_rustc_abi_rustc_abi_src_canon_abi.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/extern_abi/tests.rs
    pub mod rustc_abi_src_extern_abi_tests {
        include!("processed_rustc_abi_src_extern_abi_tests.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/layout/ty.rs
    pub mod rustc_abi_src_layout_ty {
        include!("processed_rustc_abi_src_layout_ty.rs");
    }
}

// 44: rustc_graphviz (2 files)
pub mod included_rustc_graphviz {
    // Source: ../rust/compiler/rustc_graphviz/src/tests.rs
    pub mod rustc_graphviz_rustc_graphviz_src_tests {
        include!("processed_rustc_graphviz_rustc_graphviz_src_tests.rs");
    }
}

// 45: rustc_type_ir (39 files)
pub mod included_rustc_type_ir {
    // Source: ../rust/compiler/rustc_type_ir/src/infer_ctxt.rs
    pub mod rustc_type_ir_rustc_type_ir_src_infer_ctxt {
        include!("processed_rustc_type_ir_rustc_type_ir_src_infer_ctxt.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/binder.rs
    pub mod rustc_type_ir_rustc_type_ir_src_binder {
        include!("processed_rustc_type_ir_rustc_type_ir_src_binder.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/macros.rs
    pub mod rustc_type_ir_rustc_type_ir_src_macros {
        include!("processed_rustc_type_ir_rustc_type_ir_src_macros.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/const_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_const_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_const_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/relate.rs
    pub mod rustc_type_ir_rustc_type_ir_src_relate {
        include!("processed_rustc_type_ir_rustc_type_ir_src_relate.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/opaque_ty.rs
    pub mod rustc_type_ir_rustc_type_ir_src_opaque_ty {
        include!("processed_rustc_type_ir_rustc_type_ir_src_opaque_ty.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/inherent.rs
    pub mod rustc_type_ir_rustc_type_ir_src_inherent {
        include!("processed_rustc_type_ir_rustc_type_ir_src_inherent.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/walk.rs
    pub mod rustc_type_ir_rustc_type_ir_src_walk {
        include!("processed_rustc_type_ir_rustc_type_ir_src_walk.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/lift.rs
    pub mod rustc_type_ir_rustc_type_ir_src_lift {
        include!("processed_rustc_type_ir_rustc_type_ir_src_lift.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/relate/solver_relating.rs
    pub mod rustc_type_ir_src_relate_solver_relating {
        include!("processed_rustc_type_ir_src_relate_solver_relating.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/solve/inspect.rs
    pub mod rustc_type_ir_src_solve_inspect {
        include!("processed_rustc_type_ir_src_solve_inspect.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/fast_reject.rs
    pub mod rustc_type_ir_rustc_type_ir_src_fast_reject {
        include!("processed_rustc_type_ir_rustc_type_ir_src_fast_reject.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/search_graph/stack.rs
    pub mod rustc_type_ir_src_search_graph_stack {
        include!("processed_rustc_type_ir_src_search_graph_stack.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/fold.rs
    pub mod rustc_type_ir_rustc_type_ir_src_fold {
        include!("processed_rustc_type_ir_rustc_type_ir_src_fold.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/canonical.rs
    pub mod rustc_type_ir_rustc_type_ir_src_canonical {
        include!("processed_rustc_type_ir_rustc_type_ir_src_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/search_graph/global_cache.rs
    pub mod rustc_type_ir_src_search_graph_global_cache {
        include!("processed_rustc_type_ir_src_search_graph_global_cache.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/interner.rs
    pub mod rustc_type_ir_rustc_type_ir_src_interner {
        include!("processed_rustc_type_ir_rustc_type_ir_src_interner.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ty_kind/closure.rs
    pub mod rustc_type_ir_src_ty_kind_closure {
        include!("processed_rustc_type_ir_src_ty_kind_closure.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/lang_items.rs
    pub mod rustc_type_ir_rustc_type_ir_src_lang_items {
        include!("processed_rustc_type_ir_rustc_type_ir_src_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/search_graph/mod.rs
    pub mod rustc_type_ir_src_search_graph_mod {
        include!("processed_rustc_type_ir_src_search_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/predicate_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_predicate_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_predicate_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/relate/combine.rs
    pub mod rustc_type_ir_src_relate_combine {
        include!("processed_rustc_type_ir_src_relate_combine.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/error.rs
    pub mod rustc_type_ir_rustc_type_ir_src_error {
        include!("processed_rustc_type_ir_rustc_type_ir_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/predicate.rs
    pub mod rustc_type_ir_rustc_type_ir_src_predicate {
        include!("processed_rustc_type_ir_rustc_type_ir_src_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/generic_arg.rs
    pub mod rustc_type_ir_rustc_type_ir_src_generic_arg {
        include!("processed_rustc_type_ir_rustc_type_ir_src_generic_arg.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/flags.rs
    pub mod rustc_type_ir_rustc_type_ir_src_flags {
        include!("processed_rustc_type_ir_rustc_type_ir_src_flags.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/data_structures/mod.rs
    pub mod rustc_type_ir_src_data_structures_mod {
        include!("processed_rustc_type_ir_src_data_structures_mod.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/data_structures/delayed_map.rs
    pub mod rustc_type_ir_src_data_structures_delayed_map {
        include!("processed_rustc_type_ir_src_data_structures_delayed_map.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/region_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_region_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_region_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ir_print.rs
    pub mod rustc_type_ir_rustc_type_ir_src_ir_print {
        include!("processed_rustc_type_ir_rustc_type_ir_src_ir_print.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/visit.rs
    pub mod rustc_type_ir_rustc_type_ir_src_visit {
        include!("processed_rustc_type_ir_rustc_type_ir_src_visit.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/pattern.rs
    pub mod rustc_type_ir_rustc_type_ir_src_pattern {
        include!("processed_rustc_type_ir_rustc_type_ir_src_pattern.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/outlives.rs
    pub mod rustc_type_ir_rustc_type_ir_src_outlives {
        include!("processed_rustc_type_ir_rustc_type_ir_src_outlives.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ty_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_ty_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_ty_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/solve/mod.rs
    pub mod rustc_type_ir_src_solve_mod {
        include!("processed_rustc_type_ir_src_solve_mod.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/lib.rs
    pub mod rustc_type_ir_rustc_type_ir_src_lib {
        include!("processed_rustc_type_ir_rustc_type_ir_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/elaborate.rs
    pub mod rustc_type_ir_rustc_type_ir_src_elaborate {
        include!("processed_rustc_type_ir_rustc_type_ir_src_elaborate.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ty_info.rs
    pub mod rustc_type_ir_rustc_type_ir_src_ty_info {
        include!("processed_rustc_type_ir_rustc_type_ir_src_ty_info.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/upcast.rs
    pub mod rustc_type_ir_rustc_type_ir_src_upcast {
        include!("processed_rustc_type_ir_rustc_type_ir_src_upcast.rs");
    }
}

// 46: rustc_const_eval (40 files)
pub mod included_rustc_const_eval {
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/call.rs
    pub mod rustc_const_eval_src_interpret_call {
        include!("processed_rustc_const_eval_src_interpret_call.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/alignment.rs
    pub mod rustc_const_eval_src_util_alignment {
        include!("processed_rustc_const_eval_src_util_alignment.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/caller_location.rs
    pub mod rustc_const_eval_src_util_caller_location {
        include!("processed_rustc_const_eval_src_util_caller_location.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/eval_queries.rs
    pub mod rustc_const_eval_src_const_eval_eval_queries {
        include!("processed_rustc_const_eval_src_const_eval_eval_queries.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/operand.rs
    pub mod rustc_const_eval_src_interpret_operand {
        include!("processed_rustc_const_eval_src_interpret_operand.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/projection.rs
    pub mod rustc_const_eval_src_interpret_projection {
        include!("processed_rustc_const_eval_src_interpret_projection.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/place.rs
    pub mod rustc_const_eval_src_interpret_place {
        include!("processed_rustc_const_eval_src_interpret_place.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/valtrees.rs
    pub mod rustc_const_eval_src_const_eval_valtrees {
        include!("processed_rustc_const_eval_src_const_eval_valtrees.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/qualifs.rs
    pub mod rustc_const_eval_src_check_consts_qualifs {
        include!("processed_rustc_const_eval_src_check_consts_qualifs.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/util.rs
    pub mod rustc_const_eval_src_interpret_util {
        include!("processed_rustc_const_eval_src_interpret_util.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/resolver.rs
    pub mod rustc_const_eval_src_check_consts_resolver {
        include!("processed_rustc_const_eval_src_check_consts_resolver.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/traits.rs
    pub mod rustc_const_eval_src_interpret_traits {
        include!("processed_rustc_const_eval_src_interpret_traits.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/mod.rs
    pub mod rustc_const_eval_src_const_eval_mod {
        include!("processed_rustc_const_eval_src_const_eval_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/mod.rs
    pub mod rustc_const_eval_src_check_consts_mod {
        include!("processed_rustc_const_eval_src_check_consts_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/operator.rs
    pub mod rustc_const_eval_src_interpret_operator {
        include!("processed_rustc_const_eval_src_interpret_operator.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/intern.rs
    pub mod rustc_const_eval_src_interpret_intern {
        include!("processed_rustc_const_eval_src_interpret_intern.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/dummy_machine.rs
    pub mod rustc_const_eval_src_const_eval_dummy_machine {
        include!("processed_rustc_const_eval_src_const_eval_dummy_machine.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/stack.rs
    pub mod rustc_const_eval_src_interpret_stack {
        include!("processed_rustc_const_eval_src_interpret_stack.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/discriminant.rs
    pub mod rustc_const_eval_src_interpret_discriminant {
        include!("processed_rustc_const_eval_src_interpret_discriminant.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/mod.rs
    pub mod rustc_const_eval_src_interpret_mod {
        include!("processed_rustc_const_eval_src_interpret_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/type_name.rs
    pub mod rustc_const_eval_src_util_type_name {
        include!("processed_rustc_const_eval_src_util_type_name.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/machine.rs
    pub mod rustc_const_eval_src_interpret_machine {
        include!("processed_rustc_const_eval_src_interpret_machine.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/mod.rs
    pub mod rustc_const_eval_src_util_mod {
        include!("processed_rustc_const_eval_src_util_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/visitor.rs
    pub mod rustc_const_eval_src_interpret_visitor {
        include!("processed_rustc_const_eval_src_interpret_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/step.rs
    pub mod rustc_const_eval_src_interpret_step {
        include!("processed_rustc_const_eval_src_interpret_step.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/check.rs
    pub mod rustc_const_eval_src_check_consts_check {
        include!("processed_rustc_const_eval_src_check_consts_check.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/post_drop_elaboration.rs
    pub mod rustc_const_eval_src_check_consts_post_drop_elaboration {
        include!("processed_rustc_const_eval_src_check_consts_post_drop_elaboration.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/fn_queries.rs
    pub mod rustc_const_eval_src_const_eval_fn_queries {
        include!("processed_rustc_const_eval_src_const_eval_fn_queries.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/error.rs
    pub mod rustc_const_eval_src_const_eval_error {
        include!("processed_rustc_const_eval_src_const_eval_error.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/validity.rs
    pub mod rustc_const_eval_src_interpret_validity {
        include!("processed_rustc_const_eval_src_interpret_validity.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/lib.rs
    pub mod rustc_const_eval_rustc_const_eval_src_lib {
        include!("processed_rustc_const_eval_rustc_const_eval_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/ops.rs
    pub mod rustc_const_eval_src_check_consts_ops {
        include!("processed_rustc_const_eval_src_check_consts_ops.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/eval_context.rs
    pub mod rustc_const_eval_src_interpret_eval_context {
        include!("processed_rustc_const_eval_src_interpret_eval_context.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/errors.rs
    pub mod rustc_const_eval_rustc_const_eval_src_errors {
        include!("processed_rustc_const_eval_rustc_const_eval_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/cast.rs
    pub mod rustc_const_eval_src_interpret_cast {
        include!("processed_rustc_const_eval_src_interpret_cast.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/compare_types.rs
    pub mod rustc_const_eval_src_util_compare_types {
        include!("processed_rustc_const_eval_src_util_compare_types.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/memory.rs
    pub mod rustc_const_eval_src_interpret_memory {
        include!("processed_rustc_const_eval_src_interpret_memory.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/intrinsics.rs
    pub mod rustc_const_eval_src_interpret_intrinsics {
        include!("processed_rustc_const_eval_src_interpret_intrinsics.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/machine.rs
    pub mod rustc_const_eval_src_const_eval_machine {
        include!("processed_rustc_const_eval_src_const_eval_machine.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/check_validity_requirement.rs
    pub mod rustc_const_eval_src_util_check_validity_requirement {
        include!("processed_rustc_const_eval_src_util_check_validity_requirement.rs");
    }
}

// 47: rustc_builtin_macros (49 files)
pub mod included_rustc_builtin_macros {
    // Source: ../rust/compiler/rustc_builtin_macros/src/test_harness.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_test_harness {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_test_harness.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/clone.rs
    pub mod rustc_builtin_macros_src_deriving_clone {
        include!("processed_rustc_builtin_macros_src_deriving_clone.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg_select.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg_select {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg_select.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/log_syntax.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_log_syntax {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_log_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/asm.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_asm {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_asm.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg_accessible.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg_accessible {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg_accessible.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/define_opaque.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_define_opaque {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_define_opaque.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/partial_ord.rs
    pub mod rustc_builtin_macros_deriving_cmp_partial_ord {
        include!("processed_rustc_builtin_macros_deriving_cmp_partial_ord.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/assert/context.rs
    pub mod rustc_builtin_macros_src_assert_context {
        include!("processed_rustc_builtin_macros_src_assert_context.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/pattern_type.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_pattern_type {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_pattern_type.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/ord.rs
    pub mod rustc_builtin_macros_deriving_cmp_ord {
        include!("processed_rustc_builtin_macros_deriving_cmp_ord.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/partial_eq.rs
    pub mod rustc_builtin_macros_deriving_cmp_partial_eq {
        include!("processed_rustc_builtin_macros_deriving_cmp_partial_eq.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/alloc_error_handler.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_alloc_error_handler {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_alloc_error_handler.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/format_foreign/shell/tests.rs
    pub mod rustc_builtin_macros_format_foreign_shell_tests {
        include!("processed_rustc_builtin_macros_format_foreign_shell_tests.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/util.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_util {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_util.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/errors.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_errors {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/hash.rs
    pub mod rustc_builtin_macros_src_deriving_hash {
        include!("processed_rustc_builtin_macros_src_deriving_hash.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/debug.rs
    pub mod rustc_builtin_macros_src_deriving_debug {
        include!("processed_rustc_builtin_macros_src_deriving_debug.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs
    pub mod rustc_builtin_macros_deriving_generic_mod {
        include!("processed_rustc_builtin_macros_deriving_generic_mod.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/proc_macro_harness.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_proc_macro_harness {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_proc_macro_harness.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/contracts.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_contracts {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_contracts.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/concat.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_concat {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_concat.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/trace_macros.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_trace_macros {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_trace_macros.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/compile_error.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_compile_error {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_compile_error.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/format.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_format {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_format.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg_eval.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg_eval {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg_eval.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cmdline_attrs.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cmdline_attrs {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cmdline_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs
    pub mod rustc_builtin_macros_format_foreign_printf_tests {
        include!("processed_rustc_builtin_macros_format_foreign_printf_tests.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/default.rs
    pub mod rustc_builtin_macros_src_deriving_default {
        include!("processed_rustc_builtin_macros_src_deriving_default.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/standard_library_imports.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_standard_library_imports {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_standard_library_imports.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/concat_bytes.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_concat_bytes {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_concat_bytes.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/generic/ty.rs
    pub mod rustc_builtin_macros_deriving_generic_ty {
        include!("processed_rustc_builtin_macros_deriving_generic_ty.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/env.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_env {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_env.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/coerce_pointee.rs
    pub mod rustc_builtin_macros_src_deriving_coerce_pointee {
        include!("processed_rustc_builtin_macros_src_deriving_coerce_pointee.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/source_util.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_source_util {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_source_util.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/from.rs
    pub mod rustc_builtin_macros_src_deriving_from {
        include!("processed_rustc_builtin_macros_src_deriving_from.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/lib.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_lib {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/edition_panic.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_edition_panic {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_edition_panic.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/derive.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_derive {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_derive.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/autodiff.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_autodiff {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_autodiff.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/eq.rs
    pub mod rustc_builtin_macros_deriving_cmp_eq {
        include!("processed_rustc_builtin_macros_deriving_cmp_eq.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/mod.rs
    pub mod rustc_builtin_macros_src_deriving_mod {
        include!("processed_rustc_builtin_macros_src_deriving_mod.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/iter.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_iter {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_iter.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/bounds.rs
    pub mod rustc_builtin_macros_src_deriving_bounds {
        include!("processed_rustc_builtin_macros_src_deriving_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/global_allocator.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_global_allocator {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_global_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/assert.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_assert {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_assert.rs");
    }
}

// 48: rustc_parse (24 files)
pub mod included_rustc_parse {
    // Source: ../rust/compiler/rustc_parse/src/lexer/diagnostics.rs
    pub mod rustc_parse_src_lexer_diagnostics {
        include!("processed_rustc_parse_src_lexer_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/path.rs
    pub mod rustc_parse_src_parser_path {
        include!("processed_rustc_parse_src_parser_path.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/nonterminal.rs
    pub mod rustc_parse_src_parser_nonterminal {
        include!("processed_rustc_parse_src_parser_nonterminal.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/errors.rs
    pub mod rustc_parse_rustc_parse_src_errors {
        include!("processed_rustc_parse_rustc_parse_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/diagnostics.rs
    pub mod rustc_parse_src_parser_diagnostics {
        include!("processed_rustc_parse_src_parser_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lexer/tokentrees.rs
    pub mod rustc_parse_src_lexer_tokentrees {
        include!("processed_rustc_parse_src_lexer_tokentrees.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/attr_wrapper.rs
    pub mod rustc_parse_src_parser_attr_wrapper {
        include!("processed_rustc_parse_src_parser_attr_wrapper.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/cfg_select.rs
    pub mod rustc_parse_src_parser_cfg_select {
        include!("processed_rustc_parse_src_parser_cfg_select.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/token_type.rs
    pub mod rustc_parse_src_parser_token_type {
        include!("processed_rustc_parse_src_parser_token_type.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/stmt.rs
    pub mod rustc_parse_src_parser_stmt {
        include!("processed_rustc_parse_src_parser_stmt.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/generics.rs
    pub mod rustc_parse_src_parser_generics {
        include!("processed_rustc_parse_src_parser_generics.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/asm.rs
    pub mod rustc_parse_src_parser_asm {
        include!("processed_rustc_parse_src_parser_asm.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lib.rs
    pub mod rustc_parse_rustc_parse_src_lib {
        include!("processed_rustc_parse_rustc_parse_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs
    pub mod rustc_parse_src_lexer_unescape_error_reporting {
        include!("processed_rustc_parse_src_lexer_unescape_error_reporting.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/tokenstream/tests.rs
    pub mod rustc_parse_parser_tokenstream_tests {
        include!("processed_rustc_parse_parser_tokenstream_tests.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/ty.rs
    pub mod rustc_parse_src_parser_ty {
        include!("processed_rustc_parse_src_parser_ty.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lexer/unicode_chars.rs
    pub mod rustc_parse_src_lexer_unicode_chars {
        include!("processed_rustc_parse_src_lexer_unicode_chars.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lexer/mod.rs
    pub mod rustc_parse_src_lexer_mod {
        include!("processed_rustc_parse_src_lexer_mod.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/attr.rs
    pub mod rustc_parse_src_parser_attr {
        include!("processed_rustc_parse_src_parser_attr.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/tests.rs
    pub mod rustc_parse_src_parser_tests {
        include!("processed_rustc_parse_src_parser_tests.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/pat.rs
    pub mod rustc_parse_src_parser_pat {
        include!("processed_rustc_parse_src_parser_pat.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/expr.rs
    pub mod rustc_parse_src_parser_expr {
        include!("processed_rustc_parse_src_parser_expr.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/item.rs
    pub mod rustc_parse_src_parser_item {
        include!("processed_rustc_parse_src_parser_item.rs");
    }
}

// 49: rustc_errors (22 files)
pub mod included_rustc_errors {
    // Source: ../rust/compiler/rustc_errors/src/codes.rs
    pub mod rustc_errors_rustc_errors_src_codes {
        include!("processed_rustc_errors_rustc_errors_src_codes.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/diagnostic.rs
    pub mod rustc_errors_rustc_errors_src_diagnostic {
        include!("processed_rustc_errors_rustc_errors_src_diagnostic.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/decorate_diag.rs
    pub mod rustc_errors_rustc_errors_src_decorate_diag {
        include!("processed_rustc_errors_rustc_errors_src_decorate_diag.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/error.rs
    pub mod rustc_errors_rustc_errors_src_error {
        include!("processed_rustc_errors_rustc_errors_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/emitter.rs
    pub mod rustc_errors_rustc_errors_src_emitter {
        include!("processed_rustc_errors_rustc_errors_src_emitter.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/json/tests.rs
    pub mod rustc_errors_src_json_tests {
        include!("processed_rustc_errors_src_json_tests.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/tests.rs
    pub mod rustc_errors_rustc_errors_src_tests {
        include!("processed_rustc_errors_rustc_errors_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/timings.rs
    pub mod rustc_errors_rustc_errors_src_timings {
        include!("processed_rustc_errors_rustc_errors_src_timings.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/snippet.rs
    pub mod rustc_errors_rustc_errors_src_snippet {
        include!("processed_rustc_errors_rustc_errors_src_snippet.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/markdown/tests/term.rs
    pub mod rustc_errors_markdown_tests_term {
        include!("processed_rustc_errors_markdown_tests_term.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/registry.rs
    pub mod rustc_errors_rustc_errors_src_registry {
        include!("processed_rustc_errors_rustc_errors_src_registry.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/lock.rs
    pub mod rustc_errors_rustc_errors_src_lock {
        include!("processed_rustc_errors_rustc_errors_src_lock.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/markdown/mod.rs
    pub mod rustc_errors_src_markdown_mod {
        include!("processed_rustc_errors_src_markdown_mod.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/markdown/tests/parse.rs
    pub mod rustc_errors_markdown_tests_parse {
        include!("processed_rustc_errors_markdown_tests_parse.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/translation.rs
    pub mod rustc_errors_rustc_errors_src_translation {
        include!("processed_rustc_errors_rustc_errors_src_translation.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/diagnostic_impls.rs
    pub mod rustc_errors_rustc_errors_src_diagnostic_impls {
        include!("processed_rustc_errors_rustc_errors_src_diagnostic_impls.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/annotate_snippet_emitter_writer.rs
    pub mod rustc_errors_rustc_errors_src_annotate_snippet_emitter_writer {
        include!("processed_rustc_errors_rustc_errors_src_annotate_snippet_emitter_writer.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/styled_buffer.rs
    pub mod rustc_errors_rustc_errors_src_styled_buffer {
        include!("processed_rustc_errors_rustc_errors_src_styled_buffer.rs");
    }
}

// 50: rustc_lexer (3 files)
pub mod included_rustc_lexer {
    // Source: ../rust/compiler/rustc_lexer/src/tests.rs
    pub mod rustc_lexer_rustc_lexer_src_tests {
        include!("processed_rustc_lexer_rustc_lexer_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_lexer/src/cursor.rs
    pub mod rustc_lexer_rustc_lexer_src_cursor {
        include!("processed_rustc_lexer_rustc_lexer_src_cursor.rs");
    }
}

// 51: rustc_monomorphize (9 files)
pub mod included_rustc_monomorphize {
    // Source: ../rust/compiler/rustc_monomorphize/src/util.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_util {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_util.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/partitioning.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_partitioning {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_partitioning.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/collector.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_collector {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_collector.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/collector/autodiff.rs
    pub mod rustc_monomorphize_src_collector_autodiff {
        include!("processed_rustc_monomorphize_src_collector_autodiff.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/errors.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_errors {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/lib.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_lib {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/mod.rs
    pub mod rustc_monomorphize_src_mono_checks_mod {
        include!("processed_rustc_monomorphize_src_mono_checks_mod.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/abi_check.rs
    pub mod rustc_monomorphize_src_mono_checks_abi_check {
        include!("processed_rustc_monomorphize_src_mono_checks_abi_check.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/move_check.rs
    pub mod rustc_monomorphize_src_mono_checks_move_check {
        include!("processed_rustc_monomorphize_src_mono_checks_move_check.rs");
    }
}

// 52: rustc_privacy (2 files)
pub mod included_rustc_privacy {
    // Source: ../rust/compiler/rustc_privacy/src/errors.rs
    pub mod rustc_privacy_rustc_privacy_src_errors {
        include!("processed_rustc_privacy_rustc_privacy_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_privacy/src/lib.rs
    pub mod rustc_privacy_rustc_privacy_src_lib {
        include!("processed_rustc_privacy_rustc_privacy_src_lib.rs");
    }
}

// 53: rustc_thread_pool (28 files)
pub mod included_rustc_thread_pool {
    // Source: ../rust/compiler/rustc_thread_pool/src/sleep/mod.rs
    pub mod rustc_thread_pool_src_sleep_mod {
        include!("processed_rustc_thread_pool_src_sleep_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/spawn/tests.rs
    pub mod rustc_thread_pool_src_spawn_tests {
        include!("processed_rustc_thread_pool_src_spawn_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/registry.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_registry {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_registry.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/sleep/counters.rs
    pub mod rustc_thread_pool_src_sleep_counters {
        include!("processed_rustc_thread_pool_src_sleep_counters.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/scope/tests.rs
    pub mod rustc_thread_pool_src_scope_tests {
        include!("processed_rustc_thread_pool_src_scope_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/simple_panic.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_simple_panic {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_simple_panic.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/worker_local.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_worker_local {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/scoped_threadpool.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_scoped_threadpool {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_scoped_threadpool.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/double_init_fail.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_double_init_fail {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_double_init_fail.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/compile_fail/rc_return.rs
    pub mod rustc_thread_pool_src_compile_fail_rc_return {
        include!("processed_rustc_thread_pool_src_compile_fail_rc_return.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/job.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_job {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_job.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/thread_pool/mod.rs
    pub mod rustc_thread_pool_src_thread_pool_mod {
        include!("processed_rustc_thread_pool_src_thread_pool_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/stack_overflow_crash.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_stack_overflow_crash {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_stack_overflow_crash.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/unwind.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_unwind {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_unwind.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/broadcast/mod.rs
    pub mod rustc_thread_pool_src_broadcast_mod {
        include!("processed_rustc_thread_pool_src_broadcast_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/init_zero_threads.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_init_zero_threads {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_init_zero_threads.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/tlv.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_tlv {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_tlv.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/scope_join.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_scope_join {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_scope_join.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/lib.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_lib {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/private.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_private {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_private.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/join/tests.rs
    pub mod rustc_thread_pool_src_join_tests {
        include!("processed_rustc_thread_pool_src_join_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/latch.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_latch {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_latch.rs");
    }
}

// 54: rustc_error_messages (2 files)
pub mod included_rustc_error_messages {
    // Source: ../rust/compiler/rustc_error_messages/src/lib.rs
    pub mod rustc_error_messages_rustc_error_messages_src_lib {
        include!("processed_rustc_error_messages_rustc_error_messages_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_error_messages/src/diagnostic_impls.rs
    pub mod rustc_error_messages_rustc_error_messages_src_diagnostic_impls {
        include!("processed_rustc_error_messages_rustc_error_messages_src_diagnostic_impls.rs");
    }
}

// 55: rustc_session (18 files)
pub mod included_rustc_session {
    // Source: ../rust/compiler/rustc_session/src/cstore.rs
    pub mod rustc_session_rustc_session_src_cstore {
        include!("processed_rustc_session_rustc_session_src_cstore.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/lib.rs
    pub mod rustc_session_rustc_session_src_lib {
        include!("processed_rustc_session_rustc_session_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/filesearch.rs
    pub mod rustc_session_rustc_session_src_filesearch {
        include!("processed_rustc_session_rustc_session_src_filesearch.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/parse.rs
    pub mod rustc_session_rustc_session_src_parse {
        include!("processed_rustc_session_rustc_session_src_parse.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/code_stats.rs
    pub mod rustc_session_rustc_session_src_code_stats {
        include!("processed_rustc_session_rustc_session_src_code_stats.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/config/sigpipe.rs
    pub mod rustc_session_src_config_sigpipe {
        include!("processed_rustc_session_src_config_sigpipe.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/search_paths.rs
    pub mod rustc_session_rustc_session_src_search_paths {
        include!("processed_rustc_session_rustc_session_src_search_paths.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/config/cfg.rs
    pub mod rustc_session_src_config_cfg {
        include!("processed_rustc_session_src_config_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/utils.rs
    pub mod rustc_session_rustc_session_src_utils {
        include!("processed_rustc_session_rustc_session_src_utils.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/errors.rs
    pub mod rustc_session_rustc_session_src_errors {
        include!("processed_rustc_session_rustc_session_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/session.rs
    pub mod rustc_session_rustc_session_src_session {
        include!("processed_rustc_session_rustc_session_src_session.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/options.rs
    pub mod rustc_session_rustc_session_src_options {
        include!("processed_rustc_session_rustc_session_src_options.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/output.rs
    pub mod rustc_session_rustc_session_src_output {
        include!("processed_rustc_session_rustc_session_src_output.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/config/native_libs/tests.rs
    pub mod rustc_session_config_native_libs_tests {
        include!("processed_rustc_session_config_native_libs_tests.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/config/externs/tests.rs
    pub mod rustc_session_config_externs_tests {
        include!("processed_rustc_session_config_externs_tests.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/config.rs
    pub mod rustc_session_rustc_session_src_config {
        include!("processed_rustc_session_rustc_session_src_config.rs");
    }
}

// 56: rustc_interface (10 files)
pub mod included_rustc_interface {
    // Source: ../rust/compiler/rustc_interface/src/util.rs
    pub mod rustc_interface_rustc_interface_src_util {
        include!("processed_rustc_interface_rustc_interface_src_util.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/limits.rs
    pub mod rustc_interface_rustc_interface_src_limits {
        include!("processed_rustc_interface_rustc_interface_src_limits.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/passes.rs
    pub mod rustc_interface_rustc_interface_src_passes {
        include!("processed_rustc_interface_rustc_interface_src_passes.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/proc_macro_decls.rs
    pub mod rustc_interface_rustc_interface_src_proc_macro_decls {
        include!("processed_rustc_interface_rustc_interface_src_proc_macro_decls.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/callbacks.rs
    pub mod rustc_interface_rustc_interface_src_callbacks {
        include!("processed_rustc_interface_rustc_interface_src_callbacks.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/tests.rs
    pub mod rustc_interface_rustc_interface_src_tests {
        include!("processed_rustc_interface_rustc_interface_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/errors.rs
    pub mod rustc_interface_rustc_interface_src_errors {
        include!("processed_rustc_interface_rustc_interface_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/interface.rs
    pub mod rustc_interface_rustc_interface_src_interface {
        include!("processed_rustc_interface_rustc_interface_src_interface.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/queries.rs
    pub mod rustc_interface_rustc_interface_src_queries {
        include!("processed_rustc_interface_rustc_interface_src_queries.rs");
    }
}

