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
    // Source: ../rust/compiler/rustc_data_structures/src/unord.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unord {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unord.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/aligned.rs
    pub mod rustc_data_structures_rustc_data_structures_src_aligned {
        include!("processed_rustc_data_structures_rustc_data_structures_src_aligned.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reversed.rs
    pub mod rustc_data_structures_src_graph_reversed {
        include!("processed_rustc_data_structures_src_graph_reversed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/linux.rs
    pub mod rustc_data_structures_src_flock_linux {
        include!("processed_rustc_data_structures_src_flock_linux.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/snapshot_map/tests.rs
    pub mod rustc_data_structures_src_snapshot_map_tests {
        include!("processed_rustc_data_structures_src_snapshot_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/tests.rs
    pub mod rustc_data_structures_src_sorted_map_tests {
        include!("processed_rustc_data_structures_src_sorted_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/lock.rs
    pub mod rustc_data_structures_src_sync_lock {
        include!("processed_rustc_data_structures_src_sync_lock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/linked_graph/tests.rs
    pub mod rustc_data_structures_graph_linked_graph_tests {
        include!("processed_rustc_data_structures_graph_linked_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/atomic_ref.rs
    pub mod rustc_data_structures_rustc_data_structures_src_atomic_ref {
        include!("processed_rustc_data_structures_rustc_data_structures_src_atomic_ref.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reference.rs
    pub mod rustc_data_structures_src_graph_reference {
        include!("processed_rustc_data_structures_src_graph_reference.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/set.rs
    pub mod rustc_data_structures_src_sso_set {
        include!("processed_rustc_data_structures_src_sso_set.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stable_hasher/tests.rs
    pub mod rustc_data_structures_src_stable_hasher_tests {
        include!("processed_rustc_data_structures_src_stable_hasher_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/frozen.rs
    pub mod rustc_data_structures_rustc_data_structures_src_frozen {
        include!("processed_rustc_data_structures_rustc_data_structures_src_frozen.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/jobserver.rs
    pub mod rustc_data_structures_rustc_data_structures_src_jobserver {
        include!("processed_rustc_data_structures_rustc_data_structures_src_jobserver.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/tests.rs
    pub mod rustc_data_structures_src_graph_tests {
        include!("processed_rustc_data_structures_src_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/vec_graph/tests.rs
    pub mod rustc_data_structures_graph_vec_graph_tests {
        include!("processed_rustc_data_structures_graph_vec_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/worker_local.rs
    pub mod rustc_data_structures_src_sync_worker_local {
        include!("processed_rustc_data_structures_src_sync_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/iterate/tests.rs
    pub mod rustc_data_structures_graph_iterate_tests {
        include!("processed_rustc_data_structures_graph_iterate_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/marker.rs
    pub mod rustc_data_structures_rustc_data_structures_src_marker {
        include!("processed_rustc_data_structures_rustc_data_structures_src_marker.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/tagged_ptr/tests.rs
    pub mod rustc_data_structures_src_tagged_ptr_tests {
        include!("processed_rustc_data_structures_src_tagged_ptr_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/scc/tests.rs
    pub mod rustc_data_structures_graph_scc_tests {
        include!("processed_rustc_data_structures_graph_scc_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/tests.rs
    pub mod rustc_data_structures_src_obligation_forest_tests {
        include!("processed_rustc_data_structures_src_obligation_forest_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/dominators/tests.rs
    pub mod rustc_data_structures_graph_dominators_tests {
        include!("processed_rustc_data_structures_graph_dominators_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flock {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/mod.rs
    pub mod rustc_data_structures_src_sso_mod {
        include!("processed_rustc_data_structures_src_sso_mod.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/intern/tests.rs
    pub mod rustc_data_structures_src_intern_tests {
        include!("processed_rustc_data_structures_src_intern_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unhash.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unhash {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unhash.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/windows.rs
    pub mod rustc_data_structures_src_flock_windows {
        include!("processed_rustc_data_structures_src_flock_windows.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/steal.rs
    pub mod rustc_data_structures_rustc_data_structures_src_steal {
        include!("processed_rustc_data_structures_rustc_data_structures_src_steal.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/freeze.rs
    pub mod rustc_data_structures_src_sync_freeze {
        include!("processed_rustc_data_structures_src_sync_freeze.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sync {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sync.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/profiling/tests.rs
    pub mod rustc_data_structures_src_profiling_tests {
        include!("processed_rustc_data_structures_src_profiling_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/graphviz.rs
    pub mod rustc_data_structures_src_obligation_forest_graphviz {
        include!("processed_rustc_data_structures_src_obligation_forest_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/memmap.rs
    pub mod rustc_data_structures_rustc_data_structures_src_memmap {
        include!("processed_rustc_data_structures_rustc_data_structures_src_memmap.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thinvec.rs
    pub mod rustc_data_structures_rustc_data_structures_src_thinvec {
        include!("processed_rustc_data_structures_rustc_data_structures_src_thinvec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/lib.rs
    pub mod rustc_data_structures_rustc_data_structures_src_lib {
        include!("processed_rustc_data_structures_rustc_data_structures_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/svh.rs
    pub mod rustc_data_structures_rustc_data_structures_src_svh {
        include!("processed_rustc_data_structures_rustc_data_structures_src_svh.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fx.rs
    pub mod rustc_data_structures_rustc_data_structures_src_fx {
        include!("processed_rustc_data_structures_rustc_data_structures_src_fx.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/binary_search_util/tests.rs
    pub mod rustc_data_structures_src_binary_search_util_tests {
        include!("processed_rustc_data_structures_src_binary_search_util_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/small_c_str/tests.rs
    pub mod rustc_data_structures_src_small_c_str_tests {
        include!("processed_rustc_data_structures_src_small_c_str_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unsupported.rs
    pub mod rustc_data_structures_src_flock_unsupported {
        include!("processed_rustc_data_structures_src_flock_unsupported.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/work_queue.rs
    pub mod rustc_data_structures_rustc_data_structures_src_work_queue {
        include!("processed_rustc_data_structures_rustc_data_structures_src_work_queue.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/vec.rs
    pub mod rustc_data_structures_src_sync_vec {
        include!("processed_rustc_data_structures_src_sync_vec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stack.rs
    pub mod rustc_data_structures_rustc_data_structures_src_stack {
        include!("processed_rustc_data_structures_rustc_data_structures_src_stack.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sharded.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sharded {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sharded.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thousands/tests.rs
    pub mod rustc_data_structures_src_thousands_tests {
        include!("processed_rustc_data_structures_src_thousands_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/transitive_relation/tests.rs
    pub mod rustc_data_structures_src_transitive_relation_tests {
        include!("processed_rustc_data_structures_src_transitive_relation_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/index_map.rs
    pub mod rustc_data_structures_src_sorted_map_index_map {
        include!("processed_rustc_data_structures_src_sorted_map_index_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/packed.rs
    pub mod rustc_data_structures_rustc_data_structures_src_packed {
        include!("processed_rustc_data_structures_rustc_data_structures_src_packed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flat_map_in_place.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flat_map_in_place {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flat_map_in_place.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/map.rs
    pub mod rustc_data_structures_src_sso_map {
        include!("processed_rustc_data_structures_src_sso_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/base_n/tests.rs
    pub mod rustc_data_structures_src_base_n_tests {
        include!("processed_rustc_data_structures_src_base_n_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/owned_slice/tests.rs
    pub mod rustc_data_structures_src_owned_slice_tests {
        include!("processed_rustc_data_structures_src_owned_slice_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/union_find/tests.rs
    pub mod rustc_data_structures_src_union_find_tests {
        include!("processed_rustc_data_structures_src_union_find_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unix.rs
    pub mod rustc_data_structures_src_flock_unix {
        include!("processed_rustc_data_structures_src_flock_unix.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fingerprint/tests.rs
    pub mod rustc_data_structures_src_fingerprint_tests {
        include!("processed_rustc_data_structures_src_fingerprint_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/temp_dir.rs
    pub mod rustc_data_structures_rustc_data_structures_src_temp_dir {
        include!("processed_rustc_data_structures_rustc_data_structures_src_temp_dir.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/vec_cache/tests.rs
    pub mod rustc_data_structures_src_vec_cache_tests {
        include!("processed_rustc_data_structures_src_vec_cache_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/parallel.rs
    pub mod rustc_data_structures_src_sync_parallel {
        include!("processed_rustc_data_structures_src_sync_parallel.rs");
    }
}

// 3: rustc_index (9 files)
pub mod included_rustc_index {
    // Source: ../rust/compiler/rustc_index/src/slice.rs
    pub mod rustc_index_rustc_index_src_slice {
        include!("processed_rustc_index_rustc_index_src_slice.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/idx.rs
    pub mod rustc_index_rustc_index_src_idx {
        include!("processed_rustc_index_rustc_index_src_idx.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/lib.rs
    pub mod rustc_index_rustc_index_src_lib {
        include!("processed_rustc_index_rustc_index_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/vec/tests.rs
    pub mod rustc_index_src_vec_tests {
        include!("processed_rustc_index_src_vec_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/interval/tests.rs
    pub mod rustc_index_src_interval_tests {
        include!("processed_rustc_index_src_interval_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/bit_set/tests.rs
    pub mod rustc_index_src_bit_set_tests {
        include!("processed_rustc_index_src_bit_set_tests.rs");
    }
}

// 4: rustc_span (17 files)
pub mod included_rustc_span {
    // Source: ../rust/compiler/rustc_span/src/profiling.rs
    pub mod rustc_span_rustc_span_src_profiling {
        include!("processed_rustc_span_rustc_span_src_profiling.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/span_encoding.rs
    pub mod rustc_span_rustc_span_src_span_encoding {
        include!("processed_rustc_span_rustc_span_src_span_encoding.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/def_id.rs
    pub mod rustc_span_rustc_span_src_def_id {
        include!("processed_rustc_span_rustc_span_src_def_id.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edition.rs
    pub mod rustc_span_rustc_span_src_edition {
        include!("processed_rustc_span_rustc_span_src_edition.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/analyze_source_file/tests.rs
    pub mod rustc_span_src_analyze_source_file_tests {
        include!("processed_rustc_span_src_analyze_source_file_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/hygiene.rs
    pub mod rustc_span_rustc_span_src_hygiene {
        include!("processed_rustc_span_rustc_span_src_hygiene.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/symbol/tests.rs
    pub mod rustc_span_src_symbol_tests {
        include!("processed_rustc_span_src_symbol_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/source_map/tests.rs
    pub mod rustc_span_src_source_map_tests {
        include!("processed_rustc_span_src_source_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edit_distance/tests.rs
    pub mod rustc_span_src_edit_distance_tests {
        include!("processed_rustc_span_src_edit_distance_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/fatal_error.rs
    pub mod rustc_span_rustc_span_src_fatal_error {
        include!("processed_rustc_span_rustc_span_src_fatal_error.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/caching_source_map_view.rs
    pub mod rustc_span_rustc_span_src_caching_source_map_view {
        include!("processed_rustc_span_rustc_span_src_caching_source_map_view.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/tests.rs
    pub mod rustc_span_rustc_span_src_tests {
        include!("processed_rustc_span_rustc_span_src_tests.rs");
    }
}

// 5: rustc_transmute (8 files)
pub mod included_rustc_transmute {
    // Source: ../rust/compiler/rustc_transmute/src/layout/tree/tests.rs
    pub mod rustc_transmute_layout_tree_tests {
        include!("processed_rustc_transmute_layout_tree_tests.rs");
    }
    // Source: ../rust/compiler/rustc_transmute/src/maybe_transmutable/tests.rs
    pub mod rustc_transmute_src_maybe_transmutable_tests {
        include!("processed_rustc_transmute_src_maybe_transmutable_tests.rs");
    }
    // Source: ../rust/compiler/rustc_transmute/src/lib.rs
    pub mod rustc_transmute_rustc_transmute_src_lib {
        include!("processed_rustc_transmute_rustc_transmute_src_lib.rs");
    }
}

// 6: rustc_infer (39 files)
pub mod included_rustc_infer {
    // Source: ../rust/compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs
    pub mod rustc_infer_infer_lexical_region_resolve_mod {
        include!("processed_rustc_infer_infer_lexical_region_resolve_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/free_regions.rs
    pub mod rustc_infer_src_infer_free_regions {
        include!("processed_rustc_infer_src_infer_free_regions.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/structural_impls.rs
    pub mod rustc_infer_src_traits_structural_impls {
        include!("processed_rustc_infer_src_traits_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/util.rs
    pub mod rustc_infer_src_traits_util {
        include!("processed_rustc_infer_src_traits_util.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/project.rs
    pub mod rustc_infer_src_traits_project {
        include!("processed_rustc_infer_src_traits_project.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/unify_key.rs
    pub mod rustc_infer_src_infer_unify_key {
        include!("processed_rustc_infer_src_infer_unify_key.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/mod.rs
    pub mod rustc_infer_infer_outlives_mod {
        include!("processed_rustc_infer_infer_outlives_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/lib.rs
    pub mod rustc_infer_rustc_infer_src_lib {
        include!("processed_rustc_infer_rustc_infer_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/errors.rs
    pub mod rustc_infer_rustc_infer_src_errors {
        include!("processed_rustc_infer_rustc_infer_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/snapshot/mod.rs
    pub mod rustc_infer_infer_snapshot_mod {
        include!("processed_rustc_infer_infer_snapshot_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/mod.rs
    pub mod rustc_infer_infer_canonical_mod {
        include!("processed_rustc_infer_infer_canonical_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/instantiate.rs
    pub mod rustc_infer_infer_canonical_instantiate {
        include!("processed_rustc_infer_infer_canonical_instantiate.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/opaque_types/mod.rs
    pub mod rustc_infer_infer_opaque_types_mod {
        include!("processed_rustc_infer_infer_opaque_types_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/canonicalizer.rs
    pub mod rustc_infer_infer_canonical_canonicalizer {
        include!("processed_rustc_infer_infer_canonical_canonicalizer.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/lattice.rs
    pub mod rustc_infer_infer_relate_lattice {
        include!("processed_rustc_infer_infer_relate_lattice.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/generalize.rs
    pub mod rustc_infer_infer_relate_generalize {
        include!("processed_rustc_infer_infer_relate_generalize.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/region_constraints/mod.rs
    pub mod rustc_infer_infer_region_constraints_mod {
        include!("processed_rustc_infer_infer_region_constraints_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/mod.rs
    pub mod rustc_infer_src_infer_mod {
        include!("processed_rustc_infer_src_infer_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/region_constraints/leak_check.rs
    pub mod rustc_infer_infer_region_constraints_leak_check {
        include!("processed_rustc_infer_infer_region_constraints_leak_check.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/at.rs
    pub mod rustc_infer_src_infer_at {
        include!("processed_rustc_infer_src_infer_at.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/resolve.rs
    pub mod rustc_infer_src_infer_resolve {
        include!("processed_rustc_infer_src_infer_resolve.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/test_type_match.rs
    pub mod rustc_infer_infer_outlives_test_type_match {
        include!("processed_rustc_infer_infer_outlives_test_type_match.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/opaque_types/table.rs
    pub mod rustc_infer_infer_opaque_types_table {
        include!("processed_rustc_infer_infer_opaque_types_table.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/env.rs
    pub mod rustc_infer_infer_outlives_env {
        include!("processed_rustc_infer_infer_outlives_env.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/type_variable.rs
    pub mod rustc_infer_src_infer_type_variable {
        include!("processed_rustc_infer_src_infer_type_variable.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/query_response.rs
    pub mod rustc_infer_infer_canonical_query_response {
        include!("processed_rustc_infer_infer_canonical_query_response.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/type_relating.rs
    pub mod rustc_infer_infer_relate_type_relating {
        include!("processed_rustc_infer_infer_relate_type_relating.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/mod.rs
    pub mod rustc_infer_src_traits_mod {
        include!("processed_rustc_infer_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/snapshot/fudge.rs
    pub mod rustc_infer_infer_snapshot_fudge {
        include!("processed_rustc_infer_infer_snapshot_fudge.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/engine.rs
    pub mod rustc_infer_src_traits_engine {
        include!("processed_rustc_infer_src_traits_engine.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/obligations.rs
    pub mod rustc_infer_infer_outlives_obligations {
        include!("processed_rustc_infer_infer_outlives_obligations.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/for_liveness.rs
    pub mod rustc_infer_infer_outlives_for_liveness {
        include!("processed_rustc_infer_infer_outlives_for_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/freshen.rs
    pub mod rustc_infer_src_infer_freshen {
        include!("processed_rustc_infer_src_infer_freshen.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/context.rs
    pub mod rustc_infer_src_infer_context {
        include!("processed_rustc_infer_src_infer_context.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/higher_ranked.rs
    pub mod rustc_infer_infer_relate_higher_ranked {
        include!("processed_rustc_infer_infer_relate_higher_ranked.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/verify.rs
    pub mod rustc_infer_infer_outlives_verify {
        include!("processed_rustc_infer_infer_outlives_verify.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/projection.rs
    pub mod rustc_infer_src_infer_projection {
        include!("processed_rustc_infer_src_infer_projection.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/mod.rs
    pub mod rustc_infer_infer_relate_mod {
        include!("processed_rustc_infer_infer_relate_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/snapshot/undo_log.rs
    pub mod rustc_infer_infer_snapshot_undo_log {
        include!("processed_rustc_infer_infer_snapshot_undo_log.rs");
    }
}

// 7: rustc_ast (22 files)
pub mod included_rustc_ast {
    // Source: ../rust/compiler/rustc_ast/src/util/literal.rs
    pub mod rustc_ast_src_util_literal {
        include!("processed_rustc_ast_src_util_literal.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/mut_visit.rs
    pub mod rustc_ast_rustc_ast_src_mut_visit {
        include!("processed_rustc_ast_rustc_ast_src_mut_visit.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/case.rs
    pub mod rustc_ast_src_util_case {
        include!("processed_rustc_ast_src_util_case.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/format.rs
    pub mod rustc_ast_rustc_ast_src_format {
        include!("processed_rustc_ast_rustc_ast_src_format.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/mod.rs
    pub mod rustc_ast_src_expand_mod {
        include!("processed_rustc_ast_src_expand_mod.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/comments/tests.rs
    pub mod rustc_ast_util_comments_tests {
        include!("processed_rustc_ast_util_comments_tests.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/allocator.rs
    pub mod rustc_ast_src_expand_allocator {
        include!("processed_rustc_ast_src_expand_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/visit.rs
    pub mod rustc_ast_rustc_ast_src_visit {
        include!("processed_rustc_ast_rustc_ast_src_visit.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/ast_traits.rs
    pub mod rustc_ast_rustc_ast_src_ast_traits {
        include!("processed_rustc_ast_rustc_ast_src_ast_traits.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/classify.rs
    pub mod rustc_ast_src_util_classify {
        include!("processed_rustc_ast_src_util_classify.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/autodiff_attrs.rs
    pub mod rustc_ast_src_expand_autodiff_attrs {
        include!("processed_rustc_ast_src_expand_autodiff_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/unicode.rs
    pub mod rustc_ast_src_util_unicode {
        include!("processed_rustc_ast_src_util_unicode.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/attr/mod.rs
    pub mod rustc_ast_src_attr_mod {
        include!("processed_rustc_ast_src_attr_mod.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/entry.rs
    pub mod rustc_ast_rustc_ast_src_entry {
        include!("processed_rustc_ast_rustc_ast_src_entry.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/ast.rs
    pub mod rustc_ast_rustc_ast_src_ast {
        include!("processed_rustc_ast_rustc_ast_src_ast.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/tokenstream.rs
    pub mod rustc_ast_rustc_ast_src_tokenstream {
        include!("processed_rustc_ast_rustc_ast_src_tokenstream.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/node_id.rs
    pub mod rustc_ast_rustc_ast_src_node_id {
        include!("processed_rustc_ast_rustc_ast_src_node_id.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/token.rs
    pub mod rustc_ast_rustc_ast_src_token {
        include!("processed_rustc_ast_rustc_ast_src_token.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/parser.rs
    pub mod rustc_ast_src_util_parser {
        include!("processed_rustc_ast_src_util_parser.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/lib.rs
    pub mod rustc_ast_rustc_ast_src_lib {
        include!("processed_rustc_ast_rustc_ast_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/typetree.rs
    pub mod rustc_ast_src_expand_typetree {
        include!("processed_rustc_ast_src_expand_typetree.rs");
    }
}

// 8: rustc_lexer (3 files)
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

// 9: rustc_hir_id (1 files)
pub mod included_rustc_hir_id {
    // Source: ../rust/compiler/rustc_hir_id/src/lib.rs
    pub mod rustc_hir_id_rustc_hir_id_src_lib {
        include!("processed_rustc_hir_id_rustc_hir_id_src_lib.rs");
    }
}

// 10: rustc_serialize (8 files)
pub mod included_rustc_serialize {
    // Source: ../rust/compiler/rustc_serialize/src/serialize.rs
    pub mod rustc_serialize_rustc_serialize_src_serialize {
        include!("processed_rustc_serialize_rustc_serialize_src_serialize.rs");
    }
    // Source: ../rust/compiler/rustc_serialize/src/int_overflow.rs
    pub mod rustc_serialize_rustc_serialize_src_int_overflow {
        include!("processed_rustc_serialize_rustc_serialize_src_int_overflow.rs");
    }
    // Source: ../rust/compiler/rustc_serialize/src/leb128/tests.rs
    pub mod rustc_serialize_src_leb128_tests {
        include!("processed_rustc_serialize_src_leb128_tests.rs");
    }
    // Source: ../rust/compiler/rustc_serialize/src/opaque/mem_encoder.rs
    pub mod rustc_serialize_src_opaque_mem_encoder {
        include!("processed_rustc_serialize_src_opaque_mem_encoder.rs");
    }
    // Source: ../rust/compiler/rustc_serialize/src/opaque/tests.rs
    pub mod rustc_serialize_src_opaque_tests {
        include!("processed_rustc_serialize_src_opaque_tests.rs");
    }
}

// 11: rustc_hir_analysis (51 files)
pub mod included_rustc_hir_analysis {
    // Source: ../rust/compiler/rustc_hir_analysis/src/errors.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_errors {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/wfcheck.rs
    pub mod rustc_hir_analysis_src_check_wfcheck {
        include!("processed_rustc_hir_analysis_src_check_wfcheck.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/inherent_impls_overlap.rs
    pub mod rustc_hir_analysis_src_coherence_inherent_impls_overlap {
        include!("processed_rustc_hir_analysis_src_coherence_inherent_impls_overlap.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_collect {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_collect.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/type_of.rs
    pub mod rustc_hir_analysis_src_collect_type_of {
        include!("processed_rustc_hir_analysis_src_collect_type_of.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/mod.rs
    pub mod rustc_hir_analysis_src_variance_mod {
        include!("processed_rustc_hir_analysis_src_variance_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/intrinsic.rs
    pub mod rustc_hir_analysis_src_check_intrinsic {
        include!("processed_rustc_hir_analysis_src_check_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/errors.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_errors {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/dyn_compatibility.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_dyn_compatibility {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_dyn_compatibility.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/orphan.rs
    pub mod rustc_hir_analysis_src_coherence_orphan {
        include!("processed_rustc_hir_analysis_src_coherence_orphan.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/lint.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_lint {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_lint.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/errors/precise_captures.rs
    pub mod rustc_hir_analysis_src_errors_precise_captures {
        include!("processed_rustc_hir_analysis_src_errors_precise_captures.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/utils.rs
    pub mod rustc_hir_analysis_src_outlives_utils {
        include!("processed_rustc_hir_analysis_src_outlives_utils.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/autoderef.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_autoderef {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_autoderef.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/check.rs
    pub mod rustc_hir_analysis_src_check_check {
        include!("processed_rustc_hir_analysis_src_check_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/delegation.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_delegation {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_delegation.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/constraints.rs
    pub mod rustc_hir_analysis_src_variance_constraints {
        include!("processed_rustc_hir_analysis_src_variance_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/compare_impl_item.rs
    pub mod rustc_hir_analysis_src_check_compare_impl_item {
        include!("processed_rustc_hir_analysis_src_check_compare_impl_item.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/solve.rs
    pub mod rustc_hir_analysis_src_variance_solve {
        include!("processed_rustc_hir_analysis_src_variance_solve.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/lib.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_lib {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/dump.rs
    pub mod rustc_hir_analysis_src_collect_dump {
        include!("processed_rustc_hir_analysis_src_collect_dump.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/implicit_infer.rs
    pub mod rustc_hir_analysis_src_outlives_implicit_infer {
        include!("processed_rustc_hir_analysis_src_outlives_implicit_infer.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/terms.rs
    pub mod rustc_hir_analysis_src_variance_terms {
        include!("processed_rustc_hir_analysis_src_variance_terms.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/impl_wf_check.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_impl_wf_check {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_impl_wf_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/resolve_bound_vars.rs
    pub mod rustc_hir_analysis_src_collect_resolve_bound_vars {
        include!("processed_rustc_hir_analysis_src_collect_resolve_bound_vars.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/generics.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_generics {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_generics.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/unsafety.rs
    pub mod rustc_hir_analysis_src_coherence_unsafety {
        include!("processed_rustc_hir_analysis_src_coherence_unsafety.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/entry.rs
    pub mod rustc_hir_analysis_src_check_entry {
        include!("processed_rustc_hir_analysis_src_check_entry.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/type_of/opaque.rs
    pub mod rustc_hir_analysis_collect_type_of_opaque {
        include!("processed_rustc_hir_analysis_collect_type_of_opaque.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/mod.rs
    pub mod rustc_hir_analysis_src_outlives_mod {
        include!("processed_rustc_hir_analysis_src_outlives_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/mod.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_mod {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/dump.rs
    pub mod rustc_hir_analysis_src_outlives_dump {
        include!("processed_rustc_hir_analysis_src_outlives_dump.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/cmse.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_cmse {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_cmse.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/constrained_generic_params.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_constrained_generic_params {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_constrained_generic_params.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/item_bounds.rs
    pub mod rustc_hir_analysis_src_collect_item_bounds {
        include!("processed_rustc_hir_analysis_src_collect_item_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/always_applicable.rs
    pub mod rustc_hir_analysis_src_check_always_applicable {
        include!("processed_rustc_hir_analysis_src_check_always_applicable.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/generics_of.rs
    pub mod rustc_hir_analysis_src_collect_generics_of {
        include!("processed_rustc_hir_analysis_src_collect_generics_of.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/inherent_impls.rs
    pub mod rustc_hir_analysis_src_coherence_inherent_impls {
        include!("processed_rustc_hir_analysis_src_coherence_inherent_impls.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check_unused.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_check_unused {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_check_unused.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/predicates_of.rs
    pub mod rustc_hir_analysis_src_collect_predicates_of {
        include!("processed_rustc_hir_analysis_src_collect_predicates_of.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/explicit.rs
    pub mod rustc_hir_analysis_src_outlives_explicit {
        include!("processed_rustc_hir_analysis_src_outlives_explicit.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/bounds.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_bounds {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/errors/wrong_number_of_generic_args.rs
    pub mod rustc_hir_analysis_src_errors_wrong_number_of_generic_args {
        include!("processed_rustc_hir_analysis_src_errors_wrong_number_of_generic_args.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/mod.rs
    pub mod rustc_hir_analysis_src_coherence_mod {
        include!("processed_rustc_hir_analysis_src_coherence_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/builtin.rs
    pub mod rustc_hir_analysis_src_coherence_builtin {
        include!("processed_rustc_hir_analysis_src_coherence_builtin.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/compare_impl_item/refine.rs
    pub mod rustc_hir_analysis_check_compare_impl_item_refine {
        include!("processed_rustc_hir_analysis_check_compare_impl_item_refine.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/dump.rs
    pub mod rustc_hir_analysis_src_variance_dump {
        include!("processed_rustc_hir_analysis_src_variance_dump.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/mod.rs
    pub mod rustc_hir_analysis_src_check_mod {
        include!("processed_rustc_hir_analysis_src_check_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/impl_wf_check/min_specialization.rs
    pub mod rustc_hir_analysis_src_impl_wf_check_min_specialization {
        include!("processed_rustc_hir_analysis_src_impl_wf_check_min_specialization.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/region.rs
    pub mod rustc_hir_analysis_src_check_region {
        include!("processed_rustc_hir_analysis_src_check_region.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_wf_check.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_hir_wf_check {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_hir_wf_check.rs");
    }
}

// 12: rustc_middle (113 files)
pub mod included_rustc_middle {
    // Source: ../rust/compiler/rustc_middle/src/util/mod.rs
    pub mod rustc_middle_src_util_mod {
        include!("processed_rustc_middle_src_util_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation.rs
    pub mod rustc_middle_mir_interpret_allocation {
        include!("processed_rustc_middle_mir_interpret_allocation.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/macros.rs
    pub mod rustc_middle_rustc_middle_src_macros {
        include!("processed_rustc_middle_rustc_middle_src_macros.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/fast_reject.rs
    pub mod rustc_middle_src_ty_fast_reject {
        include!("processed_rustc_middle_src_ty_fast_reject.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/traversal.rs
    pub mod rustc_middle_src_mir_traversal {
        include!("processed_rustc_middle_src_mir_traversal.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/pattern.rs
    pub mod rustc_middle_src_ty_pattern {
        include!("processed_rustc_middle_src_ty_pattern.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs
    pub mod rustc_middle_src_ty_normalize_erasing_regions {
        include!("processed_rustc_middle_src_ty_normalize_erasing_regions.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/predicate.rs
    pub mod rustc_middle_src_ty_predicate {
        include!("processed_rustc_middle_src_ty_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/keys.rs
    pub mod rustc_middle_src_query_keys {
        include!("processed_rustc_middle_src_query_keys.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/error.rs
    pub mod rustc_middle_src_ty_error {
        include!("processed_rustc_middle_src_ty_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/kind.rs
    pub mod rustc_middle_ty_consts_kind {
        include!("processed_rustc_middle_ty_consts_kind.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/privacy.rs
    pub mod rustc_middle_src_middle_privacy {
        include!("processed_rustc_middle_src_middle_privacy.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/lint.rs
    pub mod rustc_middle_rustc_middle_src_lint {
        include!("processed_rustc_middle_rustc_middle_src_lint.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/util.rs
    pub mod rustc_middle_src_ty_util {
        include!("processed_rustc_middle_src_ty_util.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/coverage.rs
    pub mod rustc_middle_src_mir_coverage {
        include!("processed_rustc_middle_src_mir_coverage.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/elaborate_impl.rs
    pub mod rustc_middle_src_ty_elaborate_impl {
        include!("processed_rustc_middle_src_ty_elaborate_impl.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/region.rs
    pub mod rustc_middle_src_middle_region {
        include!("processed_rustc_middle_src_middle_region.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/init_mask/tests.rs
    pub mod rustc_middle_allocation_init_mask_tests {
        include!("processed_rustc_middle_allocation_init_mask_tests.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/generic_graph.rs
    pub mod rustc_middle_src_mir_generic_graph {
        include!("processed_rustc_middle_src_mir_generic_graph.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/plumbing.rs
    pub mod rustc_middle_src_query_plumbing {
        include!("processed_rustc_middle_src_query_plumbing.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/error.rs
    pub mod rustc_middle_rustc_middle_src_error {
        include!("processed_rustc_middle_rustc_middle_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/print/pretty.rs
    pub mod rustc_middle_ty_print_pretty {
        include!("processed_rustc_middle_ty_print_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/error.rs
    pub mod rustc_middle_mir_interpret_error {
        include!("processed_rustc_middle_mir_interpret_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/query.rs
    pub mod rustc_middle_src_mir_query {
        include!("processed_rustc_middle_src_mir_query.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/select.rs
    pub mod rustc_middle_src_traits_select {
        include!("processed_rustc_middle_src_traits_select.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/inhabitedness/mod.rs
    pub mod rustc_middle_ty_inhabitedness_mod {
        include!("processed_rustc_middle_ty_inhabitedness_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/closure.rs
    pub mod rustc_middle_src_ty_closure {
        include!("processed_rustc_middle_src_ty_closure.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/vtable.rs
    pub mod rustc_middle_src_ty_vtable {
        include!("processed_rustc_middle_src_ty_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/infer/canonical.rs
    pub mod rustc_middle_src_infer_canonical {
        include!("processed_rustc_middle_src_infer_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/rvalue_scopes.rs
    pub mod rustc_middle_src_ty_rvalue_scopes {
        include!("processed_rustc_middle_src_ty_rvalue_scopes.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/inhabitedness/inhabited_predicate.rs
    pub mod rustc_middle_ty_inhabitedness_inhabited_predicate {
        include!("processed_rustc_middle_ty_inhabitedness_inhabited_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/valtree.rs
    pub mod rustc_middle_ty_consts_valtree {
        include!("processed_rustc_middle_ty_consts_valtree.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/adt.rs
    pub mod rustc_middle_src_ty_adt {
        include!("processed_rustc_middle_src_ty_adt.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/adjustment.rs
    pub mod rustc_middle_src_ty_adjustment {
        include!("processed_rustc_middle_src_ty_adjustment.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/list.rs
    pub mod rustc_middle_src_ty_list {
        include!("processed_rustc_middle_src_ty_list.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/fold.rs
    pub mod rustc_middle_src_ty_fold {
        include!("processed_rustc_middle_src_ty_fold.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/lang_items.rs
    pub mod rustc_middle_src_middle_lang_items {
        include!("processed_rustc_middle_src_middle_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/metadata.rs
    pub mod rustc_middle_rustc_middle_src_metadata {
        include!("processed_rustc_middle_rustc_middle_src_metadata.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/sty.rs
    pub mod rustc_middle_src_ty_sty {
        include!("processed_rustc_middle_src_ty_sty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/place.rs
    pub mod rustc_middle_src_hir_place {
        include!("processed_rustc_middle_src_hir_place.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/codegen_fn_attrs.rs
    pub mod rustc_middle_src_middle_codegen_fn_attrs {
        include!("processed_rustc_middle_src_middle_codegen_fn_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/thir/visit.rs
    pub mod rustc_middle_src_thir_visit {
        include!("processed_rustc_middle_src_thir_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/dependency_format.rs
    pub mod rustc_middle_src_middle_dependency_format {
        include!("processed_rustc_middle_src_middle_dependency_format.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/mono.rs
    pub mod rustc_middle_src_mir_mono {
        include!("processed_rustc_middle_src_mir_mono.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/mod.rs
    pub mod rustc_middle_mir_interpret_mod {
        include!("processed_rustc_middle_mir_interpret_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/queries.rs
    pub mod rustc_middle_mir_interpret_queries {
        include!("processed_rustc_middle_mir_interpret_queries.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/assoc.rs
    pub mod rustc_middle_src_ty_assoc {
        include!("processed_rustc_middle_src_ty_assoc.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/instance.rs
    pub mod rustc_middle_src_ty_instance {
        include!("processed_rustc_middle_src_ty_instance.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/trait_def.rs
    pub mod rustc_middle_src_ty_trait_def {
        include!("processed_rustc_middle_src_ty_trait_def.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/loops.rs
    pub mod rustc_middle_src_mir_loops {
        include!("processed_rustc_middle_src_mir_loops.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/value.rs
    pub mod rustc_middle_mir_interpret_value {
        include!("processed_rustc_middle_mir_interpret_value.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/tests.rs
    pub mod rustc_middle_rustc_middle_src_tests {
        include!("processed_rustc_middle_rustc_middle_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/context/tls.rs
    pub mod rustc_middle_ty_context_tls {
        include!("processed_rustc_middle_ty_context_tls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/util/bug.rs
    pub mod rustc_middle_src_util_bug {
        include!("processed_rustc_middle_src_util_bug.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/debugger_visualizer.rs
    pub mod rustc_middle_src_middle_debugger_visualizer {
        include!("processed_rustc_middle_src_middle_debugger_visualizer.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/visit.rs
    pub mod rustc_middle_src_ty_visit {
        include!("processed_rustc_middle_src_ty_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/resolve_bound_vars.rs
    pub mod rustc_middle_src_middle_resolve_bound_vars {
        include!("processed_rustc_middle_src_middle_resolve_bound_vars.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/generic_args.rs
    pub mod rustc_middle_src_ty_generic_args {
        include!("processed_rustc_middle_src_ty_generic_args.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/query.rs
    pub mod rustc_middle_src_traits_query {
        include!("processed_rustc_middle_src_traits_query.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/provenance_map.rs
    pub mod rustc_middle_interpret_allocation_provenance_map {
        include!("processed_rustc_middle_interpret_allocation_provenance_map.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/dep_graph/dep_node.rs
    pub mod rustc_middle_src_dep_graph_dep_node {
        include!("processed_rustc_middle_src_dep_graph_dep_node.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/codec.rs
    pub mod rustc_middle_src_ty_codec {
        include!("processed_rustc_middle_src_ty_codec.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/solve.rs
    pub mod rustc_middle_src_traits_solve {
        include!("processed_rustc_middle_src_traits_solve.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/generic_graphviz.rs
    pub mod rustc_middle_src_mir_generic_graphviz {
        include!("processed_rustc_middle_src_mir_generic_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/relate.rs
    pub mod rustc_middle_src_ty_relate {
        include!("processed_rustc_middle_src_ty_relate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/erase_regions.rs
    pub mod rustc_middle_src_ty_erase_regions {
        include!("processed_rustc_middle_src_ty_erase_regions.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/cast.rs
    pub mod rustc_middle_src_ty_cast {
        include!("processed_rustc_middle_src_ty_cast.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/pretty.rs
    pub mod rustc_middle_src_mir_pretty {
        include!("processed_rustc_middle_src_mir_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/visit.rs
    pub mod rustc_middle_src_mir_visit {
        include!("processed_rustc_middle_src_mir_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/structural_impls.rs
    pub mod rustc_middle_src_ty_structural_impls {
        include!("processed_rustc_middle_src_ty_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/stability.rs
    pub mod rustc_middle_src_middle_stability {
        include!("processed_rustc_middle_src_middle_stability.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/consts.rs
    pub mod rustc_middle_src_mir_consts {
        include!("processed_rustc_middle_src_mir_consts.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/values.rs
    pub mod rustc_middle_rustc_middle_src_values {
        include!("processed_rustc_middle_rustc_middle_src_values.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/opaque_types.rs
    pub mod rustc_middle_src_ty_opaque_types {
        include!("processed_rustc_middle_src_ty_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/mod.rs
    pub mod rustc_middle_src_traits_mod {
        include!("processed_rustc_middle_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/statement.rs
    pub mod rustc_middle_src_mir_statement {
        include!("processed_rustc_middle_src_mir_statement.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/structural_impls.rs
    pub mod rustc_middle_src_traits_structural_impls {
        include!("processed_rustc_middle_src_traits_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/exported_symbols.rs
    pub mod rustc_middle_src_middle_exported_symbols {
        include!("processed_rustc_middle_src_middle_exported_symbols.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts.rs
    pub mod rustc_middle_src_ty_consts {
        include!("processed_rustc_middle_src_ty_consts.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/intrinsic.rs
    pub mod rustc_middle_src_ty_intrinsic {
        include!("processed_rustc_middle_src_ty_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/significant_drop_order.rs
    pub mod rustc_middle_src_ty_significant_drop_order {
        include!("processed_rustc_middle_src_ty_significant_drop_order.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/terminator.rs
    pub mod rustc_middle_src_mir_terminator {
        include!("processed_rustc_middle_src_mir_terminator.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/int.rs
    pub mod rustc_middle_ty_consts_int {
        include!("processed_rustc_middle_ty_consts_int.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/typeck_results.rs
    pub mod rustc_middle_src_ty_typeck_results {
        include!("processed_rustc_middle_src_ty_typeck_results.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/diagnostics.rs
    pub mod rustc_middle_src_ty_diagnostics {
        include!("processed_rustc_middle_src_ty_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/nested_filter.rs
    pub mod rustc_middle_src_hir_nested_filter {
        include!("processed_rustc_middle_src_hir_nested_filter.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/pointer.rs
    pub mod rustc_middle_mir_interpret_pointer {
        include!("processed_rustc_middle_mir_interpret_pointer.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/generics.rs
    pub mod rustc_middle_src_ty_generics {
        include!("processed_rustc_middle_src_ty_generics.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/dep_graph/mod.rs
    pub mod rustc_middle_src_dep_graph_mod {
        include!("processed_rustc_middle_src_dep_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/abstract_const.rs
    pub mod rustc_middle_src_ty_abstract_const {
        include!("processed_rustc_middle_src_ty_abstract_const.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hooks/mod.rs
    pub mod rustc_middle_src_hooks_mod {
        include!("processed_rustc_middle_src_hooks_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/basic_blocks.rs
    pub mod rustc_middle_src_mir_basic_blocks {
        include!("processed_rustc_middle_src_mir_basic_blocks.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/thir.rs
    pub mod rustc_middle_rustc_middle_src_thir {
        include!("processed_rustc_middle_rustc_middle_src_thir.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/mod.rs
    pub mod rustc_middle_src_middle_mod {
        include!("processed_rustc_middle_src_middle_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/specialization_graph.rs
    pub mod rustc_middle_src_traits_specialization_graph {
        include!("processed_rustc_middle_src_traits_specialization_graph.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/context.rs
    pub mod rustc_middle_src_ty_context {
        include!("processed_rustc_middle_src_ty_context.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/on_disk_cache.rs
    pub mod rustc_middle_src_query_on_disk_cache {
        include!("processed_rustc_middle_src_query_on_disk_cache.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/graphviz.rs
    pub mod rustc_middle_src_mir_graphviz {
        include!("processed_rustc_middle_src_mir_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/region.rs
    pub mod rustc_middle_src_ty_region {
        include!("processed_rustc_middle_src_ty_region.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/mod.rs
    pub mod rustc_middle_src_hir_mod {
        include!("processed_rustc_middle_src_hir_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/impls_ty.rs
    pub mod rustc_middle_src_ty_impls_ty {
        include!("processed_rustc_middle_src_ty_impls_ty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/print/mod.rs
    pub mod rustc_middle_ty_print_mod {
        include!("processed_rustc_middle_ty_print_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/map.rs
    pub mod rustc_middle_src_hir_map {
        include!("processed_rustc_middle_src_hir_map.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/mod.rs
    pub mod rustc_middle_src_ty_mod {
        include!("processed_rustc_middle_src_ty_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/layout.rs
    pub mod rustc_middle_src_ty_layout {
        include!("processed_rustc_middle_src_ty_layout.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/syntax.rs
    pub mod rustc_middle_src_mir_syntax {
        include!("processed_rustc_middle_src_mir_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/arena_cached.rs
    pub mod rustc_middle_src_query_arena_cached {
        include!("processed_rustc_middle_src_query_arena_cached.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/erase.rs
    pub mod rustc_middle_src_query_erase {
        include!("processed_rustc_middle_src_query_erase.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/mod.rs
    pub mod rustc_middle_src_query_mod {
        include!("processed_rustc_middle_src_query_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/arena.rs
    pub mod rustc_middle_rustc_middle_src_arena {
        include!("processed_rustc_middle_rustc_middle_src_arena.rs");
    }
}

// 13: rustc_baked_icu_data (2 files)
pub mod included_rustc_baked_icu_data {
    // Source: ../rust/compiler/rustc_baked_icu_data/src/data/mod.rs
    pub mod rustc_baked_icu_data_src_data_mod {
        include!("processed_rustc_baked_icu_data_src_data_mod.rs");
    }
    // Source: ../rust/compiler/rustc_baked_icu_data/src/lib.rs
    pub mod rustc_baked_icu_data_rustc_baked_icu_data_src_lib {
        include!("processed_rustc_baked_icu_data_rustc_baked_icu_data_src_lib.rs");
    }
}

// 14: rustc_symbol_mangling (7 files)
pub mod included_rustc_symbol_mangling {
    // Source: ../rust/compiler/rustc_symbol_mangling/src/legacy.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_legacy {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_legacy.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/errors.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_errors {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/test.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_test {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_test.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/lib.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_lib {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/export.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_export {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_export.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/hashed.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_hashed {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_hashed.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/v0.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_v0 {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_v0.rs");
    }
}

// 15: rustc_driver (1 files)
pub mod included_rustc_driver {
    // Source: ../rust/compiler/rustc_driver/src/lib.rs
    pub mod rustc_driver_rustc_driver_src_lib {
        include!("processed_rustc_driver_rustc_driver_src_lib.rs");
    }
}

// 16: rustc_hashes (1 files)
pub mod included_rustc_hashes {
    // Source: ../rust/compiler/rustc_hashes/src/lib.rs
    pub mod rustc_hashes_rustc_hashes_src_lib {
        include!("processed_rustc_hashes_rustc_hashes_src_lib.rs");
    }
}

// 17: rustc (2 files)
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

// 18: rustc_ty_utils (17 files)
pub mod included_rustc_ty_utils {
    // Source: ../rust/compiler/rustc_ty_utils/src/common_traits.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_common_traits {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_common_traits.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/opaque_types.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_opaque_types {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/lib.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_lib {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/structural_match.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_structural_match {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_structural_match.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/nested_bodies.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_nested_bodies {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_nested_bodies.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/layout/invariant.rs
    pub mod rustc_ty_utils_src_layout_invariant {
        include!("processed_rustc_ty_utils_src_layout_invariant.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/consts.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_consts {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_consts.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/abi.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_abi {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_abi.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/sig_types.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_sig_types {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_sig_types.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/assoc.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_assoc {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_assoc.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/needs_drop.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_needs_drop {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_needs_drop.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/representability.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_representability {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_representability.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/layout.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_layout {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_layout.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/ty.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_ty {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_ty.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/errors.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_errors {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/instance.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_instance {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_instance.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/implied_bounds.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_implied_bounds {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_implied_bounds.rs");
    }
}

// 19: rustc_error_codes (1 files)
pub mod included_rustc_error_codes {
    // Source: ../rust/compiler/rustc_error_codes/src/lib.rs
    pub mod rustc_error_codes_rustc_error_codes_src_lib {
        include!("processed_rustc_error_codes_rustc_error_codes_src_lib.rs");
    }
}

// 20: rustc_interface (10 files)
pub mod included_rustc_interface {
    // Source: ../rust/compiler/rustc_interface/src/callbacks.rs
    pub mod rustc_interface_rustc_interface_src_callbacks {
        include!("processed_rustc_interface_rustc_interface_src_callbacks.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/queries.rs
    pub mod rustc_interface_rustc_interface_src_queries {
        include!("processed_rustc_interface_rustc_interface_src_queries.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/passes.rs
    pub mod rustc_interface_rustc_interface_src_passes {
        include!("processed_rustc_interface_rustc_interface_src_passes.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/limits.rs
    pub mod rustc_interface_rustc_interface_src_limits {
        include!("processed_rustc_interface_rustc_interface_src_limits.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/errors.rs
    pub mod rustc_interface_rustc_interface_src_errors {
        include!("processed_rustc_interface_rustc_interface_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/proc_macro_decls.rs
    pub mod rustc_interface_rustc_interface_src_proc_macro_decls {
        include!("processed_rustc_interface_rustc_interface_src_proc_macro_decls.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/tests.rs
    pub mod rustc_interface_rustc_interface_src_tests {
        include!("processed_rustc_interface_rustc_interface_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/util.rs
    pub mod rustc_interface_rustc_interface_src_util {
        include!("processed_rustc_interface_rustc_interface_src_util.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/interface.rs
    pub mod rustc_interface_rustc_interface_src_interface {
        include!("processed_rustc_interface_rustc_interface_src_interface.rs");
    }
}

// 21: rustc_monomorphize (9 files)
pub mod included_rustc_monomorphize {
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/move_check.rs
    pub mod rustc_monomorphize_src_mono_checks_move_check {
        include!("processed_rustc_monomorphize_src_mono_checks_move_check.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/partitioning.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_partitioning {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_partitioning.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/lib.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_lib {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/mod.rs
    pub mod rustc_monomorphize_src_mono_checks_mod {
        include!("processed_rustc_monomorphize_src_mono_checks_mod.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/util.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_util {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_util.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/collector/autodiff.rs
    pub mod rustc_monomorphize_src_collector_autodiff {
        include!("processed_rustc_monomorphize_src_collector_autodiff.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/abi_check.rs
    pub mod rustc_monomorphize_src_mono_checks_abi_check {
        include!("processed_rustc_monomorphize_src_mono_checks_abi_check.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/errors.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_errors {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/collector.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_collector {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_collector.rs");
    }
}

// 22: rustc_const_eval (40 files)
pub mod included_rustc_const_eval {
    // Source: ../rust/compiler/rustc_const_eval/src/util/mod.rs
    pub mod rustc_const_eval_src_util_mod {
        include!("processed_rustc_const_eval_src_util_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/check.rs
    pub mod rustc_const_eval_src_check_consts_check {
        include!("processed_rustc_const_eval_src_check_consts_check.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/compare_types.rs
    pub mod rustc_const_eval_src_util_compare_types {
        include!("processed_rustc_const_eval_src_util_compare_types.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/traits.rs
    pub mod rustc_const_eval_src_interpret_traits {
        include!("processed_rustc_const_eval_src_interpret_traits.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/cast.rs
    pub mod rustc_const_eval_src_interpret_cast {
        include!("processed_rustc_const_eval_src_interpret_cast.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/alignment.rs
    pub mod rustc_const_eval_src_util_alignment {
        include!("processed_rustc_const_eval_src_util_alignment.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/mod.rs
    pub mod rustc_const_eval_src_interpret_mod {
        include!("processed_rustc_const_eval_src_interpret_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/memory.rs
    pub mod rustc_const_eval_src_interpret_memory {
        include!("processed_rustc_const_eval_src_interpret_memory.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/step.rs
    pub mod rustc_const_eval_src_interpret_step {
        include!("processed_rustc_const_eval_src_interpret_step.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/valtrees.rs
    pub mod rustc_const_eval_src_const_eval_valtrees {
        include!("processed_rustc_const_eval_src_const_eval_valtrees.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/intern.rs
    pub mod rustc_const_eval_src_interpret_intern {
        include!("processed_rustc_const_eval_src_interpret_intern.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/resolver.rs
    pub mod rustc_const_eval_src_check_consts_resolver {
        include!("processed_rustc_const_eval_src_check_consts_resolver.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/validity.rs
    pub mod rustc_const_eval_src_interpret_validity {
        include!("processed_rustc_const_eval_src_interpret_validity.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/eval_context.rs
    pub mod rustc_const_eval_src_interpret_eval_context {
        include!("processed_rustc_const_eval_src_interpret_eval_context.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/fn_queries.rs
    pub mod rustc_const_eval_src_const_eval_fn_queries {
        include!("processed_rustc_const_eval_src_const_eval_fn_queries.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/machine.rs
    pub mod rustc_const_eval_src_interpret_machine {
        include!("processed_rustc_const_eval_src_interpret_machine.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/errors.rs
    pub mod rustc_const_eval_rustc_const_eval_src_errors {
        include!("processed_rustc_const_eval_rustc_const_eval_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/error.rs
    pub mod rustc_const_eval_src_const_eval_error {
        include!("processed_rustc_const_eval_src_const_eval_error.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/dummy_machine.rs
    pub mod rustc_const_eval_src_const_eval_dummy_machine {
        include!("processed_rustc_const_eval_src_const_eval_dummy_machine.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/util.rs
    pub mod rustc_const_eval_src_interpret_util {
        include!("processed_rustc_const_eval_src_interpret_util.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/projection.rs
    pub mod rustc_const_eval_src_interpret_projection {
        include!("processed_rustc_const_eval_src_interpret_projection.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/qualifs.rs
    pub mod rustc_const_eval_src_check_consts_qualifs {
        include!("processed_rustc_const_eval_src_check_consts_qualifs.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/post_drop_elaboration.rs
    pub mod rustc_const_eval_src_check_consts_post_drop_elaboration {
        include!("processed_rustc_const_eval_src_check_consts_post_drop_elaboration.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/stack.rs
    pub mod rustc_const_eval_src_interpret_stack {
        include!("processed_rustc_const_eval_src_interpret_stack.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/type_name.rs
    pub mod rustc_const_eval_src_util_type_name {
        include!("processed_rustc_const_eval_src_util_type_name.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/intrinsics.rs
    pub mod rustc_const_eval_src_interpret_intrinsics {
        include!("processed_rustc_const_eval_src_interpret_intrinsics.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/eval_queries.rs
    pub mod rustc_const_eval_src_const_eval_eval_queries {
        include!("processed_rustc_const_eval_src_const_eval_eval_queries.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/call.rs
    pub mod rustc_const_eval_src_interpret_call {
        include!("processed_rustc_const_eval_src_interpret_call.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/machine.rs
    pub mod rustc_const_eval_src_const_eval_machine {
        include!("processed_rustc_const_eval_src_const_eval_machine.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/operand.rs
    pub mod rustc_const_eval_src_interpret_operand {
        include!("processed_rustc_const_eval_src_interpret_operand.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/mod.rs
    pub mod rustc_const_eval_src_const_eval_mod {
        include!("processed_rustc_const_eval_src_const_eval_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/lib.rs
    pub mod rustc_const_eval_rustc_const_eval_src_lib {
        include!("processed_rustc_const_eval_rustc_const_eval_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/caller_location.rs
    pub mod rustc_const_eval_src_util_caller_location {
        include!("processed_rustc_const_eval_src_util_caller_location.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/check_validity_requirement.rs
    pub mod rustc_const_eval_src_util_check_validity_requirement {
        include!("processed_rustc_const_eval_src_util_check_validity_requirement.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/place.rs
    pub mod rustc_const_eval_src_interpret_place {
        include!("processed_rustc_const_eval_src_interpret_place.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/ops.rs
    pub mod rustc_const_eval_src_check_consts_ops {
        include!("processed_rustc_const_eval_src_check_consts_ops.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/operator.rs
    pub mod rustc_const_eval_src_interpret_operator {
        include!("processed_rustc_const_eval_src_interpret_operator.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/visitor.rs
    pub mod rustc_const_eval_src_interpret_visitor {
        include!("processed_rustc_const_eval_src_interpret_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/discriminant.rs
    pub mod rustc_const_eval_src_interpret_discriminant {
        include!("processed_rustc_const_eval_src_interpret_discriminant.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/mod.rs
    pub mod rustc_const_eval_src_check_consts_mod {
        include!("processed_rustc_const_eval_src_check_consts_mod.rs");
    }
}

// 23: rustc_codegen_cranelift (74 files)
pub mod included_rustc_codegen_cranelift {
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/issue-72793.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue_72793 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue-72793.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/emit.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_emit {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_emit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/num.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_num {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_num.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/aot.rs
    pub mod rustc_codegen_cranelift_src_driver_aot {
        include!("processed_rustc_codegen_cranelift_src_driver_aot.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/mini_core_hello_world.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core_hello_world {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core_hello_world.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/tests.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_tests {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/linkage.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_linkage {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_linkage.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/pass_mode.rs
    pub mod rustc_codegen_cranelift_src_abi_pass_mode {
        include!("processed_rustc_codegen_cranelift_src_abi_pass_mode.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/discriminant.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_discriminant {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_discriminant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/subslice-patterns-const-eval.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_subslice_patterns_const_eval {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_subslice-patterns-const-eval.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/vtable.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_vtable {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/float-minmax-pass.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_float_minmax_pass {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_float-minmax-pass.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/compiler_builtins.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_compiler_builtins {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_compiler_builtins.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/jit.rs
    pub mod rustc_codegen_cranelift_src_driver_jit {
        include!("processed_rustc_codegen_cranelift_src_driver_jit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/common.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_common {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/comments.rs
    pub mod rustc_codegen_cranelift_src_abi_comments {
        include!("processed_rustc_codegen_cranelift_src_abi_comments.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/simd.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_simd {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_simd.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/neon.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_neon {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_neon.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/track-caller-attribute.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_track_caller_attribute {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_track-caller-attribute.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/constant.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_constant {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_constant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/std_example.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_std_example {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_std_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/concurrency_limiter.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_concurrency_limiter {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_concurrency_limiter.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/shared_utils.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_shared_utils {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_shared_utils.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/build_sysroot.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_sysroot {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_sysroot.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/mini_core.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/mod.rs
    pub mod rustc_codegen_cranelift_src_abi_mod {
        include!("processed_rustc_codegen_cranelift_src_abi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/codegen_f16_f128.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_f16_f128 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_f16_f128.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/dst-field-align.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_dst_field_align {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_dst-field-align.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/unsize.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_unsize {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_unsize.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/pretty_clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_pretty_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_pretty_clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm_aarch64.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm_aarch64 {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm_aarch64.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/bench.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_bench {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_bench.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/rustdoc-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustdoc_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustdoc-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/line_info.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_line_info {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_line_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/analyze.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_analyze {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_analyze.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/build_backend.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_backend {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_backend.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/object.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_object {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_object.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/utils.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_utils {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_utils.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/mod.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_mod {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/toolchain.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_toolchain {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_toolchain.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/codegen_i128.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_i128 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_i128.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/types.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_types {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_types.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/config.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_config {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_config.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/unwind_module.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_unwind_module {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_unwind_module.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/rustc-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustc_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustc-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/config.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_config {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_config.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/abi_cafe.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_abi_cafe {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_abi_cafe.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/inline_asm.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_inline_asm {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_inline_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/value_and_place.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_value_and_place {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_value_and_place.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/arbitrary_self_types_pointers_and_wrappers.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_arbitrary_self_types_pointers_and_wrappers {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_arbitrary_self_types_pointers_and_wrappers.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/raw-dylib.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_raw_dylib {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_raw-dylib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/issue-59326.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue_59326 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue-59326.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/main_shim.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_main_shim {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_main_shim.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/optimize/peephole.rs
    pub mod rustc_codegen_cranelift_src_optimize_peephole {
        include!("processed_rustc_codegen_cranelift_src_optimize_peephole.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/rustc_info.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_rustc_info {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_rustc_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/gen_block_iterate.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_gen_block_iterate {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_gen_block_iterate.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/returning.rs
    pub mod rustc_codegen_cranelift_src_abi_returning {
        include!("processed_rustc_codegen_cranelift_src_abi_returning.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/main.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_main {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_main.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/example.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_example {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/lib.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_lib {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/filter_profile.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_filter_profile {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_filter_profile.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/path.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_path {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_path.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/global_asm.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_global_asm {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_global_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/allocator.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_allocator {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/unwind.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_unwind {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_unwind.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/cargo-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_cargo_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_cargo-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/cast.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_cast {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_cast.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/pointer.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_pointer {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_pointer.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/mod.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_mod {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/base.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_base {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm_x86.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm_x86 {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm_x86.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/prepare.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_prepare {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_prepare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/mod.rs
    pub mod rustc_codegen_cranelift_src_driver_mod {
        include!("processed_rustc_codegen_cranelift_src_driver_mod.rs");
    }
}

// 24: rustc_public (26 files)
pub mod included_rustc_public {
    // Source: ../rust/compiler/rustc_public/src/error.rs
    pub mod rustc_public_rustc_public_src_error {
        include!("processed_rustc_public_rustc_public_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/internal_cx/mod.rs
    pub mod rustc_public_unstable_internal_cx_mod {
        include!("processed_rustc_public_unstable_internal_cx_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/mod.rs
    pub mod rustc_public_unstable_convert_mod {
        include!("processed_rustc_public_unstable_convert_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/stable/abi.rs
    pub mod rustc_public_convert_stable_abi {
        include!("processed_rustc_public_convert_stable_abi.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/body.rs
    pub mod rustc_public_src_mir_body {
        include!("processed_rustc_public_src_mir_body.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/stable/mir.rs
    pub mod rustc_public_convert_stable_mir {
        include!("processed_rustc_public_convert_stable_mir.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/visitor.rs
    pub mod rustc_public_rustc_public_src_visitor {
        include!("processed_rustc_public_rustc_public_src_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/mod.rs
    pub mod rustc_public_src_unstable_mod {
        include!("processed_rustc_public_src_unstable_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/rustc_internal/pretty.rs
    pub mod rustc_public_src_rustc_internal_pretty {
        include!("processed_rustc_public_src_rustc_internal_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/lib.rs
    pub mod rustc_public_rustc_public_src_lib {
        include!("processed_rustc_public_rustc_public_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/target.rs
    pub mod rustc_public_rustc_public_src_target {
        include!("processed_rustc_public_rustc_public_src_target.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/alloc.rs
    pub mod rustc_public_src_mir_alloc {
        include!("processed_rustc_public_src_mir_alloc.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir.rs
    pub mod rustc_public_rustc_public_src_mir {
        include!("processed_rustc_public_rustc_public_src_mir.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/rustc_internal/mod.rs
    pub mod rustc_public_src_rustc_internal_mod {
        include!("processed_rustc_public_src_rustc_internal_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/compiler_interface.rs
    pub mod rustc_public_rustc_public_src_compiler_interface {
        include!("processed_rustc_public_rustc_public_src_compiler_interface.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/stable/mod.rs
    pub mod rustc_public_convert_stable_mod {
        include!("processed_rustc_public_convert_stable_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/stable/ty.rs
    pub mod rustc_public_convert_stable_ty {
        include!("processed_rustc_public_convert_stable_ty.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/alloc.rs
    pub mod rustc_public_rustc_public_src_alloc {
        include!("processed_rustc_public_rustc_public_src_alloc.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/internal_cx/helpers.rs
    pub mod rustc_public_unstable_internal_cx_helpers {
        include!("processed_rustc_public_unstable_internal_cx_helpers.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/abi.rs
    pub mod rustc_public_rustc_public_src_abi {
        include!("processed_rustc_public_rustc_public_src_abi.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/crate_def.rs
    pub mod rustc_public_rustc_public_src_crate_def {
        include!("processed_rustc_public_rustc_public_src_crate_def.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/ty.rs
    pub mod rustc_public_rustc_public_src_ty {
        include!("processed_rustc_public_rustc_public_src_ty.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/visit.rs
    pub mod rustc_public_src_mir_visit {
        include!("processed_rustc_public_src_mir_visit.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/pretty.rs
    pub mod rustc_public_src_mir_pretty {
        include!("processed_rustc_public_src_mir_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/mono.rs
    pub mod rustc_public_src_mir_mono {
        include!("processed_rustc_public_src_mir_mono.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/internal.rs
    pub mod rustc_public_unstable_convert_internal {
        include!("processed_rustc_public_unstable_convert_internal.rs");
    }
}

// 25: rustc_mir_dataflow (24 files)
pub mod included_rustc_mir_dataflow {
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/mod.rs
    pub mod rustc_mir_dataflow_src_impls_mod {
        include!("processed_rustc_mir_dataflow_src_impls_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/storage_liveness.rs
    pub mod rustc_mir_dataflow_src_impls_storage_liveness {
        include!("processed_rustc_mir_dataflow_src_impls_storage_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/drop_flag_effects.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_drop_flag_effects {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_drop_flag_effects.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/errors.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_errors {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/liveness.rs
    pub mod rustc_mir_dataflow_src_impls_liveness {
        include!("processed_rustc_mir_dataflow_src_impls_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/un_derefer.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_un_derefer {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_un_derefer.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/debuginfo.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_debuginfo {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/visitor.rs
    pub mod rustc_mir_dataflow_src_framework_visitor {
        include!("processed_rustc_mir_dataflow_src_framework_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/fmt.rs
    pub mod rustc_mir_dataflow_src_framework_fmt {
        include!("processed_rustc_mir_dataflow_src_framework_fmt.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/move_paths/mod.rs
    pub mod rustc_mir_dataflow_src_move_paths_mod {
        include!("processed_rustc_mir_dataflow_src_move_paths_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/direction.rs
    pub mod rustc_mir_dataflow_src_framework_direction {
        include!("processed_rustc_mir_dataflow_src_framework_direction.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/move_paths/builder.rs
    pub mod rustc_mir_dataflow_src_move_paths_builder {
        include!("processed_rustc_mir_dataflow_src_move_paths_builder.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/tests.rs
    pub mod rustc_mir_dataflow_src_framework_tests {
        include!("processed_rustc_mir_dataflow_src_framework_tests.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/borrowed_locals.rs
    pub mod rustc_mir_dataflow_src_impls_borrowed_locals {
        include!("processed_rustc_mir_dataflow_src_impls_borrowed_locals.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/rustc_peek.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_rustc_peek {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_rustc_peek.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/graphviz.rs
    pub mod rustc_mir_dataflow_src_framework_graphviz {
        include!("processed_rustc_mir_dataflow_src_framework_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/lattice.rs
    pub mod rustc_mir_dataflow_src_framework_lattice {
        include!("processed_rustc_mir_dataflow_src_framework_lattice.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/initialized.rs
    pub mod rustc_mir_dataflow_src_impls_initialized {
        include!("processed_rustc_mir_dataflow_src_impls_initialized.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/lib.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_lib {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/points.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_points {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_points.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/results.rs
    pub mod rustc_mir_dataflow_src_framework_results {
        include!("processed_rustc_mir_dataflow_src_framework_results.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/value_analysis.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_value_analysis {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_value_analysis.rs");
    }
}

// 26: rustc_codegen_ssa (54 files)
pub mod included_rustc_codegen_ssa {
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/coverageinfo.rs
    pub mod rustc_codegen_ssa_src_traits_coverageinfo {
        include!("processed_rustc_codegen_ssa_src_traits_coverageinfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/abi.rs
    pub mod rustc_codegen_ssa_src_traits_abi {
        include!("processed_rustc_codegen_ssa_src_traits_abi.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/asm.rs
    pub mod rustc_codegen_ssa_src_traits_asm {
        include!("processed_rustc_codegen_ssa_src_traits_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/write.rs
    pub mod rustc_codegen_ssa_src_traits_write {
        include!("processed_rustc_codegen_ssa_src_traits_write.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/archive.rs
    pub mod rustc_codegen_ssa_src_back_archive {
        include!("processed_rustc_codegen_ssa_src_back_archive.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/linker/tests.rs
    pub mod rustc_codegen_ssa_back_linker_tests {
        include!("processed_rustc_codegen_ssa_back_linker_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/intrinsic.rs
    pub mod rustc_codegen_ssa_src_mir_intrinsic {
        include!("processed_rustc_codegen_ssa_src_mir_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/size_of_val.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_size_of_val {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_size_of_val.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/apple/tests.rs
    pub mod rustc_codegen_ssa_back_apple_tests {
        include!("processed_rustc_codegen_ssa_back_apple_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/builder.rs
    pub mod rustc_codegen_ssa_src_traits_builder {
        include!("processed_rustc_codegen_ssa_src_traits_builder.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/statics.rs
    pub mod rustc_codegen_ssa_src_traits_statics {
        include!("processed_rustc_codegen_ssa_src_traits_statics.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/coverageinfo.rs
    pub mod rustc_codegen_ssa_src_mir_coverageinfo {
        include!("processed_rustc_codegen_ssa_src_mir_coverageinfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/target_features.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_target_features {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_target_features.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/place.rs
    pub mod rustc_codegen_ssa_src_mir_place {
        include!("processed_rustc_codegen_ssa_src_mir_place.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/type_.rs
    pub mod rustc_codegen_ssa_src_traits_type_ {
        include!("processed_rustc_codegen_ssa_src_traits_type_.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/mod.rs
    pub mod rustc_codegen_ssa_src_traits_mod {
        include!("processed_rustc_codegen_ssa_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/misc.rs
    pub mod rustc_codegen_ssa_src_traits_misc {
        include!("processed_rustc_codegen_ssa_src_traits_misc.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/write.rs
    pub mod rustc_codegen_ssa_src_back_write {
        include!("processed_rustc_codegen_ssa_src_back_write.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/constant.rs
    pub mod rustc_codegen_ssa_src_mir_constant {
        include!("processed_rustc_codegen_ssa_src_mir_constant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/lib.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_lib {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/assert_module_sources.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_assert_module_sources {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_assert_module_sources.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/link.rs
    pub mod rustc_codegen_ssa_src_back_link {
        include!("processed_rustc_codegen_ssa_src_back_link.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/backend.rs
    pub mod rustc_codegen_ssa_src_traits_backend {
        include!("processed_rustc_codegen_ssa_src_traits_backend.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/debuginfo/mod.rs
    pub mod rustc_codegen_ssa_src_debuginfo_mod {
        include!("processed_rustc_codegen_ssa_src_debuginfo_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/common.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_common {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/meth.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_meth {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_meth.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/codegen_attrs.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_codegen_attrs {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_codegen_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/mod.rs
    pub mod rustc_codegen_ssa_src_mir_mod {
        include!("processed_rustc_codegen_ssa_src_mir_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/debuginfo.rs
    pub mod rustc_codegen_ssa_src_traits_debuginfo {
        include!("processed_rustc_codegen_ssa_src_traits_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/intrinsic.rs
    pub mod rustc_codegen_ssa_src_traits_intrinsic {
        include!("processed_rustc_codegen_ssa_src_traits_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/metadata.rs
    pub mod rustc_codegen_ssa_src_back_metadata {
        include!("processed_rustc_codegen_ssa_src_back_metadata.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/consts.rs
    pub mod rustc_codegen_ssa_src_traits_consts {
        include!("processed_rustc_codegen_ssa_src_traits_consts.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs
    pub mod rustc_codegen_ssa_src_debuginfo_type_names {
        include!("processed_rustc_codegen_ssa_src_debuginfo_type_names.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/link/raw_dylib.rs
    pub mod rustc_codegen_ssa_back_link_raw_dylib {
        include!("processed_rustc_codegen_ssa_back_link_raw_dylib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/base.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_base {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/operand.rs
    pub mod rustc_codegen_ssa_src_mir_operand {
        include!("processed_rustc_codegen_ssa_src_mir_operand.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/locals.rs
    pub mod rustc_codegen_ssa_src_mir_locals {
        include!("processed_rustc_codegen_ssa_src_mir_locals.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/rvalue.rs
    pub mod rustc_codegen_ssa_src_mir_rvalue {
        include!("processed_rustc_codegen_ssa_src_mir_rvalue.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mono_item.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_mono_item {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_mono_item.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/analyze.rs
    pub mod rustc_codegen_ssa_src_mir_analyze {
        include!("processed_rustc_codegen_ssa_src_mir_analyze.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/statement.rs
    pub mod rustc_codegen_ssa_src_mir_statement {
        include!("processed_rustc_codegen_ssa_src_mir_statement.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/naked_asm.rs
    pub mod rustc_codegen_ssa_src_mir_naked_asm {
        include!("processed_rustc_codegen_ssa_src_mir_naked_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/declare.rs
    pub mod rustc_codegen_ssa_src_traits_declare {
        include!("processed_rustc_codegen_ssa_src_traits_declare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/errors.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_errors {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/command.rs
    pub mod rustc_codegen_ssa_src_back_command {
        include!("processed_rustc_codegen_ssa_src_back_command.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/rpath/tests.rs
    pub mod rustc_codegen_ssa_back_rpath_tests {
        include!("processed_rustc_codegen_ssa_back_rpath_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/symbol_export.rs
    pub mod rustc_codegen_ssa_src_back_symbol_export {
        include!("processed_rustc_codegen_ssa_src_back_symbol_export.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/debuginfo.rs
    pub mod rustc_codegen_ssa_src_mir_debuginfo {
        include!("processed_rustc_codegen_ssa_src_mir_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/lto.rs
    pub mod rustc_codegen_ssa_src_back_lto {
        include!("processed_rustc_codegen_ssa_src_back_lto.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/block.rs
    pub mod rustc_codegen_ssa_src_mir_block {
        include!("processed_rustc_codegen_ssa_src_mir_block.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/mod.rs
    pub mod rustc_codegen_ssa_src_back_mod {
        include!("processed_rustc_codegen_ssa_src_back_mod.rs");
    }
}

// 27: rustc_privacy (2 files)
pub mod included_rustc_privacy {
    // Source: ../rust/compiler/rustc_privacy/src/errors.rs
    pub mod rustc_privacy_rustc_privacy_src_errors {
        include!("processed_rustc_privacy_rustc_privacy_src_errors.rs");
    }
