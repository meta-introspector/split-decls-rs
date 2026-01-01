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
    // Source: ../rust/compiler/rustc_data_structures/src/marker.rs
    pub mod rustc_data_structures_rustc_data_structures_src_marker {
        include!("processed_rustc_data_structures_rustc_data_structures_src_marker.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/work_queue.rs
    pub mod rustc_data_structures_rustc_data_structures_src_work_queue {
        include!("processed_rustc_data_structures_rustc_data_structures_src_work_queue.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/memmap.rs
    pub mod rustc_data_structures_rustc_data_structures_src_memmap {
        include!("processed_rustc_data_structures_rustc_data_structures_src_memmap.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/owned_slice/tests.rs
    pub mod rustc_data_structures_src_owned_slice_tests {
        include!("processed_rustc_data_structures_src_owned_slice_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stack.rs
    pub mod rustc_data_structures_rustc_data_structures_src_stack {
        include!("processed_rustc_data_structures_rustc_data_structures_src_stack.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/packed.rs
    pub mod rustc_data_structures_rustc_data_structures_src_packed {
        include!("processed_rustc_data_structures_rustc_data_structures_src_packed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sync {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sync.rs");
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
    // Source: ../rust/compiler/rustc_data_structures/src/sync/worker_local.rs
    pub mod rustc_data_structures_src_sync_worker_local {
        include!("processed_rustc_data_structures_src_sync_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unord.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unord {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unord.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unhash.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unhash {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unhash.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/index_map.rs
    pub mod rustc_data_structures_src_sorted_map_index_map {
        include!("processed_rustc_data_structures_src_sorted_map_index_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/intern/tests.rs
    pub mod rustc_data_structures_src_intern_tests {
        include!("processed_rustc_data_structures_src_intern_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/snapshot_map/tests.rs
    pub mod rustc_data_structures_src_snapshot_map_tests {
        include!("processed_rustc_data_structures_src_snapshot_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/parallel.rs
    pub mod rustc_data_structures_src_sync_parallel {
        include!("processed_rustc_data_structures_src_sync_parallel.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/tests.rs
    pub mod rustc_data_structures_src_graph_tests {
        include!("processed_rustc_data_structures_src_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/steal.rs
    pub mod rustc_data_structures_rustc_data_structures_src_steal {
        include!("processed_rustc_data_structures_rustc_data_structures_src_steal.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/vec_cache/tests.rs
    pub mod rustc_data_structures_src_vec_cache_tests {
        include!("processed_rustc_data_structures_src_vec_cache_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thinvec.rs
    pub mod rustc_data_structures_rustc_data_structures_src_thinvec {
        include!("processed_rustc_data_structures_rustc_data_structures_src_thinvec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/jobserver.rs
    pub mod rustc_data_structures_rustc_data_structures_src_jobserver {
        include!("processed_rustc_data_structures_rustc_data_structures_src_jobserver.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/lock.rs
    pub mod rustc_data_structures_src_sync_lock {
        include!("processed_rustc_data_structures_src_sync_lock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thousands/tests.rs
    pub mod rustc_data_structures_src_thousands_tests {
        include!("processed_rustc_data_structures_src_thousands_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/vec.rs
    pub mod rustc_data_structures_src_sync_vec {
        include!("processed_rustc_data_structures_src_sync_vec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flat_map_in_place.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flat_map_in_place {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flat_map_in_place.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/tagged_ptr/tests.rs
    pub mod rustc_data_structures_src_tagged_ptr_tests {
        include!("processed_rustc_data_structures_src_tagged_ptr_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/dominators/tests.rs
    pub mod rustc_data_structures_graph_dominators_tests {
        include!("processed_rustc_data_structures_graph_dominators_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/vec_graph/tests.rs
    pub mod rustc_data_structures_graph_vec_graph_tests {
        include!("processed_rustc_data_structures_graph_vec_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/tests.rs
    pub mod rustc_data_structures_src_sorted_map_tests {
        include!("processed_rustc_data_structures_src_sorted_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/base_n/tests.rs
    pub mod rustc_data_structures_src_base_n_tests {
        include!("processed_rustc_data_structures_src_base_n_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/union_find/tests.rs
    pub mod rustc_data_structures_src_union_find_tests {
        include!("processed_rustc_data_structures_src_union_find_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flock {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unix.rs
    pub mod rustc_data_structures_src_flock_unix {
        include!("processed_rustc_data_structures_src_flock_unix.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/lib.rs
    pub mod rustc_data_structures_rustc_data_structures_src_lib {
        include!("processed_rustc_data_structures_rustc_data_structures_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fingerprint/tests.rs
    pub mod rustc_data_structures_src_fingerprint_tests {
        include!("processed_rustc_data_structures_src_fingerprint_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/scc/tests.rs
    pub mod rustc_data_structures_graph_scc_tests {
        include!("processed_rustc_data_structures_graph_scc_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/temp_dir.rs
    pub mod rustc_data_structures_rustc_data_structures_src_temp_dir {
        include!("processed_rustc_data_structures_rustc_data_structures_src_temp_dir.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/profiling/tests.rs
    pub mod rustc_data_structures_src_profiling_tests {
        include!("processed_rustc_data_structures_src_profiling_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/iterate/tests.rs
    pub mod rustc_data_structures_graph_iterate_tests {
        include!("processed_rustc_data_structures_graph_iterate_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/set.rs
    pub mod rustc_data_structures_src_sso_set {
        include!("processed_rustc_data_structures_src_sso_set.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/small_c_str/tests.rs
    pub mod rustc_data_structures_src_small_c_str_tests {
        include!("processed_rustc_data_structures_src_small_c_str_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/aligned.rs
    pub mod rustc_data_structures_rustc_data_structures_src_aligned {
        include!("processed_rustc_data_structures_rustc_data_structures_src_aligned.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unsupported.rs
    pub mod rustc_data_structures_src_flock_unsupported {
        include!("processed_rustc_data_structures_src_flock_unsupported.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/mod.rs
    pub mod rustc_data_structures_src_sso_mod {
        include!("processed_rustc_data_structures_src_sso_mod.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sharded.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sharded {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sharded.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/linked_graph/tests.rs
    pub mod rustc_data_structures_graph_linked_graph_tests {
        include!("processed_rustc_data_structures_graph_linked_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/map.rs
    pub mod rustc_data_structures_src_sso_map {
        include!("processed_rustc_data_structures_src_sso_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/freeze.rs
    pub mod rustc_data_structures_src_sync_freeze {
        include!("processed_rustc_data_structures_src_sync_freeze.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reversed.rs
    pub mod rustc_data_structures_src_graph_reversed {
        include!("processed_rustc_data_structures_src_graph_reversed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/windows.rs
    pub mod rustc_data_structures_src_flock_windows {
        include!("processed_rustc_data_structures_src_flock_windows.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/atomic_ref.rs
    pub mod rustc_data_structures_rustc_data_structures_src_atomic_ref {
        include!("processed_rustc_data_structures_rustc_data_structures_src_atomic_ref.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stable_hasher/tests.rs
    pub mod rustc_data_structures_src_stable_hasher_tests {
        include!("processed_rustc_data_structures_src_stable_hasher_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/linux.rs
    pub mod rustc_data_structures_src_flock_linux {
        include!("processed_rustc_data_structures_src_flock_linux.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/tests.rs
    pub mod rustc_data_structures_src_obligation_forest_tests {
        include!("processed_rustc_data_structures_src_obligation_forest_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/frozen.rs
    pub mod rustc_data_structures_rustc_data_structures_src_frozen {
        include!("processed_rustc_data_structures_rustc_data_structures_src_frozen.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reference.rs
    pub mod rustc_data_structures_src_graph_reference {
        include!("processed_rustc_data_structures_src_graph_reference.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/transitive_relation/tests.rs
    pub mod rustc_data_structures_src_transitive_relation_tests {
        include!("processed_rustc_data_structures_src_transitive_relation_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/graphviz.rs
    pub mod rustc_data_structures_src_obligation_forest_graphviz {
        include!("processed_rustc_data_structures_src_obligation_forest_graphviz.rs");
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
    // Source: ../rust/compiler/rustc_index/src/vec/tests.rs
    pub mod rustc_index_src_vec_tests {
        include!("processed_rustc_index_src_vec_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/interval/tests.rs
    pub mod rustc_index_src_interval_tests {
        include!("processed_rustc_index_src_interval_tests.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/slice.rs
    pub mod rustc_index_rustc_index_src_slice {
        include!("processed_rustc_index_rustc_index_src_slice.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/idx.rs
    pub mod rustc_index_rustc_index_src_idx {
        include!("processed_rustc_index_rustc_index_src_idx.rs");
    }
}

// 4: rustc_span (17 files)
pub mod included_rustc_span {
    // Source: ../rust/compiler/rustc_span/src/analyze_source_file/tests.rs
    pub mod rustc_span_src_analyze_source_file_tests {
        include!("processed_rustc_span_src_analyze_source_file_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/fatal_error.rs
    pub mod rustc_span_rustc_span_src_fatal_error {
        include!("processed_rustc_span_rustc_span_src_fatal_error.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/def_id.rs
    pub mod rustc_span_rustc_span_src_def_id {
        include!("processed_rustc_span_rustc_span_src_def_id.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/hygiene.rs
    pub mod rustc_span_rustc_span_src_hygiene {
        include!("processed_rustc_span_rustc_span_src_hygiene.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/tests.rs
    pub mod rustc_span_rustc_span_src_tests {
        include!("processed_rustc_span_rustc_span_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/source_map/tests.rs
    pub mod rustc_span_src_source_map_tests {
        include!("processed_rustc_span_src_source_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edit_distance/tests.rs
    pub mod rustc_span_src_edit_distance_tests {
        include!("processed_rustc_span_src_edit_distance_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/caching_source_map_view.rs
    pub mod rustc_span_rustc_span_src_caching_source_map_view {
        include!("processed_rustc_span_rustc_span_src_caching_source_map_view.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/span_encoding.rs
    pub mod rustc_span_rustc_span_src_span_encoding {
        include!("processed_rustc_span_rustc_span_src_span_encoding.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/profiling.rs
    pub mod rustc_span_rustc_span_src_profiling {
        include!("processed_rustc_span_rustc_span_src_profiling.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edition.rs
    pub mod rustc_span_rustc_span_src_edition {
        include!("processed_rustc_span_rustc_span_src_edition.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/symbol/tests.rs
    pub mod rustc_span_src_symbol_tests {
        include!("processed_rustc_span_src_symbol_tests.rs");
    }
}

// 5: rustc_resolve (14 files)
pub mod included_rustc_resolve {
    // Source: ../rust/compiler/rustc_resolve/src/imports.rs
    pub mod rustc_resolve_rustc_resolve_src_imports {
        include!("processed_rustc_resolve_rustc_resolve_src_imports.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/def_collector.rs
    pub mod rustc_resolve_rustc_resolve_src_def_collector {
        include!("processed_rustc_resolve_rustc_resolve_src_def_collector.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/late.rs
    pub mod rustc_resolve_rustc_resolve_src_late {
        include!("processed_rustc_resolve_rustc_resolve_src_late.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/rustdoc/tests.rs
    pub mod rustc_resolve_src_rustdoc_tests {
        include!("processed_rustc_resolve_src_rustdoc_tests.rs");
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
    // Source: ../rust/compiler/rustc_resolve/src/errors.rs
    pub mod rustc_resolve_rustc_resolve_src_errors {
        include!("processed_rustc_resolve_rustc_resolve_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/macros.rs
    pub mod rustc_resolve_rustc_resolve_src_macros {
        include!("processed_rustc_resolve_rustc_resolve_src_macros.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/build_reduced_graph.rs
    pub mod rustc_resolve_rustc_resolve_src_build_reduced_graph {
        include!("processed_rustc_resolve_rustc_resolve_src_build_reduced_graph.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/lib.rs
    pub mod rustc_resolve_rustc_resolve_src_lib {
        include!("processed_rustc_resolve_rustc_resolve_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_resolve/src/diagnostics.rs
    pub mod rustc_resolve_rustc_resolve_src_diagnostics {
        include!("processed_rustc_resolve_rustc_resolve_src_diagnostics.rs");
    }
}

// 6: rustc_lexer (3 files)
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

// 7: rustc_metadata (15 files)
pub mod included_rustc_metadata {
    // Source: ../rust/compiler/rustc_metadata/src/native_libs.rs
    pub mod rustc_metadata_rustc_metadata_src_native_libs {
        include!("processed_rustc_metadata_rustc_metadata_src_native_libs.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs
    pub mod rustc_metadata_rmeta_decoder_cstore_impl {
        include!("processed_rustc_metadata_rmeta_decoder_cstore_impl.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/lib.rs
    pub mod rustc_metadata_rustc_metadata_src_lib {
        include!("processed_rustc_metadata_rustc_metadata_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/rmeta/encoder.rs
    pub mod rustc_metadata_src_rmeta_encoder {
        include!("processed_rustc_metadata_src_rmeta_encoder.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/rmeta/mod.rs
    pub mod rustc_metadata_src_rmeta_mod {
        include!("processed_rustc_metadata_src_rmeta_mod.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/locator.rs
    pub mod rustc_metadata_rustc_metadata_src_locator {
        include!("processed_rustc_metadata_rustc_metadata_src_locator.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/fs.rs
    pub mod rustc_metadata_rustc_metadata_src_fs {
        include!("processed_rustc_metadata_rustc_metadata_src_fs.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/rmeta/parameterized.rs
    pub mod rustc_metadata_src_rmeta_parameterized {
        include!("processed_rustc_metadata_src_rmeta_parameterized.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/creader.rs
    pub mod rustc_metadata_rustc_metadata_src_creader {
        include!("processed_rustc_metadata_rustc_metadata_src_creader.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/rmeta/table.rs
    pub mod rustc_metadata_src_rmeta_table {
        include!("processed_rustc_metadata_src_rmeta_table.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/rmeta/decoder.rs
    pub mod rustc_metadata_src_rmeta_decoder {
        include!("processed_rustc_metadata_src_rmeta_decoder.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/dependency_format.rs
    pub mod rustc_metadata_rustc_metadata_src_dependency_format {
        include!("processed_rustc_metadata_rustc_metadata_src_dependency_format.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/rmeta/def_path_hash_map.rs
    pub mod rustc_metadata_src_rmeta_def_path_hash_map {
        include!("processed_rustc_metadata_src_rmeta_def_path_hash_map.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/foreign_modules.rs
    pub mod rustc_metadata_rustc_metadata_src_foreign_modules {
        include!("processed_rustc_metadata_rustc_metadata_src_foreign_modules.rs");
    }
    // Source: ../rust/compiler/rustc_metadata/src/errors.rs
    pub mod rustc_metadata_rustc_metadata_src_errors {
        include!("processed_rustc_metadata_rustc_metadata_src_errors.rs");
    }
}

// 8: rustc_driver (1 files)
pub mod included_rustc_driver {
    // Source: ../rust/compiler/rustc_driver/src/lib.rs
    pub mod rustc_driver_rustc_driver_src_lib {
        include!("processed_rustc_driver_rustc_driver_src_lib.rs");
    }
}

// 9: rustc_log (1 files)
pub mod included_rustc_log {
    // Source: ../rust/compiler/rustc_log/src/lib.rs
    pub mod rustc_log_rustc_log_src_lib {
        include!("processed_rustc_log_rustc_log_src_lib.rs");
    }
}

// 10: rustc_lint_defs (2 files)
pub mod included_rustc_lint_defs {
    // Source: ../rust/compiler/rustc_lint_defs/src/builtin.rs
    pub mod rustc_lint_defs_rustc_lint_defs_src_builtin {
        include!("processed_rustc_lint_defs_rustc_lint_defs_src_builtin.rs");
    }
    // Source: ../rust/compiler/rustc_lint_defs/src/lib.rs
    pub mod rustc_lint_defs_rustc_lint_defs_src_lib {
        include!("processed_rustc_lint_defs_rustc_lint_defs_src_lib.rs");
    }
}

// 11: rustc_symbol_mangling (7 files)
pub mod included_rustc_symbol_mangling {
    // Source: ../rust/compiler/rustc_symbol_mangling/src/test.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_test {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_test.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/lib.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_lib {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/legacy.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_legacy {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_legacy.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/errors.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_errors {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/hashed.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_hashed {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_hashed.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/export.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_export {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_export.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/v0.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_v0 {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_v0.rs");
    }
}

// 12: rustc_hir_typeck (40 files)
pub mod included_rustc_hir_typeck {
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/mod.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_mod {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/typeck_root_ctxt.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_typeck_root_ctxt {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_typeck_root_ctxt.rs");
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
    // Source: ../rust/compiler/rustc_hir_typeck/src/check.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_check {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/writeback.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_writeback {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_writeback.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs
    pub mod rustc_hir_typeck_src_fn_ctxt__impl {
        include!("processed_rustc_hir_typeck_src_fn_ctxt__impl.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/probe.rs
    pub mod rustc_hir_typeck_src_method_probe {
        include!("processed_rustc_hir_typeck_src_method_probe.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/_match.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src__match {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src__match.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/expr_use_visitor.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_expr_use_visitor {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_expr_use_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fallback.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_fallback {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_fallback.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/gather_locals.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_gather_locals {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_gather_locals.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/closure.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_closure {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_closure.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/inline_asm.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_inline_asm {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_inline_asm.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/diverges.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_diverges {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_diverges.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/pat.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_pat {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_pat.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/inspect_obligations.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_inspect_obligations {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_inspect_obligations.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/callee.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_callee {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_callee.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/upvar.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_upvar {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_upvar.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/prelude_edition_lints.rs
    pub mod rustc_hir_typeck_src_method_prelude_edition_lints {
        include!("processed_rustc_hir_typeck_src_method_prelude_edition_lints.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/op.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_op {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_op.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/checks.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_checks {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_checks.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/mod.rs
    pub mod rustc_hir_typeck_src_method_mod {
        include!("processed_rustc_hir_typeck_src_method_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/suggestions.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_suggestions {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_suggestions.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/errors.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_errors {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/intrinsicck.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_intrinsicck {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_intrinsicck.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/rvalue_scopes.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_rvalue_scopes {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_rvalue_scopes.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/opaque_types.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_opaque_types {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/confirm.rs
    pub mod rustc_hir_typeck_src_method_confirm {
        include!("processed_rustc_hir_typeck_src_method_confirm.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/arg_matrix.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_arg_matrix {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_arg_matrix.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/place_op.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_place_op {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_place_op.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/method/suggest.rs
    pub mod rustc_hir_typeck_src_method_suggest {
        include!("processed_rustc_hir_typeck_src_method_suggest.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/naked_functions.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_naked_functions {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_naked_functions.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/autoderef.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_autoderef {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_autoderef.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/expectation.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_expectation {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_expectation.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/loops.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_loops {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_loops.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/demand.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_demand {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_demand.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/fn_ctxt/adjust_fulfillment_errors.rs
    pub mod rustc_hir_typeck_src_fn_ctxt_adjust_fulfillment_errors {
        include!("processed_rustc_hir_typeck_src_fn_ctxt_adjust_fulfillment_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_typeck/src/expr.rs
    pub mod rustc_hir_typeck_rustc_hir_typeck_src_expr {
        include!("processed_rustc_hir_typeck_rustc_hir_typeck_src_expr.rs");
    }
}

// 13: rustc_codegen_ssa (54 files)
pub mod included_rustc_codegen_ssa {
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/backend.rs
    pub mod rustc_codegen_ssa_src_traits_backend {
        include!("processed_rustc_codegen_ssa_src_traits_backend.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/operand.rs
    pub mod rustc_codegen_ssa_src_mir_operand {
        include!("processed_rustc_codegen_ssa_src_mir_operand.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/mod.rs
    pub mod rustc_codegen_ssa_src_mir_mod {
        include!("processed_rustc_codegen_ssa_src_mir_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/lib.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_lib {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/type_.rs
    pub mod rustc_codegen_ssa_src_traits_type_ {
        include!("processed_rustc_codegen_ssa_src_traits_type_.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/mod.rs
    pub mod rustc_codegen_ssa_src_back_mod {
        include!("processed_rustc_codegen_ssa_src_back_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/place.rs
    pub mod rustc_codegen_ssa_src_mir_place {
        include!("processed_rustc_codegen_ssa_src_mir_place.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/link.rs
    pub mod rustc_codegen_ssa_src_back_link {
        include!("processed_rustc_codegen_ssa_src_back_link.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/codegen_attrs.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_codegen_attrs {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_codegen_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/naked_asm.rs
    pub mod rustc_codegen_ssa_src_mir_naked_asm {
        include!("processed_rustc_codegen_ssa_src_mir_naked_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/archive.rs
    pub mod rustc_codegen_ssa_src_back_archive {
        include!("processed_rustc_codegen_ssa_src_back_archive.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/coverageinfo.rs
    pub mod rustc_codegen_ssa_src_mir_coverageinfo {
        include!("processed_rustc_codegen_ssa_src_mir_coverageinfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/lto.rs
    pub mod rustc_codegen_ssa_src_back_lto {
        include!("processed_rustc_codegen_ssa_src_back_lto.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/write.rs
    pub mod rustc_codegen_ssa_src_back_write {
        include!("processed_rustc_codegen_ssa_src_back_write.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/coverageinfo.rs
    pub mod rustc_codegen_ssa_src_traits_coverageinfo {
        include!("processed_rustc_codegen_ssa_src_traits_coverageinfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/analyze.rs
    pub mod rustc_codegen_ssa_src_mir_analyze {
        include!("processed_rustc_codegen_ssa_src_mir_analyze.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/builder.rs
    pub mod rustc_codegen_ssa_src_traits_builder {
        include!("processed_rustc_codegen_ssa_src_traits_builder.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/symbol_export.rs
    pub mod rustc_codegen_ssa_src_back_symbol_export {
        include!("processed_rustc_codegen_ssa_src_back_symbol_export.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs
    pub mod rustc_codegen_ssa_src_debuginfo_type_names {
        include!("processed_rustc_codegen_ssa_src_debuginfo_type_names.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/abi.rs
    pub mod rustc_codegen_ssa_src_traits_abi {
        include!("processed_rustc_codegen_ssa_src_traits_abi.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/debuginfo/mod.rs
    pub mod rustc_codegen_ssa_src_debuginfo_mod {
        include!("processed_rustc_codegen_ssa_src_debuginfo_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/block.rs
    pub mod rustc_codegen_ssa_src_mir_block {
        include!("processed_rustc_codegen_ssa_src_mir_block.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/size_of_val.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_size_of_val {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_size_of_val.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/asm.rs
    pub mod rustc_codegen_ssa_src_traits_asm {
        include!("processed_rustc_codegen_ssa_src_traits_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/apple/tests.rs
    pub mod rustc_codegen_ssa_back_apple_tests {
        include!("processed_rustc_codegen_ssa_back_apple_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/command.rs
    pub mod rustc_codegen_ssa_src_back_command {
        include!("processed_rustc_codegen_ssa_src_back_command.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/link/raw_dylib.rs
    pub mod rustc_codegen_ssa_back_link_raw_dylib {
        include!("processed_rustc_codegen_ssa_back_link_raw_dylib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/base.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_base {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/intrinsic.rs
    pub mod rustc_codegen_ssa_src_traits_intrinsic {
        include!("processed_rustc_codegen_ssa_src_traits_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/consts.rs
    pub mod rustc_codegen_ssa_src_traits_consts {
        include!("processed_rustc_codegen_ssa_src_traits_consts.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/declare.rs
    pub mod rustc_codegen_ssa_src_traits_declare {
        include!("processed_rustc_codegen_ssa_src_traits_declare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/metadata.rs
    pub mod rustc_codegen_ssa_src_back_metadata {
        include!("processed_rustc_codegen_ssa_src_back_metadata.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/statement.rs
    pub mod rustc_codegen_ssa_src_mir_statement {
        include!("processed_rustc_codegen_ssa_src_mir_statement.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mono_item.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_mono_item {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_mono_item.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/common.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_common {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/constant.rs
    pub mod rustc_codegen_ssa_src_mir_constant {
        include!("processed_rustc_codegen_ssa_src_mir_constant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/debuginfo.rs
    pub mod rustc_codegen_ssa_src_mir_debuginfo {
        include!("processed_rustc_codegen_ssa_src_mir_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/statics.rs
    pub mod rustc_codegen_ssa_src_traits_statics {
        include!("processed_rustc_codegen_ssa_src_traits_statics.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/meth.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_meth {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_meth.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/locals.rs
    pub mod rustc_codegen_ssa_src_mir_locals {
        include!("processed_rustc_codegen_ssa_src_mir_locals.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/assert_module_sources.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_assert_module_sources {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_assert_module_sources.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/intrinsic.rs
    pub mod rustc_codegen_ssa_src_mir_intrinsic {
        include!("processed_rustc_codegen_ssa_src_mir_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/debuginfo.rs
    pub mod rustc_codegen_ssa_src_traits_debuginfo {
        include!("processed_rustc_codegen_ssa_src_traits_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/errors.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_errors {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/rpath/tests.rs
    pub mod rustc_codegen_ssa_back_rpath_tests {
        include!("processed_rustc_codegen_ssa_back_rpath_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/linker/tests.rs
    pub mod rustc_codegen_ssa_back_linker_tests {
        include!("processed_rustc_codegen_ssa_back_linker_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/target_features.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_target_features {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_target_features.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/rvalue.rs
    pub mod rustc_codegen_ssa_src_mir_rvalue {
        include!("processed_rustc_codegen_ssa_src_mir_rvalue.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/misc.rs
    pub mod rustc_codegen_ssa_src_traits_misc {
        include!("processed_rustc_codegen_ssa_src_traits_misc.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/write.rs
    pub mod rustc_codegen_ssa_src_traits_write {
        include!("processed_rustc_codegen_ssa_src_traits_write.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/mod.rs
    pub mod rustc_codegen_ssa_src_traits_mod {
        include!("processed_rustc_codegen_ssa_src_traits_mod.rs");
    }
}

// 14: rustc_public_bridge (7 files)
pub mod included_rustc_public_bridge {
    // Source: ../rust/compiler/rustc_public_bridge/src/context/impls.rs
    pub mod rustc_public_bridge_src_context_impls {
        include!("processed_rustc_public_bridge_src_context_impls.rs");
    }
    // Source: ../rust/compiler/rustc_public_bridge/src/bridge.rs
    pub mod rustc_public_bridge_rustc_public_bridge_src_bridge {
        include!("processed_rustc_public_bridge_rustc_public_bridge_src_bridge.rs");
    }
    // Source: ../rust/compiler/rustc_public_bridge/src/lib.rs
    pub mod rustc_public_bridge_rustc_public_bridge_src_lib {
        include!("processed_rustc_public_bridge_rustc_public_bridge_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_public_bridge/src/context/mod.rs
    pub mod rustc_public_bridge_src_context_mod {
        include!("processed_rustc_public_bridge_src_context_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public_bridge/src/alloc.rs
    pub mod rustc_public_bridge_rustc_public_bridge_src_alloc {
        include!("processed_rustc_public_bridge_rustc_public_bridge_src_alloc.rs");
    }
    // Source: ../rust/compiler/rustc_public_bridge/src/builder.rs
    pub mod rustc_public_bridge_rustc_public_bridge_src_builder {
        include!("processed_rustc_public_bridge_rustc_public_bridge_src_builder.rs");
    }
    // Source: ../rust/compiler/rustc_public_bridge/src/context/helpers.rs
    pub mod rustc_public_bridge_src_context_helpers {
        include!("processed_rustc_public_bridge_src_context_helpers.rs");
    }
}

// 15: rustc_index_macros (2 files)
pub mod included_rustc_index_macros {
    // Source: ../rust/compiler/rustc_index_macros/src/lib.rs
    pub mod rustc_index_macros_rustc_index_macros_src_lib {
        include!("processed_rustc_index_macros_rustc_index_macros_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_index_macros/src/newtype.rs
    pub mod rustc_index_macros_rustc_index_macros_src_newtype {
        include!("processed_rustc_index_macros_rustc_index_macros_src_newtype.rs");
    }
}

// 16: rustc_incremental (12 files)
pub mod included_rustc_incremental {
    // Source: ../rust/compiler/rustc_incremental/src/errors.rs
    pub mod rustc_incremental_rustc_incremental_src_errors {
        include!("processed_rustc_incremental_rustc_incremental_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/load.rs
    pub mod rustc_incremental_src_persist_load {
        include!("processed_rustc_incremental_src_persist_load.rs");
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
    // Source: ../rust/compiler/rustc_incremental/src/assert_dep_graph.rs
    pub mod rustc_incremental_rustc_incremental_src_assert_dep_graph {
        include!("processed_rustc_incremental_rustc_incremental_src_assert_dep_graph.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/mod.rs
    pub mod rustc_incremental_src_persist_mod {
        include!("processed_rustc_incremental_src_persist_mod.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/work_product.rs
    pub mod rustc_incremental_src_persist_work_product {
        include!("processed_rustc_incremental_src_persist_work_product.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/data.rs
    pub mod rustc_incremental_src_persist_data {
        include!("processed_rustc_incremental_src_persist_data.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/save.rs
    pub mod rustc_incremental_src_persist_save {
        include!("processed_rustc_incremental_src_persist_save.rs");
    }
    // Source: ../rust/compiler/rustc_incremental/src/persist/dirty_clean.rs
    pub mod rustc_incremental_src_persist_dirty_clean {
        include!("processed_rustc_incremental_src_persist_dirty_clean.rs");
    }
}

// 17: rustc_error_codes (1 files)
pub mod included_rustc_error_codes {
    // Source: ../rust/compiler/rustc_error_codes/src/lib.rs
    pub mod rustc_error_codes_rustc_error_codes_src_lib {
        include!("processed_rustc_error_codes_rustc_error_codes_src_lib.rs");
    }
}

// 18: rustc (2 files)
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

// 19: rustc_ast (22 files)
pub mod included_rustc_ast {
    // Source: ../rust/compiler/rustc_ast/src/util/case.rs
    pub mod rustc_ast_src_util_case {
        include!("processed_rustc_ast_src_util_case.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/parser.rs
    pub mod rustc_ast_src_util_parser {
        include!("processed_rustc_ast_src_util_parser.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/literal.rs
    pub mod rustc_ast_src_util_literal {
        include!("processed_rustc_ast_src_util_literal.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/unicode.rs
    pub mod rustc_ast_src_util_unicode {
        include!("processed_rustc_ast_src_util_unicode.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/token.rs
    pub mod rustc_ast_rustc_ast_src_token {
        include!("processed_rustc_ast_rustc_ast_src_token.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/ast_traits.rs
    pub mod rustc_ast_rustc_ast_src_ast_traits {
        include!("processed_rustc_ast_rustc_ast_src_ast_traits.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/autodiff_attrs.rs
    pub mod rustc_ast_src_expand_autodiff_attrs {
        include!("processed_rustc_ast_src_expand_autodiff_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/allocator.rs
    pub mod rustc_ast_src_expand_allocator {
        include!("processed_rustc_ast_src_expand_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/lib.rs
    pub mod rustc_ast_rustc_ast_src_lib {
        include!("processed_rustc_ast_rustc_ast_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/ast.rs
    pub mod rustc_ast_rustc_ast_src_ast {
        include!("processed_rustc_ast_rustc_ast_src_ast.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/node_id.rs
    pub mod rustc_ast_rustc_ast_src_node_id {
        include!("processed_rustc_ast_rustc_ast_src_node_id.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/visit.rs
    pub mod rustc_ast_rustc_ast_src_visit {
        include!("processed_rustc_ast_rustc_ast_src_visit.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/classify.rs
    pub mod rustc_ast_src_util_classify {
        include!("processed_rustc_ast_src_util_classify.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/entry.rs
    pub mod rustc_ast_rustc_ast_src_entry {
        include!("processed_rustc_ast_rustc_ast_src_entry.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/util/comments/tests.rs
    pub mod rustc_ast_util_comments_tests {
        include!("processed_rustc_ast_util_comments_tests.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/tokenstream.rs
    pub mod rustc_ast_rustc_ast_src_tokenstream {
        include!("processed_rustc_ast_rustc_ast_src_tokenstream.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/mod.rs
    pub mod rustc_ast_src_expand_mod {
        include!("processed_rustc_ast_src_expand_mod.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/format.rs
    pub mod rustc_ast_rustc_ast_src_format {
        include!("processed_rustc_ast_rustc_ast_src_format.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/attr/mod.rs
    pub mod rustc_ast_src_attr_mod {
        include!("processed_rustc_ast_src_attr_mod.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/mut_visit.rs
    pub mod rustc_ast_rustc_ast_src_mut_visit {
        include!("processed_rustc_ast_rustc_ast_src_mut_visit.rs");
    }
    // Source: ../rust/compiler/rustc_ast/src/expand/typetree.rs
    pub mod rustc_ast_src_expand_typetree {
        include!("processed_rustc_ast_src_expand_typetree.rs");
    }
}

// 20: rustc_query_impl (3 files)
pub mod included_rustc_query_impl {
    // Source: ../rust/compiler/rustc_query_impl/src/plumbing.rs
    pub mod rustc_query_impl_rustc_query_impl_src_plumbing {
        include!("processed_rustc_query_impl_rustc_query_impl_src_plumbing.rs");
    }
    // Source: ../rust/compiler/rustc_query_impl/src/profiling_support.rs
    pub mod rustc_query_impl_rustc_query_impl_src_profiling_support {
        include!("processed_rustc_query_impl_rustc_query_impl_src_profiling_support.rs");
    }
    // Source: ../rust/compiler/rustc_query_impl/src/lib.rs
    pub mod rustc_query_impl_rustc_query_impl_src_lib {
        include!("processed_rustc_query_impl_rustc_query_impl_src_lib.rs");
    }
}

// 21: rustc_llvm (2 files)
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

// 22: rustc_thread_pool (28 files)
pub mod included_rustc_thread_pool {
    // Source: ../rust/compiler/rustc_thread_pool/src/unwind.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_unwind {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_unwind.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/job.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_job {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_job.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/simple_panic.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_simple_panic {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_simple_panic.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/sleep/mod.rs
    pub mod rustc_thread_pool_src_sleep_mod {
        include!("processed_rustc_thread_pool_src_sleep_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/worker_local.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_worker_local {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/private.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_private {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_private.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/init_zero_threads.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_init_zero_threads {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_init_zero_threads.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/tlv.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_tlv {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_tlv.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/spawn/tests.rs
    pub mod rustc_thread_pool_src_spawn_tests {
        include!("processed_rustc_thread_pool_src_spawn_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/sleep/counters.rs
    pub mod rustc_thread_pool_src_sleep_counters {
        include!("processed_rustc_thread_pool_src_sleep_counters.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/compile_fail/rc_return.rs
    pub mod rustc_thread_pool_src_compile_fail_rc_return {
        include!("processed_rustc_thread_pool_src_compile_fail_rc_return.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/stack_overflow_crash.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_stack_overflow_crash {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_stack_overflow_crash.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/thread_pool/mod.rs
    pub mod rustc_thread_pool_src_thread_pool_mod {
        include!("processed_rustc_thread_pool_src_thread_pool_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/latch.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_latch {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_latch.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/scoped_threadpool.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_scoped_threadpool {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_scoped_threadpool.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/scope_join.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_scope_join {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_scope_join.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/registry.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_registry {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_registry.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/lib.rs
    pub mod rustc_thread_pool_rustc_thread_pool_src_lib {
        include!("processed_rustc_thread_pool_rustc_thread_pool_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/tests/double_init_fail.rs
    pub mod rustc_thread_pool_rustc_thread_pool_tests_double_init_fail {
        include!("processed_rustc_thread_pool_rustc_thread_pool_tests_double_init_fail.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/join/tests.rs
    pub mod rustc_thread_pool_src_join_tests {
        include!("processed_rustc_thread_pool_src_join_tests.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/broadcast/mod.rs
    pub mod rustc_thread_pool_src_broadcast_mod {
        include!("processed_rustc_thread_pool_src_broadcast_mod.rs");
    }
    // Source: ../rust/compiler/rustc_thread_pool/src/scope/tests.rs
    pub mod rustc_thread_pool_src_scope_tests {
        include!("processed_rustc_thread_pool_src_scope_tests.rs");
    }
}

// 23: rustc_ast_ir (2 files)
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

// 24: rustc_passes (20 files)
pub mod included_rustc_passes {
    // Source: ../rust/compiler/rustc_passes/src/reachable.rs
    pub mod rustc_passes_rustc_passes_src_reachable {
        include!("processed_rustc_passes_rustc_passes_src_reachable.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/hir_id_validator.rs
    pub mod rustc_passes_rustc_passes_src_hir_id_validator {
        include!("processed_rustc_passes_rustc_passes_src_hir_id_validator.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/stability.rs
    pub mod rustc_passes_rustc_passes_src_stability {
        include!("processed_rustc_passes_rustc_passes_src_stability.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/entry.rs
    pub mod rustc_passes_rustc_passes_src_entry {
        include!("processed_rustc_passes_rustc_passes_src_entry.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/lib_features.rs
    pub mod rustc_passes_rustc_passes_src_lib_features {
        include!("processed_rustc_passes_rustc_passes_src_lib_features.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/lang_items.rs
    pub mod rustc_passes_rustc_passes_src_lang_items {
        include!("processed_rustc_passes_rustc_passes_src_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/check_attr.rs
    pub mod rustc_passes_rustc_passes_src_check_attr {
        include!("processed_rustc_passes_rustc_passes_src_check_attr.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/weak_lang_items.rs
    pub mod rustc_passes_rustc_passes_src_weak_lang_items {
        include!("processed_rustc_passes_rustc_passes_src_weak_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/check_export.rs
    pub mod rustc_passes_rustc_passes_src_check_export {
        include!("processed_rustc_passes_rustc_passes_src_check_export.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/input_stats.rs
    pub mod rustc_passes_rustc_passes_src_input_stats {
        include!("processed_rustc_passes_rustc_passes_src_input_stats.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/liveness/rwu_table.rs
    pub mod rustc_passes_src_liveness_rwu_table {
        include!("processed_rustc_passes_src_liveness_rwu_table.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/lib.rs
    pub mod rustc_passes_rustc_passes_src_lib {
        include!("processed_rustc_passes_rustc_passes_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/dead.rs
    pub mod rustc_passes_rustc_passes_src_dead {
        include!("processed_rustc_passes_rustc_passes_src_dead.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/debugger_visualizer.rs
    pub mod rustc_passes_rustc_passes_src_debugger_visualizer {
        include!("processed_rustc_passes_rustc_passes_src_debugger_visualizer.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/errors.rs
    pub mod rustc_passes_rustc_passes_src_errors {
        include!("processed_rustc_passes_rustc_passes_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/liveness.rs
    pub mod rustc_passes_rustc_passes_src_liveness {
        include!("processed_rustc_passes_rustc_passes_src_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/upvars.rs
    pub mod rustc_passes_rustc_passes_src_upvars {
        include!("processed_rustc_passes_rustc_passes_src_upvars.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/abi_test.rs
    pub mod rustc_passes_rustc_passes_src_abi_test {
        include!("processed_rustc_passes_rustc_passes_src_abi_test.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/layout_test.rs
    pub mod rustc_passes_rustc_passes_src_layout_test {
        include!("processed_rustc_passes_rustc_passes_src_layout_test.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/diagnostic_items.rs
    pub mod rustc_passes_rustc_passes_src_diagnostic_items {
        include!("processed_rustc_passes_rustc_passes_src_diagnostic_items.rs");
    }
}

// 25: rustc_infer (39 files)
pub mod included_rustc_infer {
    // Source: ../rust/compiler/rustc_infer/src/infer/snapshot/mod.rs
    pub mod rustc_infer_infer_snapshot_mod {
        include!("processed_rustc_infer_infer_snapshot_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/freshen.rs
    pub mod rustc_infer_src_infer_freshen {
        include!("processed_rustc_infer_src_infer_freshen.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/projection.rs
    pub mod rustc_infer_src_infer_projection {
        include!("processed_rustc_infer_src_infer_projection.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/free_regions.rs
    pub mod rustc_infer_src_infer_free_regions {
        include!("processed_rustc_infer_src_infer_free_regions.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/snapshot/fudge.rs
    pub mod rustc_infer_infer_snapshot_fudge {
        include!("processed_rustc_infer_infer_snapshot_fudge.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/test_type_match.rs
    pub mod rustc_infer_infer_outlives_test_type_match {
        include!("processed_rustc_infer_infer_outlives_test_type_match.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/lib.rs
    pub mod rustc_infer_rustc_infer_src_lib {
        include!("processed_rustc_infer_rustc_infer_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/canonicalizer.rs
    pub mod rustc_infer_infer_canonical_canonicalizer {
        include!("processed_rustc_infer_infer_canonical_canonicalizer.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs
    pub mod rustc_infer_infer_lexical_region_resolve_mod {
        include!("processed_rustc_infer_infer_lexical_region_resolve_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/snapshot/undo_log.rs
    pub mod rustc_infer_infer_snapshot_undo_log {
        include!("processed_rustc_infer_infer_snapshot_undo_log.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/region_constraints/leak_check.rs
    pub mod rustc_infer_infer_region_constraints_leak_check {
        include!("processed_rustc_infer_infer_region_constraints_leak_check.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/for_liveness.rs
    pub mod rustc_infer_infer_outlives_for_liveness {
        include!("processed_rustc_infer_infer_outlives_for_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/verify.rs
    pub mod rustc_infer_infer_outlives_verify {
        include!("processed_rustc_infer_infer_outlives_verify.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/structural_impls.rs
    pub mod rustc_infer_src_traits_structural_impls {
        include!("processed_rustc_infer_src_traits_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/mod.rs
    pub mod rustc_infer_src_infer_mod {
        include!("processed_rustc_infer_src_infer_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/type_variable.rs
    pub mod rustc_infer_src_infer_type_variable {
        include!("processed_rustc_infer_src_infer_type_variable.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/util.rs
    pub mod rustc_infer_src_traits_util {
        include!("processed_rustc_infer_src_traits_util.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/at.rs
    pub mod rustc_infer_src_infer_at {
        include!("processed_rustc_infer_src_infer_at.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/instantiate.rs
    pub mod rustc_infer_infer_canonical_instantiate {
        include!("processed_rustc_infer_infer_canonical_instantiate.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/lattice.rs
    pub mod rustc_infer_infer_relate_lattice {
        include!("processed_rustc_infer_infer_relate_lattice.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/mod.rs
    pub mod rustc_infer_infer_relate_mod {
        include!("processed_rustc_infer_infer_relate_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/generalize.rs
    pub mod rustc_infer_infer_relate_generalize {
        include!("processed_rustc_infer_infer_relate_generalize.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/unify_key.rs
    pub mod rustc_infer_src_infer_unify_key {
        include!("processed_rustc_infer_src_infer_unify_key.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/opaque_types/table.rs
    pub mod rustc_infer_infer_opaque_types_table {
        include!("processed_rustc_infer_infer_opaque_types_table.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/engine.rs
    pub mod rustc_infer_src_traits_engine {
        include!("processed_rustc_infer_src_traits_engine.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/obligations.rs
    pub mod rustc_infer_infer_outlives_obligations {
        include!("processed_rustc_infer_infer_outlives_obligations.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/region_constraints/mod.rs
    pub mod rustc_infer_infer_region_constraints_mod {
        include!("processed_rustc_infer_infer_region_constraints_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/type_relating.rs
    pub mod rustc_infer_infer_relate_type_relating {
        include!("processed_rustc_infer_infer_relate_type_relating.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/env.rs
    pub mod rustc_infer_infer_outlives_env {
        include!("processed_rustc_infer_infer_outlives_env.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/relate/higher_ranked.rs
    pub mod rustc_infer_infer_relate_higher_ranked {
        include!("processed_rustc_infer_infer_relate_higher_ranked.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/opaque_types/mod.rs
    pub mod rustc_infer_infer_opaque_types_mod {
        include!("processed_rustc_infer_infer_opaque_types_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/query_response.rs
    pub mod rustc_infer_infer_canonical_query_response {
        include!("processed_rustc_infer_infer_canonical_query_response.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/context.rs
    pub mod rustc_infer_src_infer_context {
        include!("processed_rustc_infer_src_infer_context.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/errors.rs
    pub mod rustc_infer_rustc_infer_src_errors {
        include!("processed_rustc_infer_rustc_infer_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/mod.rs
    pub mod rustc_infer_src_traits_mod {
        include!("processed_rustc_infer_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/outlives/mod.rs
    pub mod rustc_infer_infer_outlives_mod {
        include!("processed_rustc_infer_infer_outlives_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/resolve.rs
    pub mod rustc_infer_src_infer_resolve {
        include!("processed_rustc_infer_src_infer_resolve.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/infer/canonical/mod.rs
    pub mod rustc_infer_infer_canonical_mod {
        include!("processed_rustc_infer_infer_canonical_mod.rs");
    }
    // Source: ../rust/compiler/rustc_infer/src/traits/project.rs
    pub mod rustc_infer_src_traits_project {
        include!("processed_rustc_infer_src_traits_project.rs");
    }
}

// 26: rustc_session (18 files)
pub mod included_rustc_session {
    // Source: ../rust/compiler/rustc_session/src/config/cfg.rs
    pub mod rustc_session_src_config_cfg {
        include!("processed_rustc_session_src_config_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/config/native_libs/tests.rs
    pub mod rustc_session_config_native_libs_tests {
        include!("processed_rustc_session_config_native_libs_tests.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/parse.rs
    pub mod rustc_session_rustc_session_src_parse {
        include!("processed_rustc_session_rustc_session_src_parse.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/search_paths.rs
    pub mod rustc_session_rustc_session_src_search_paths {
        include!("processed_rustc_session_rustc_session_src_search_paths.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/config.rs
    pub mod rustc_session_rustc_session_src_config {
        include!("processed_rustc_session_rustc_session_src_config.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/errors.rs
    pub mod rustc_session_rustc_session_src_errors {
        include!("processed_rustc_session_rustc_session_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/utils.rs
    pub mod rustc_session_rustc_session_src_utils {
        include!("processed_rustc_session_rustc_session_src_utils.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/lib.rs
    pub mod rustc_session_rustc_session_src_lib {
        include!("processed_rustc_session_rustc_session_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/filesearch.rs
    pub mod rustc_session_rustc_session_src_filesearch {
        include!("processed_rustc_session_rustc_session_src_filesearch.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/config/externs/tests.rs
    pub mod rustc_session_config_externs_tests {
        include!("processed_rustc_session_config_externs_tests.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/cstore.rs
    pub mod rustc_session_rustc_session_src_cstore {
        include!("processed_rustc_session_rustc_session_src_cstore.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/output.rs
    pub mod rustc_session_rustc_session_src_output {
        include!("processed_rustc_session_rustc_session_src_output.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/session.rs
    pub mod rustc_session_rustc_session_src_session {
        include!("processed_rustc_session_rustc_session_src_session.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/config/sigpipe.rs
    pub mod rustc_session_src_config_sigpipe {
        include!("processed_rustc_session_src_config_sigpipe.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/code_stats.rs
    pub mod rustc_session_rustc_session_src_code_stats {
        include!("processed_rustc_session_rustc_session_src_code_stats.rs");
    }
    // Source: ../rust/compiler/rustc_session/src/options.rs
    pub mod rustc_session_rustc_session_src_options {
        include!("processed_rustc_session_rustc_session_src_options.rs");
    }
}

// 27: rustc_traits (9 files)
pub mod included_rustc_traits {
    // Source: ../rust/compiler/rustc_traits/src/evaluate_obligation.rs
    pub mod rustc_traits_rustc_traits_src_evaluate_obligation {
        include!("processed_rustc_traits_rustc_traits_src_evaluate_obligation.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/type_op.rs
    pub mod rustc_traits_rustc_traits_src_type_op {
        include!("processed_rustc_traits_rustc_traits_src_type_op.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/codegen.rs
    pub mod rustc_traits_rustc_traits_src_codegen {
        include!("processed_rustc_traits_rustc_traits_src_codegen.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/lib.rs
    pub mod rustc_traits_rustc_traits_src_lib {
        include!("processed_rustc_traits_rustc_traits_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/normalize_projection_ty.rs
    pub mod rustc_traits_rustc_traits_src_normalize_projection_ty {
        include!("processed_rustc_traits_rustc_traits_src_normalize_projection_ty.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/coroutine_witnesses.rs
    pub mod rustc_traits_rustc_traits_src_coroutine_witnesses {
        include!("processed_rustc_traits_rustc_traits_src_coroutine_witnesses.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/implied_outlives_bounds.rs
    pub mod rustc_traits_rustc_traits_src_implied_outlives_bounds {
        include!("processed_rustc_traits_rustc_traits_src_implied_outlives_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/normalize_erasing_regions.rs
    pub mod rustc_traits_rustc_traits_src_normalize_erasing_regions {
        include!("processed_rustc_traits_rustc_traits_src_normalize_erasing_regions.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/dropck_outlives.rs
    pub mod rustc_traits_rustc_traits_src_dropck_outlives {
        include!("processed_rustc_traits_rustc_traits_src_dropck_outlives.rs");
    }
}

// 28: rustc_mir_transform (93 files)
pub mod included_rustc_mir_transform {
    // Source: ../rust/compiler/rustc_mir_transform/src/deduce_param_attrs.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_deduce_param_attrs {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_deduce_param_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coroutine/drop.rs
    pub mod rustc_mir_transform_src_coroutine_drop {
        include!("processed_rustc_mir_transform_src_coroutine_drop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/instsimplify.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_instsimplify {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_instsimplify.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/dead_store_elimination.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_dead_store_elimination {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_dead_store_elimination.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/add_moves_for_packed_drops.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_add_moves_for_packed_drops {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_add_moves_for_packed_drops.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/lib.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lib {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/unreachable_enum_branching.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_unreachable_enum_branching {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_unreachable_enum_branching.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/prettify.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_prettify {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_prettify.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/add_subtyping_projections.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_add_subtyping_projections {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_add_subtyping_projections.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/lower_slice_len.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lower_slice_len {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lower_slice_len.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/expansion.rs
    pub mod rustc_mir_transform_src_coverage_expansion {
        include!("processed_rustc_mir_transform_src_coverage_expansion.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/post_drop_elaboration.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_post_drop_elaboration {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_post_drop_elaboration.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/known_panics_lint.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_known_panics_lint {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_known_panics_lint.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/dest_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_dest_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_dest_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/tests.rs
    pub mod rustc_mir_transform_src_coverage_tests {
        include!("processed_rustc_mir_transform_src_coverage_tests.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/counters/balanced_flow.rs
    pub mod rustc_mir_transform_coverage_counters_balanced_flow {
        include!("processed_rustc_mir_transform_coverage_counters_balanced_flow.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/lint.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lint {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lint.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/add_call_guards.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_add_call_guards {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_add_call_guards.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_alignment.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_alignment {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_alignment.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/ssa.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_ssa {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_ssa.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/inline.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_inline {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_inline.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/unreachable_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_unreachable_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_unreachable_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/strip_debuginfo.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_strip_debuginfo {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_strip_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/pass_manager.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_pass_manager {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_pass_manager.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/query.rs
    pub mod rustc_mir_transform_src_coverage_query {
        include!("processed_rustc_mir_transform_src_coverage_query.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_noop_landing_pads.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_noop_landing_pads {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_noop_landing_pads.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/impossible_predicates.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_impossible_predicates {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_impossible_predicates.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/lint_tail_expr_drop_order.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lint_tail_expr_drop_order {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lint_tail_expr_drop_order.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/deref_separator.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_deref_separator {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_deref_separator.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/function_item_references.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_function_item_references {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_function_item_references.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/validate.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_validate {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_validate.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/sanity_check.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_sanity_check {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_sanity_check.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/add_retag.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_add_retag {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_add_retag.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/nrvo.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_nrvo {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_nrvo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/abort_unwinding_calls.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_abort_unwinding_calls {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_abort_unwinding_calls.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/simplify_branches.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_simplify_branches {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_simplify_branches.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/gvn.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_gvn {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_gvn.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coroutine/by_move_body.rs
    pub mod rustc_mir_transform_src_coroutine_by_move_body {
        include!("processed_rustc_mir_transform_src_coroutine_by_move_body.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_place_mention.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_place_mention {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_place_mention.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/mentioned_items.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_mentioned_items {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_mentioned_items.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_call_recursion.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_call_recursion {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_call_recursion.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_packed_ref.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_packed_ref {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_packed_ref.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/match_branches.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_match_branches {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_match_branches.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/inline/cycle.rs
    pub mod rustc_mir_transform_src_inline_cycle {
        include!("processed_rustc_mir_transform_src_inline_cycle.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/cross_crate_inline.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_cross_crate_inline {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_cross_crate_inline.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/simplify_comparison_integral.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_simplify_comparison_integral {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_simplify_comparison_integral.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_zsts.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_zsts {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_zsts.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/simplify.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_simplify {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_simplify.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/errors.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_errors {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/elaborate_box_derefs.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_elaborate_box_derefs {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_elaborate_box_derefs.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/promote_consts.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_promote_consts {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_promote_consts.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/cleanup_post_borrowck.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_cleanup_post_borrowck {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_cleanup_post_borrowck.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/lower_intrinsics.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_lower_intrinsics {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_lower_intrinsics.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/elaborate_drop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_elaborate_drop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_elaborate_drop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/dump_mir.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_dump_mir {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_dump_mir.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/early_otherwise_branch.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_early_otherwise_branch {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_early_otherwise_branch.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_inline_always_target_features.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_inline_always_target_features {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_inline_always_target_features.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/shim/async_destructor_ctor.rs
    pub mod rustc_mir_transform_src_shim_async_destructor_ctor {
        include!("processed_rustc_mir_transform_src_shim_async_destructor_ctor.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/unexpand.rs
    pub mod rustc_mir_transform_src_coverage_unexpand {
        include!("processed_rustc_mir_transform_src_coverage_unexpand.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_pointers.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_pointers {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_pointers.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_storage_markers.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_storage_markers {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_storage_markers.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/hir_info.rs
    pub mod rustc_mir_transform_src_coverage_hir_info {
        include!("processed_rustc_mir_transform_src_coverage_hir_info.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/ffi_unwind_calls.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_ffi_unwind_calls {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_ffi_unwind_calls.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_null.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_null {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_null.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/ctfe_limit.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_ctfe_limit {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_ctfe_limit.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/single_use_consts.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_single_use_consts {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_single_use_consts.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/patch.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_patch {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_patch.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/jump_threading.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_jump_threading {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_jump_threading.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/spans/from_mir.rs
    pub mod rustc_mir_transform_coverage_spans_from_mir {
        include!("processed_rustc_mir_transform_coverage_spans_from_mir.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_unneeded_drops.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_unneeded_drops {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_unneeded_drops.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/shim.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_shim {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_shim.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/ref_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_ref_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_ref_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/post_analysis_normalize.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_post_analysis_normalize {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_post_analysis_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/mappings.rs
    pub mod rustc_mir_transform_src_coverage_mappings {
        include!("processed_rustc_mir_transform_src_coverage_mappings.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_const_item_mutation.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_const_item_mutation {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_const_item_mutation.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/large_enums.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_large_enums {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_large_enums.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/remove_uninit_drops.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_remove_uninit_drops {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_remove_uninit_drops.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_inline.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_inline {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_inline.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/sroa.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_sroa {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_sroa.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/check_enums.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_check_enums {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_check_enums.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/graph.rs
    pub mod rustc_mir_transform_src_coverage_graph {
        include!("processed_rustc_mir_transform_src_coverage_graph.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/cost_checker.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_cost_checker {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_cost_checker.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/multiple_return_terminators.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_multiple_return_terminators {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_multiple_return_terminators.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/copy_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_copy_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_copy_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/dataflow_const_prop.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_dataflow_const_prop {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_dataflow_const_prop.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/counters/node_flow/tests.rs
    pub mod rustc_mir_transform_counters_node_flow_tests {
        include!("processed_rustc_mir_transform_counters_node_flow_tests.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/counters.rs
    pub mod rustc_mir_transform_src_coverage_counters {
        include!("processed_rustc_mir_transform_src_coverage_counters.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coroutine.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_coroutine {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_coroutine.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/required_consts.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_required_consts {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_required_consts.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/elaborate_drops.rs
    pub mod rustc_mir_transform_rustc_mir_transform_src_elaborate_drops {
        include!("processed_rustc_mir_transform_rustc_mir_transform_src_elaborate_drops.rs");
    }
    // Source: ../rust/compiler/rustc_mir_transform/src/coverage/spans.rs
    pub mod rustc_mir_transform_src_coverage_spans {
        include!("processed_rustc_mir_transform_src_coverage_spans.rs");
    }
}

// 29: rustc_fluent_macro (2 files)
pub mod included_rustc_fluent_macro {
    // Source: ../rust/compiler/rustc_fluent_macro/src/lib.rs
    pub mod rustc_fluent_macro_rustc_fluent_macro_src_lib {
        include!("processed_rustc_fluent_macro_rustc_fluent_macro_src_lib.rs");
    }
}

// 30: rustc_hir_pretty (1 files)
pub mod included_rustc_hir_pretty {
    // Source: ../rust/compiler/rustc_hir_pretty/src/lib.rs
    pub mod rustc_hir_pretty_rustc_hir_pretty_src_lib {
        include!("processed_rustc_hir_pretty_rustc_hir_pretty_src_lib.rs");
    }
}

// 31: rustc_mir_build (36 files)
pub mod included_rustc_mir_build {
    // Source: ../rust/compiler/rustc_mir_build/src/thir/pattern/const_to_pat.rs
    pub mod rustc_mir_build_thir_pattern_const_to_pat {
        include!("processed_rustc_mir_build_thir_pattern_const_to_pat.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/match_pair.rs
    pub mod rustc_mir_build_builder_matches_match_pair {
        include!("processed_rustc_mir_build_builder_matches_match_pair.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/coverageinfo.rs
    pub mod rustc_mir_build_src_builder_coverageinfo {
        include!("processed_rustc_mir_build_src_builder_coverageinfo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/mod.rs
    pub mod rustc_mir_build_builder_matches_mod {
        include!("processed_rustc_mir_build_builder_matches_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/custom/mod.rs
    pub mod rustc_mir_build_builder_custom_mod {
        include!("processed_rustc_mir_build_builder_custom_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/stmt.rs
    pub mod rustc_mir_build_builder_expr_stmt {
        include!("processed_rustc_mir_build_builder_expr_stmt.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/category.rs
    pub mod rustc_mir_build_builder_expr_category {
        include!("processed_rustc_mir_build_builder_expr_category.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_constant.rs
    pub mod rustc_mir_build_builder_expr_as_constant {
        include!("processed_rustc_mir_build_builder_expr_as_constant.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/check_tail_calls.rs
    pub mod rustc_mir_build_rustc_mir_build_src_check_tail_calls {
        include!("processed_rustc_mir_build_rustc_mir_build_src_check_tail_calls.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/cx/expr.rs
    pub mod rustc_mir_build_thir_cx_expr {
        include!("processed_rustc_mir_build_thir_cx_expr.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/into.rs
    pub mod rustc_mir_build_builder_expr_into {
        include!("processed_rustc_mir_build_builder_expr_into.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/user_ty.rs
    pub mod rustc_mir_build_builder_matches_user_ty {
        include!("processed_rustc_mir_build_builder_matches_user_ty.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/pattern/migration.rs
    pub mod rustc_mir_build_thir_pattern_migration {
        include!("processed_rustc_mir_build_thir_pattern_migration.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_place.rs
    pub mod rustc_mir_build_builder_expr_as_place {
        include!("processed_rustc_mir_build_builder_expr_as_place.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_operand.rs
    pub mod rustc_mir_build_builder_expr_as_operand {
        include!("processed_rustc_mir_build_builder_expr_as_operand.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/util.rs
    pub mod rustc_mir_build_src_thir_util {
        include!("processed_rustc_mir_build_src_thir_util.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/scope.rs
    pub mod rustc_mir_build_src_builder_scope {
        include!("processed_rustc_mir_build_src_builder_scope.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/test.rs
    pub mod rustc_mir_build_builder_matches_test {
        include!("processed_rustc_mir_build_builder_matches_test.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/constant.rs
    pub mod rustc_mir_build_src_thir_constant {
        include!("processed_rustc_mir_build_src_thir_constant.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/custom/parse.rs
    pub mod rustc_mir_build_builder_custom_parse {
        include!("processed_rustc_mir_build_builder_custom_parse.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/check_unsafety.rs
    pub mod rustc_mir_build_rustc_mir_build_src_check_unsafety {
        include!("processed_rustc_mir_build_rustc_mir_build_src_check_unsafety.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/block.rs
    pub mod rustc_mir_build_src_builder_block {
        include!("processed_rustc_mir_build_src_builder_block.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/errors.rs
    pub mod rustc_mir_build_rustc_mir_build_src_errors {
        include!("processed_rustc_mir_build_rustc_mir_build_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_temp.rs
    pub mod rustc_mir_build_builder_expr_as_temp {
        include!("processed_rustc_mir_build_builder_expr_as_temp.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/pattern/mod.rs
    pub mod rustc_mir_build_thir_pattern_mod {
        include!("processed_rustc_mir_build_thir_pattern_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/pattern/check_match.rs
    pub mod rustc_mir_build_thir_pattern_check_match {
        include!("processed_rustc_mir_build_thir_pattern_check_match.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/cx/mod.rs
    pub mod rustc_mir_build_thir_cx_mod {
        include!("processed_rustc_mir_build_thir_cx_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/matches/util.rs
    pub mod rustc_mir_build_builder_matches_util {
        include!("processed_rustc_mir_build_builder_matches_util.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/custom/parse/instruction.rs
    pub mod rustc_mir_build_custom_parse_instruction {
        include!("processed_rustc_mir_build_custom_parse_instruction.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/expr/as_rvalue.rs
    pub mod rustc_mir_build_builder_expr_as_rvalue {
        include!("processed_rustc_mir_build_builder_expr_as_rvalue.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/print.rs
    pub mod rustc_mir_build_src_thir_print {
        include!("processed_rustc_mir_build_src_thir_print.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/cfg.rs
    pub mod rustc_mir_build_src_builder_cfg {
        include!("processed_rustc_mir_build_src_builder_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/misc.rs
    pub mod rustc_mir_build_src_builder_misc {
        include!("processed_rustc_mir_build_src_builder_misc.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/lib.rs
    pub mod rustc_mir_build_rustc_mir_build_src_lib {
        include!("processed_rustc_mir_build_rustc_mir_build_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/thir/cx/block.rs
    pub mod rustc_mir_build_thir_cx_block {
        include!("processed_rustc_mir_build_thir_cx_block.rs");
    }
    // Source: ../rust/compiler/rustc_mir_build/src/builder/mod.rs
    pub mod rustc_mir_build_src_builder_mod {
        include!("processed_rustc_mir_build_src_builder_mod.rs");
    }
}

// 32: rustc_mir_dataflow (24 files)
pub mod included_rustc_mir_dataflow {
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/tests.rs
    pub mod rustc_mir_dataflow_src_framework_tests {
        include!("processed_rustc_mir_dataflow_src_framework_tests.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/direction.rs
    pub mod rustc_mir_dataflow_src_framework_direction {
        include!("processed_rustc_mir_dataflow_src_framework_direction.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/drop_flag_effects.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_drop_flag_effects {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_drop_flag_effects.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/move_paths/mod.rs
    pub mod rustc_mir_dataflow_src_move_paths_mod {
        include!("processed_rustc_mir_dataflow_src_move_paths_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/errors.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_errors {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/lattice.rs
    pub mod rustc_mir_dataflow_src_framework_lattice {
        include!("processed_rustc_mir_dataflow_src_framework_lattice.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/initialized.rs
    pub mod rustc_mir_dataflow_src_impls_initialized {
        include!("processed_rustc_mir_dataflow_src_impls_initialized.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/borrowed_locals.rs
    pub mod rustc_mir_dataflow_src_impls_borrowed_locals {
        include!("processed_rustc_mir_dataflow_src_impls_borrowed_locals.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/un_derefer.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_un_derefer {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_un_derefer.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/lib.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_lib {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/graphviz.rs
    pub mod rustc_mir_dataflow_src_framework_graphviz {
        include!("processed_rustc_mir_dataflow_src_framework_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/visitor.rs
    pub mod rustc_mir_dataflow_src_framework_visitor {
        include!("processed_rustc_mir_dataflow_src_framework_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/move_paths/builder.rs
    pub mod rustc_mir_dataflow_src_move_paths_builder {
        include!("processed_rustc_mir_dataflow_src_move_paths_builder.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/debuginfo.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_debuginfo {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/mod.rs
    pub mod rustc_mir_dataflow_src_impls_mod {
        include!("processed_rustc_mir_dataflow_src_impls_mod.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/liveness.rs
    pub mod rustc_mir_dataflow_src_impls_liveness {
        include!("processed_rustc_mir_dataflow_src_impls_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/value_analysis.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_value_analysis {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_value_analysis.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/results.rs
    pub mod rustc_mir_dataflow_src_framework_results {
        include!("processed_rustc_mir_dataflow_src_framework_results.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/rustc_peek.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_rustc_peek {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_rustc_peek.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/impls/storage_liveness.rs
    pub mod rustc_mir_dataflow_src_impls_storage_liveness {
        include!("processed_rustc_mir_dataflow_src_impls_storage_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/points.rs
    pub mod rustc_mir_dataflow_rustc_mir_dataflow_src_points {
        include!("processed_rustc_mir_dataflow_rustc_mir_dataflow_src_points.rs");
    }
    // Source: ../rust/compiler/rustc_mir_dataflow/src/framework/fmt.rs
    pub mod rustc_mir_dataflow_src_framework_fmt {
        include!("processed_rustc_mir_dataflow_src_framework_fmt.rs");
    }
}

// 33: rustc_parse_format (2 files)
pub mod included_rustc_parse_format {
    // Source: ../rust/compiler/rustc_parse_format/src/tests.rs
    pub mod rustc_parse_format_rustc_parse_format_src_tests {
        include!("processed_rustc_parse_format_rustc_parse_format_src_tests.rs");
    }
}

// 34: rustc_middle (113 files)
pub mod included_rustc_middle {
    // Source: ../rust/compiler/rustc_middle/src/ty/list.rs
    pub mod rustc_middle_src_ty_list {
        include!("processed_rustc_middle_src_ty_list.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/place.rs
    pub mod rustc_middle_src_hir_place {
        include!("processed_rustc_middle_src_hir_place.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/mono.rs
    pub mod rustc_middle_src_mir_mono {
        include!("processed_rustc_middle_src_mir_mono.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/elaborate_impl.rs
    pub mod rustc_middle_src_ty_elaborate_impl {
        include!("processed_rustc_middle_src_ty_elaborate_impl.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/lint.rs
    pub mod rustc_middle_rustc_middle_src_lint {
        include!("processed_rustc_middle_rustc_middle_src_lint.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/keys.rs
    pub mod rustc_middle_src_query_keys {
        include!("processed_rustc_middle_src_query_keys.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/pattern.rs
    pub mod rustc_middle_src_ty_pattern {
        include!("processed_rustc_middle_src_ty_pattern.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/abstract_const.rs
    pub mod rustc_middle_src_ty_abstract_const {
        include!("processed_rustc_middle_src_ty_abstract_const.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/rvalue_scopes.rs
    pub mod rustc_middle_src_ty_rvalue_scopes {
        include!("processed_rustc_middle_src_ty_rvalue_scopes.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/region.rs
    pub mod rustc_middle_src_middle_region {
        include!("processed_rustc_middle_src_middle_region.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/arena.rs
    pub mod rustc_middle_rustc_middle_src_arena {
        include!("processed_rustc_middle_rustc_middle_src_arena.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/mod.rs
    pub mod rustc_middle_src_query_mod {
        include!("processed_rustc_middle_src_query_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/instance.rs
    pub mod rustc_middle_src_ty_instance {
        include!("processed_rustc_middle_src_ty_instance.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/predicate.rs
    pub mod rustc_middle_src_ty_predicate {
        include!("processed_rustc_middle_src_ty_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/mod.rs
    pub mod rustc_middle_src_ty_mod {
        include!("processed_rustc_middle_src_ty_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/sty.rs
    pub mod rustc_middle_src_ty_sty {
        include!("processed_rustc_middle_src_ty_sty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/loops.rs
    pub mod rustc_middle_src_mir_loops {
        include!("processed_rustc_middle_src_mir_loops.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/assoc.rs
    pub mod rustc_middle_src_ty_assoc {
        include!("processed_rustc_middle_src_ty_assoc.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs
    pub mod rustc_middle_src_ty_normalize_erasing_regions {
        include!("processed_rustc_middle_src_ty_normalize_erasing_regions.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/impls_ty.rs
    pub mod rustc_middle_src_ty_impls_ty {
        include!("processed_rustc_middle_src_ty_impls_ty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/visit.rs
    pub mod rustc_middle_src_mir_visit {
        include!("processed_rustc_middle_src_mir_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/query.rs
    pub mod rustc_middle_src_mir_query {
        include!("processed_rustc_middle_src_mir_query.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/plumbing.rs
    pub mod rustc_middle_src_query_plumbing {
        include!("processed_rustc_middle_src_query_plumbing.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/opaque_types.rs
    pub mod rustc_middle_src_ty_opaque_types {
        include!("processed_rustc_middle_src_ty_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/select.rs
    pub mod rustc_middle_src_traits_select {
        include!("processed_rustc_middle_src_traits_select.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/adjustment.rs
    pub mod rustc_middle_src_ty_adjustment {
        include!("processed_rustc_middle_src_ty_adjustment.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/queries.rs
    pub mod rustc_middle_mir_interpret_queries {
        include!("processed_rustc_middle_mir_interpret_queries.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/util/mod.rs
    pub mod rustc_middle_src_util_mod {
        include!("processed_rustc_middle_src_util_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/print/pretty.rs
    pub mod rustc_middle_ty_print_pretty {
        include!("processed_rustc_middle_ty_print_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/specialization_graph.rs
    pub mod rustc_middle_src_traits_specialization_graph {
        include!("processed_rustc_middle_src_traits_specialization_graph.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/significant_drop_order.rs
    pub mod rustc_middle_src_ty_significant_drop_order {
        include!("processed_rustc_middle_src_ty_significant_drop_order.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/mod.rs
    pub mod rustc_middle_src_traits_mod {
        include!("processed_rustc_middle_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/stability.rs
    pub mod rustc_middle_src_middle_stability {
        include!("processed_rustc_middle_src_middle_stability.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts.rs
    pub mod rustc_middle_src_ty_consts {
        include!("processed_rustc_middle_src_ty_consts.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/generic_graph.rs
    pub mod rustc_middle_src_mir_generic_graph {
        include!("processed_rustc_middle_src_mir_generic_graph.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/init_mask/tests.rs
    pub mod rustc_middle_allocation_init_mask_tests {
        include!("processed_rustc_middle_allocation_init_mask_tests.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/erase_regions.rs
    pub mod rustc_middle_src_ty_erase_regions {
        include!("processed_rustc_middle_src_ty_erase_regions.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/metadata.rs
    pub mod rustc_middle_rustc_middle_src_metadata {
        include!("processed_rustc_middle_rustc_middle_src_metadata.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/context/tls.rs
    pub mod rustc_middle_ty_context_tls {
        include!("processed_rustc_middle_ty_context_tls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/value.rs
    pub mod rustc_middle_mir_interpret_value {
        include!("processed_rustc_middle_mir_interpret_value.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/util.rs
    pub mod rustc_middle_src_ty_util {
        include!("processed_rustc_middle_src_ty_util.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/closure.rs
    pub mod rustc_middle_src_ty_closure {
        include!("processed_rustc_middle_src_ty_closure.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/structural_impls.rs
    pub mod rustc_middle_src_ty_structural_impls {
        include!("processed_rustc_middle_src_ty_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/relate.rs
    pub mod rustc_middle_src_ty_relate {
        include!("processed_rustc_middle_src_ty_relate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/trait_def.rs
    pub mod rustc_middle_src_ty_trait_def {
        include!("processed_rustc_middle_src_ty_trait_def.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/cast.rs
    pub mod rustc_middle_src_ty_cast {
        include!("processed_rustc_middle_src_ty_cast.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/codec.rs
    pub mod rustc_middle_src_ty_codec {
        include!("processed_rustc_middle_src_ty_codec.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/kind.rs
    pub mod rustc_middle_ty_consts_kind {
        include!("processed_rustc_middle_ty_consts_kind.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/fast_reject.rs
    pub mod rustc_middle_src_ty_fast_reject {
        include!("processed_rustc_middle_src_ty_fast_reject.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/valtree.rs
    pub mod rustc_middle_ty_consts_valtree {
        include!("processed_rustc_middle_ty_consts_valtree.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/terminator.rs
    pub mod rustc_middle_src_mir_terminator {
        include!("processed_rustc_middle_src_mir_terminator.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/thir.rs
    pub mod rustc_middle_rustc_middle_src_thir {
        include!("processed_rustc_middle_rustc_middle_src_thir.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/fold.rs
    pub mod rustc_middle_src_ty_fold {
        include!("processed_rustc_middle_src_ty_fold.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/adt.rs
    pub mod rustc_middle_src_ty_adt {
        include!("processed_rustc_middle_src_ty_adt.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/solve.rs
    pub mod rustc_middle_src_traits_solve {
        include!("processed_rustc_middle_src_traits_solve.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/error.rs
    pub mod rustc_middle_rustc_middle_src_error {
        include!("processed_rustc_middle_rustc_middle_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation.rs
    pub mod rustc_middle_mir_interpret_allocation {
        include!("processed_rustc_middle_mir_interpret_allocation.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/structural_impls.rs
    pub mod rustc_middle_src_traits_structural_impls {
        include!("processed_rustc_middle_src_traits_structural_impls.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hooks/mod.rs
    pub mod rustc_middle_src_hooks_mod {
        include!("processed_rustc_middle_src_hooks_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/generics.rs
    pub mod rustc_middle_src_ty_generics {
        include!("processed_rustc_middle_src_ty_generics.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/allocation/provenance_map.rs
    pub mod rustc_middle_interpret_allocation_provenance_map {
        include!("processed_rustc_middle_interpret_allocation_provenance_map.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/context.rs
    pub mod rustc_middle_src_ty_context {
        include!("processed_rustc_middle_src_ty_context.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/debugger_visualizer.rs
    pub mod rustc_middle_src_middle_debugger_visualizer {
        include!("processed_rustc_middle_src_middle_debugger_visualizer.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/generic_graphviz.rs
    pub mod rustc_middle_src_mir_generic_graphviz {
        include!("processed_rustc_middle_src_mir_generic_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/traversal.rs
    pub mod rustc_middle_src_mir_traversal {
        include!("processed_rustc_middle_src_mir_traversal.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/intrinsic.rs
    pub mod rustc_middle_src_ty_intrinsic {
        include!("processed_rustc_middle_src_ty_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/mod.rs
    pub mod rustc_middle_src_middle_mod {
        include!("processed_rustc_middle_src_middle_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/error.rs
    pub mod rustc_middle_mir_interpret_error {
        include!("processed_rustc_middle_mir_interpret_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/consts/int.rs
    pub mod rustc_middle_ty_consts_int {
        include!("processed_rustc_middle_ty_consts_int.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/basic_blocks.rs
    pub mod rustc_middle_src_mir_basic_blocks {
        include!("processed_rustc_middle_src_mir_basic_blocks.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/nested_filter.rs
    pub mod rustc_middle_src_hir_nested_filter {
        include!("processed_rustc_middle_src_hir_nested_filter.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/codegen_fn_attrs.rs
    pub mod rustc_middle_src_middle_codegen_fn_attrs {
        include!("processed_rustc_middle_src_middle_codegen_fn_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/statement.rs
    pub mod rustc_middle_src_mir_statement {
        include!("processed_rustc_middle_src_mir_statement.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/layout.rs
    pub mod rustc_middle_src_ty_layout {
        include!("processed_rustc_middle_src_ty_layout.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/syntax.rs
    pub mod rustc_middle_src_mir_syntax {
        include!("processed_rustc_middle_src_mir_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/tests.rs
    pub mod rustc_middle_rustc_middle_src_tests {
        include!("processed_rustc_middle_rustc_middle_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/map.rs
    pub mod rustc_middle_src_hir_map {
        include!("processed_rustc_middle_src_hir_map.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/values.rs
    pub mod rustc_middle_rustc_middle_src_values {
        include!("processed_rustc_middle_rustc_middle_src_values.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/traits/query.rs
    pub mod rustc_middle_src_traits_query {
        include!("processed_rustc_middle_src_traits_query.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/erase.rs
    pub mod rustc_middle_src_query_erase {
        include!("processed_rustc_middle_src_query_erase.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/privacy.rs
    pub mod rustc_middle_src_middle_privacy {
        include!("processed_rustc_middle_src_middle_privacy.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/infer/canonical.rs
    pub mod rustc_middle_src_infer_canonical {
        include!("processed_rustc_middle_src_infer_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/mod.rs
    pub mod rustc_middle_mir_interpret_mod {
        include!("processed_rustc_middle_mir_interpret_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/dep_graph/mod.rs
    pub mod rustc_middle_src_dep_graph_mod {
        include!("processed_rustc_middle_src_dep_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/graphviz.rs
    pub mod rustc_middle_src_mir_graphviz {
        include!("processed_rustc_middle_src_mir_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/error.rs
    pub mod rustc_middle_src_ty_error {
        include!("processed_rustc_middle_src_ty_error.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/vtable.rs
    pub mod rustc_middle_src_ty_vtable {
        include!("processed_rustc_middle_src_ty_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/macros.rs
    pub mod rustc_middle_rustc_middle_src_macros {
        include!("processed_rustc_middle_rustc_middle_src_macros.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/region.rs
    pub mod rustc_middle_src_ty_region {
        include!("processed_rustc_middle_src_ty_region.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/inhabitedness/inhabited_predicate.rs
    pub mod rustc_middle_ty_inhabitedness_inhabited_predicate {
        include!("processed_rustc_middle_ty_inhabitedness_inhabited_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/on_disk_cache.rs
    pub mod rustc_middle_src_query_on_disk_cache {
        include!("processed_rustc_middle_src_query_on_disk_cache.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/query/arena_cached.rs
    pub mod rustc_middle_src_query_arena_cached {
        include!("processed_rustc_middle_src_query_arena_cached.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/thir/visit.rs
    pub mod rustc_middle_src_thir_visit {
        include!("processed_rustc_middle_src_thir_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/dependency_format.rs
    pub mod rustc_middle_src_middle_dependency_format {
        include!("processed_rustc_middle_src_middle_dependency_format.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/interpret/pointer.rs
    pub mod rustc_middle_mir_interpret_pointer {
        include!("processed_rustc_middle_mir_interpret_pointer.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/exported_symbols.rs
    pub mod rustc_middle_src_middle_exported_symbols {
        include!("processed_rustc_middle_src_middle_exported_symbols.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/diagnostics.rs
    pub mod rustc_middle_src_ty_diagnostics {
        include!("processed_rustc_middle_src_ty_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/typeck_results.rs
    pub mod rustc_middle_src_ty_typeck_results {
        include!("processed_rustc_middle_src_ty_typeck_results.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/consts.rs
    pub mod rustc_middle_src_mir_consts {
        include!("processed_rustc_middle_src_mir_consts.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/dep_graph/dep_node.rs
    pub mod rustc_middle_src_dep_graph_dep_node {
        include!("processed_rustc_middle_src_dep_graph_dep_node.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/print/mod.rs
    pub mod rustc_middle_ty_print_mod {
        include!("processed_rustc_middle_ty_print_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/util/bug.rs
    pub mod rustc_middle_src_util_bug {
        include!("processed_rustc_middle_src_util_bug.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/coverage.rs
    pub mod rustc_middle_src_mir_coverage {
        include!("processed_rustc_middle_src_mir_coverage.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/inhabitedness/mod.rs
    pub mod rustc_middle_ty_inhabitedness_mod {
        include!("processed_rustc_middle_ty_inhabitedness_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/hir/mod.rs
    pub mod rustc_middle_src_hir_mod {
        include!("processed_rustc_middle_src_hir_mod.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/resolve_bound_vars.rs
    pub mod rustc_middle_src_middle_resolve_bound_vars {
        include!("processed_rustc_middle_src_middle_resolve_bound_vars.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/generic_args.rs
    pub mod rustc_middle_src_ty_generic_args {
        include!("processed_rustc_middle_src_ty_generic_args.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/ty/visit.rs
    pub mod rustc_middle_src_ty_visit {
        include!("processed_rustc_middle_src_ty_visit.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/middle/lang_items.rs
    pub mod rustc_middle_src_middle_lang_items {
        include!("processed_rustc_middle_src_middle_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_middle/src/mir/pretty.rs
    pub mod rustc_middle_src_mir_pretty {
        include!("processed_rustc_middle_src_mir_pretty.rs");
    }
}

// 35: rustc_type_ir_macros (1 files)
pub mod included_rustc_type_ir_macros {
    // Source: ../rust/compiler/rustc_type_ir_macros/src/lib.rs
    pub mod rustc_type_ir_macros_rustc_type_ir_macros_src_lib {
        include!("processed_rustc_type_ir_macros_rustc_type_ir_macros_src_lib.rs");
    }
}

// 36: rustc_baked_icu_data (2 files)
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

// 37: rustc_driver_impl (6 files)
pub mod included_rustc_driver_impl {
    // Source: ../rust/compiler/rustc_driver_impl/src/print.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_print {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_print.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/pretty.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_pretty {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/args.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_args {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_args.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/lib.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_lib {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/signal_handler.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_signal_handler {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_signal_handler.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/session_diagnostics.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_session_diagnostics {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_session_diagnostics.rs");
    }
}

// 38: rustc_query_system (19 files)
pub mod included_rustc_query_system {
    // Source: ../rust/compiler/rustc_query_system/src/query/config.rs
    pub mod rustc_query_system_src_query_config {
        include!("processed_rustc_query_system_src_query_config.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/caches.rs
    pub mod rustc_query_system_src_query_caches {
        include!("processed_rustc_query_system_src_query_caches.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/mod.rs
    pub mod rustc_query_system_src_dep_graph_mod {
        include!("processed_rustc_query_system_src_dep_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/job.rs
    pub mod rustc_query_system_src_query_job {
        include!("processed_rustc_query_system_src_query_job.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/query.rs
    pub mod rustc_query_system_src_dep_graph_query {
        include!("processed_rustc_query_system_src_dep_graph_query.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/error.rs
    pub mod rustc_query_system_rustc_query_system_src_error {
        include!("processed_rustc_query_system_rustc_query_system_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/cache.rs
    pub mod rustc_query_system_rustc_query_system_src_cache {
        include!("processed_rustc_query_system_rustc_query_system_src_cache.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/mod.rs
    pub mod rustc_query_system_src_ich_mod {
        include!("processed_rustc_query_system_src_ich_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/graph.rs
    pub mod rustc_query_system_src_dep_graph_graph {
        include!("processed_rustc_query_system_src_dep_graph_graph.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/debug.rs
    pub mod rustc_query_system_src_dep_graph_debug {
        include!("processed_rustc_query_system_src_dep_graph_debug.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/hcx.rs
    pub mod rustc_query_system_src_ich_hcx {
        include!("processed_rustc_query_system_src_ich_hcx.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/lib.rs
    pub mod rustc_query_system_rustc_query_system_src_lib {
        include!("processed_rustc_query_system_rustc_query_system_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/impls_syntax.rs
    pub mod rustc_query_system_src_ich_impls_syntax {
        include!("processed_rustc_query_system_src_ich_impls_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/edges.rs
    pub mod rustc_query_system_src_dep_graph_edges {
        include!("processed_rustc_query_system_src_dep_graph_edges.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/dep_node.rs
    pub mod rustc_query_system_src_dep_graph_dep_node {
        include!("processed_rustc_query_system_src_dep_graph_dep_node.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/values.rs
    pub mod rustc_query_system_rustc_query_system_src_values {
        include!("processed_rustc_query_system_rustc_query_system_src_values.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/plumbing.rs
    pub mod rustc_query_system_src_query_plumbing {
        include!("processed_rustc_query_system_src_query_plumbing.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/mod.rs
    pub mod rustc_query_system_src_query_mod {
        include!("processed_rustc_query_system_src_query_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/serialized.rs
    pub mod rustc_query_system_src_dep_graph_serialized {
        include!("processed_rustc_query_system_src_dep_graph_serialized.rs");
    }
}

// 39: rustc_serialize (8 files)
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
    // Source: ../rust/compiler/rustc_serialize/src/int_overflow.rs
    pub mod rustc_serialize_rustc_serialize_src_int_overflow {
        include!("processed_rustc_serialize_rustc_serialize_src_int_overflow.rs");
    }
    // Source: ../rust/compiler/rustc_serialize/src/leb128/tests.rs
    pub mod rustc_serialize_src_leb128_tests {
        include!("processed_rustc_serialize_src_leb128_tests.rs");
    }
}

// 40: rustc_error_messages (2 files)
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

// 41: rustc_lint (55 files)
pub mod included_rustc_lint {
    // Source: ../rust/compiler/rustc_lint/src/types/literal.rs
    pub mod rustc_lint_src_types_literal {
        include!("processed_rustc_lint_src_types_literal.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/async_closures.rs
    pub mod rustc_lint_rustc_lint_src_async_closures {
        include!("processed_rustc_lint_rustc_lint_src_async_closures.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/autorefs.rs
    pub mod rustc_lint_rustc_lint_src_autorefs {
        include!("processed_rustc_lint_rustc_lint_src_autorefs.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/context.rs
    pub mod rustc_lint_rustc_lint_src_context {
        include!("processed_rustc_lint_rustc_lint_src_context.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/non_ascii_idents.rs
    pub mod rustc_lint_rustc_lint_src_non_ascii_idents {
        include!("processed_rustc_lint_rustc_lint_src_non_ascii_idents.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/passes.rs
    pub mod rustc_lint_rustc_lint_src_passes {
        include!("processed_rustc_lint_rustc_lint_src_passes.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/impl_trait_overcaptures.rs
    pub mod rustc_lint_rustc_lint_src_impl_trait_overcaptures {
        include!("processed_rustc_lint_rustc_lint_src_impl_trait_overcaptures.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/late.rs
    pub mod rustc_lint_rustc_lint_src_late {
        include!("processed_rustc_lint_rustc_lint_src_late.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/redundant_semicolon.rs
    pub mod rustc_lint_rustc_lint_src_redundant_semicolon {
        include!("processed_rustc_lint_rustc_lint_src_redundant_semicolon.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/async_fn_in_trait.rs
    pub mod rustc_lint_rustc_lint_src_async_fn_in_trait {
        include!("processed_rustc_lint_rustc_lint_src_async_fn_in_trait.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/internal.rs
    pub mod rustc_lint_rustc_lint_src_internal {
        include!("processed_rustc_lint_rustc_lint_src_internal.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/precedence.rs
    pub mod rustc_lint_rustc_lint_src_precedence {
        include!("processed_rustc_lint_rustc_lint_src_precedence.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/static_mut_refs.rs
    pub mod rustc_lint_rustc_lint_src_static_mut_refs {
        include!("processed_rustc_lint_rustc_lint_src_static_mut_refs.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/shadowed_into_iter.rs
    pub mod rustc_lint_rustc_lint_src_shadowed_into_iter {
        include!("processed_rustc_lint_rustc_lint_src_shadowed_into_iter.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/levels.rs
    pub mod rustc_lint_rustc_lint_src_levels {
        include!("processed_rustc_lint_rustc_lint_src_levels.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/dangling.rs
    pub mod rustc_lint_rustc_lint_src_dangling {
        include!("processed_rustc_lint_rustc_lint_src_dangling.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/macro_expr_fragment_specifier_2024_migration.rs
    pub mod rustc_lint_rustc_lint_src_macro_expr_fragment_specifier_2024_migration {
        include!("processed_rustc_lint_rustc_lint_src_macro_expr_fragment_specifier_2024_migration.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/enum_intrinsics_non_enums.rs
    pub mod rustc_lint_rustc_lint_src_enum_intrinsics_non_enums {
        include!("processed_rustc_lint_rustc_lint_src_enum_intrinsics_non_enums.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/tests.rs
    pub mod rustc_lint_rustc_lint_src_tests {
        include!("processed_rustc_lint_rustc_lint_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/invalid_from_utf8.rs
    pub mod rustc_lint_rustc_lint_src_invalid_from_utf8 {
        include!("processed_rustc_lint_rustc_lint_src_invalid_from_utf8.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/reference_casting.rs
    pub mod rustc_lint_rustc_lint_src_reference_casting {
        include!("processed_rustc_lint_rustc_lint_src_reference_casting.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/traits.rs
    pub mod rustc_lint_rustc_lint_src_traits {
        include!("processed_rustc_lint_rustc_lint_src_traits.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/errors.rs
    pub mod rustc_lint_rustc_lint_src_errors {
        include!("processed_rustc_lint_rustc_lint_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/unqualified_local_imports.rs
    pub mod rustc_lint_rustc_lint_src_unqualified_local_imports {
        include!("processed_rustc_lint_rustc_lint_src_unqualified_local_imports.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/pass_by_value.rs
    pub mod rustc_lint_rustc_lint_src_pass_by_value {
        include!("processed_rustc_lint_rustc_lint_src_pass_by_value.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/non_fmt_panic.rs
    pub mod rustc_lint_rustc_lint_src_non_fmt_panic {
        include!("processed_rustc_lint_rustc_lint_src_non_fmt_panic.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/unused.rs
    pub mod rustc_lint_rustc_lint_src_unused {
        include!("processed_rustc_lint_rustc_lint_src_unused.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/noop_method_call.rs
    pub mod rustc_lint_rustc_lint_src_noop_method_call {
        include!("processed_rustc_lint_rustc_lint_src_noop_method_call.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/expect.rs
    pub mod rustc_lint_rustc_lint_src_expect {
        include!("processed_rustc_lint_rustc_lint_src_expect.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/transmute.rs
    pub mod rustc_lint_rustc_lint_src_transmute {
        include!("processed_rustc_lint_rustc_lint_src_transmute.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/let_underscore.rs
    pub mod rustc_lint_rustc_lint_src_let_underscore {
        include!("processed_rustc_lint_rustc_lint_src_let_underscore.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/opaque_hidden_inferred_bound.rs
    pub mod rustc_lint_rustc_lint_src_opaque_hidden_inferred_bound {
        include!("processed_rustc_lint_rustc_lint_src_opaque_hidden_inferred_bound.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/deref_into_dyn_supertrait.rs
    pub mod rustc_lint_rustc_lint_src_deref_into_dyn_supertrait {
        include!("processed_rustc_lint_rustc_lint_src_deref_into_dyn_supertrait.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/ptr_nulls.rs
    pub mod rustc_lint_rustc_lint_src_ptr_nulls {
        include!("processed_rustc_lint_rustc_lint_src_ptr_nulls.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/early.rs
    pub mod rustc_lint_rustc_lint_src_early {
        include!("processed_rustc_lint_rustc_lint_src_early.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/if_let_rescope.rs
    pub mod rustc_lint_rustc_lint_src_if_let_rescope {
        include!("processed_rustc_lint_rustc_lint_src_if_let_rescope.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/lints.rs
    pub mod rustc_lint_rustc_lint_src_lints {
        include!("processed_rustc_lint_rustc_lint_src_lints.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/early/diagnostics/check_cfg.rs
    pub mod rustc_lint_early_diagnostics_check_cfg {
        include!("processed_rustc_lint_early_diagnostics_check_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/nonstandard_style/tests.rs
    pub mod rustc_lint_src_nonstandard_style_tests {
        include!("processed_rustc_lint_src_nonstandard_style_tests.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/types/improper_ctypes.rs
    pub mod rustc_lint_src_types_improper_ctypes {
        include!("processed_rustc_lint_src_types_improper_ctypes.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/multiple_supertrait_upcastable.rs
    pub mod rustc_lint_rustc_lint_src_multiple_supertrait_upcastable {
        include!("processed_rustc_lint_rustc_lint_src_multiple_supertrait_upcastable.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/builtin.rs
    pub mod rustc_lint_rustc_lint_src_builtin {
        include!("processed_rustc_lint_rustc_lint_src_builtin.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/default_could_be_derived.rs
    pub mod rustc_lint_rustc_lint_src_default_could_be_derived {
        include!("processed_rustc_lint_rustc_lint_src_default_could_be_derived.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/map_unit_fn.rs
    pub mod rustc_lint_rustc_lint_src_map_unit_fn {
        include!("processed_rustc_lint_rustc_lint_src_map_unit_fn.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/types.rs
    pub mod rustc_lint_rustc_lint_src_types {
        include!("processed_rustc_lint_rustc_lint_src_types.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/non_local_def.rs
    pub mod rustc_lint_rustc_lint_src_non_local_def {
        include!("processed_rustc_lint_rustc_lint_src_non_local_def.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/unit_bindings.rs
    pub mod rustc_lint_rustc_lint_src_unit_bindings {
        include!("processed_rustc_lint_rustc_lint_src_unit_bindings.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/drop_forget_useless.rs
    pub mod rustc_lint_rustc_lint_src_drop_forget_useless {
        include!("processed_rustc_lint_rustc_lint_src_drop_forget_useless.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/foreign_modules.rs
    pub mod rustc_lint_rustc_lint_src_foreign_modules {
        include!("processed_rustc_lint_rustc_lint_src_foreign_modules.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/early/diagnostics.rs
    pub mod rustc_lint_src_early_diagnostics {
        include!("processed_rustc_lint_src_early_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/utils.rs
    pub mod rustc_lint_rustc_lint_src_utils {
        include!("processed_rustc_lint_rustc_lint_src_utils.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/lifetime_syntax.rs
    pub mod rustc_lint_rustc_lint_src_lifetime_syntax {
        include!("processed_rustc_lint_rustc_lint_src_lifetime_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_lint/src/for_loops_over_fallibles.rs
    pub mod rustc_lint_rustc_lint_src_for_loops_over_fallibles {
        include!("processed_rustc_lint_rustc_lint_src_for_loops_over_fallibles.rs");
    }
}

// 42: rustc_next_trait_solver (24 files)
pub mod included_rustc_next_trait_solver {
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/anon_const.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_anon_const {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_anon_const.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/placeholder.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_placeholder {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_placeholder.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/resolve.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_resolve {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_resolve.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/mod.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_mod {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/eval_ctxt/canonical.rs
    pub mod rustc_next_trait_solver_solve_eval_ctxt_canonical {
        include!("processed_rustc_next_trait_solver_solve_eval_ctxt_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/inspect/build.rs
    pub mod rustc_next_trait_solver_solve_inspect_build {
        include!("processed_rustc_next_trait_solver_solve_inspect_build.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/free_alias.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_free_alias {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_free_alias.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/canonicalizer.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_canonicalizer {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_canonicalizer.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/alias_relate.rs
    pub mod rustc_next_trait_solver_src_solve_alias_relate {
        include!("processed_rustc_next_trait_solver_src_solve_alias_relate.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/opaque_types.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_opaque_types {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/delegate.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_delegate {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_delegate.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/inspect/mod.rs
    pub mod rustc_next_trait_solver_solve_inspect_mod {
        include!("processed_rustc_next_trait_solver_solve_inspect_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/inherent.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_inherent {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_inherent.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/lib.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_lib {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/eval_ctxt/mod.rs
    pub mod rustc_next_trait_solver_solve_eval_ctxt_mod {
        include!("processed_rustc_next_trait_solver_solve_eval_ctxt_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/effect_goals.rs
    pub mod rustc_next_trait_solver_src_solve_effect_goals {
        include!("processed_rustc_next_trait_solver_src_solve_effect_goals.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/eval_ctxt/probe.rs
    pub mod rustc_next_trait_solver_solve_eval_ctxt_probe {
        include!("processed_rustc_next_trait_solver_solve_eval_ctxt_probe.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/trait_goals.rs
    pub mod rustc_next_trait_solver_src_solve_trait_goals {
        include!("processed_rustc_next_trait_solver_src_solve_trait_goals.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/coherence.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_coherence {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_coherence.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/mod.rs
    pub mod rustc_next_trait_solver_src_solve_mod {
        include!("processed_rustc_next_trait_solver_src_solve_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/assembly/structural_traits.rs
    pub mod rustc_next_trait_solver_solve_assembly_structural_traits {
        include!("processed_rustc_next_trait_solver_solve_assembly_structural_traits.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/assembly/mod.rs
    pub mod rustc_next_trait_solver_solve_assembly_mod {
        include!("processed_rustc_next_trait_solver_solve_assembly_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/project_goals.rs
    pub mod rustc_next_trait_solver_src_solve_project_goals {
        include!("processed_rustc_next_trait_solver_src_solve_project_goals.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/search_graph.rs
    pub mod rustc_next_trait_solver_src_solve_search_graph {
        include!("processed_rustc_next_trait_solver_src_solve_search_graph.rs");
    }
}

// 43: rustc_type_ir (39 files)
pub mod included_rustc_type_ir {
    // Source: ../rust/compiler/rustc_type_ir/src/search_graph/stack.rs
    pub mod rustc_type_ir_src_search_graph_stack {
        include!("processed_rustc_type_ir_src_search_graph_stack.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/canonical.rs
    pub mod rustc_type_ir_rustc_type_ir_src_canonical {
        include!("processed_rustc_type_ir_rustc_type_ir_src_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/solve/inspect.rs
    pub mod rustc_type_ir_src_solve_inspect {
        include!("processed_rustc_type_ir_src_solve_inspect.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ir_print.rs
    pub mod rustc_type_ir_rustc_type_ir_src_ir_print {
        include!("processed_rustc_type_ir_rustc_type_ir_src_ir_print.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/const_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_const_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_const_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ty_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_ty_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_ty_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/pattern.rs
    pub mod rustc_type_ir_rustc_type_ir_src_pattern {
        include!("processed_rustc_type_ir_rustc_type_ir_src_pattern.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ty_info.rs
    pub mod rustc_type_ir_rustc_type_ir_src_ty_info {
        include!("processed_rustc_type_ir_rustc_type_ir_src_ty_info.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/infer_ctxt.rs
    pub mod rustc_type_ir_rustc_type_ir_src_infer_ctxt {
        include!("processed_rustc_type_ir_rustc_type_ir_src_infer_ctxt.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/data_structures/mod.rs
    pub mod rustc_type_ir_src_data_structures_mod {
        include!("processed_rustc_type_ir_src_data_structures_mod.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/solve/mod.rs
    pub mod rustc_type_ir_src_solve_mod {
        include!("processed_rustc_type_ir_src_solve_mod.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/flags.rs
    pub mod rustc_type_ir_rustc_type_ir_src_flags {
        include!("processed_rustc_type_ir_rustc_type_ir_src_flags.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/predicate_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_predicate_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_predicate_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/search_graph/global_cache.rs
    pub mod rustc_type_ir_src_search_graph_global_cache {
        include!("processed_rustc_type_ir_src_search_graph_global_cache.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/generic_arg.rs
    pub mod rustc_type_ir_rustc_type_ir_src_generic_arg {
        include!("processed_rustc_type_ir_rustc_type_ir_src_generic_arg.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/relate.rs
    pub mod rustc_type_ir_rustc_type_ir_src_relate {
        include!("processed_rustc_type_ir_rustc_type_ir_src_relate.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/elaborate.rs
    pub mod rustc_type_ir_rustc_type_ir_src_elaborate {
        include!("processed_rustc_type_ir_rustc_type_ir_src_elaborate.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/region_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_region_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_region_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/inherent.rs
    pub mod rustc_type_ir_rustc_type_ir_src_inherent {
        include!("processed_rustc_type_ir_rustc_type_ir_src_inherent.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/opaque_ty.rs
    pub mod rustc_type_ir_rustc_type_ir_src_opaque_ty {
        include!("processed_rustc_type_ir_rustc_type_ir_src_opaque_ty.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/predicate.rs
    pub mod rustc_type_ir_rustc_type_ir_src_predicate {
        include!("processed_rustc_type_ir_rustc_type_ir_src_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/upcast.rs
    pub mod rustc_type_ir_rustc_type_ir_src_upcast {
        include!("processed_rustc_type_ir_rustc_type_ir_src_upcast.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/error.rs
    pub mod rustc_type_ir_rustc_type_ir_src_error {
        include!("processed_rustc_type_ir_rustc_type_ir_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/fold.rs
    pub mod rustc_type_ir_rustc_type_ir_src_fold {
        include!("processed_rustc_type_ir_rustc_type_ir_src_fold.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/outlives.rs
    pub mod rustc_type_ir_rustc_type_ir_src_outlives {
        include!("processed_rustc_type_ir_rustc_type_ir_src_outlives.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/visit.rs
    pub mod rustc_type_ir_rustc_type_ir_src_visit {
        include!("processed_rustc_type_ir_rustc_type_ir_src_visit.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/binder.rs
    pub mod rustc_type_ir_rustc_type_ir_src_binder {
        include!("processed_rustc_type_ir_rustc_type_ir_src_binder.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/data_structures/delayed_map.rs
    pub mod rustc_type_ir_src_data_structures_delayed_map {
        include!("processed_rustc_type_ir_src_data_structures_delayed_map.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/search_graph/mod.rs
    pub mod rustc_type_ir_src_search_graph_mod {
        include!("processed_rustc_type_ir_src_search_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/lift.rs
    pub mod rustc_type_ir_rustc_type_ir_src_lift {
        include!("processed_rustc_type_ir_rustc_type_ir_src_lift.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/relate/combine.rs
    pub mod rustc_type_ir_src_relate_combine {
        include!("processed_rustc_type_ir_src_relate_combine.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/lang_items.rs
    pub mod rustc_type_ir_rustc_type_ir_src_lang_items {
        include!("processed_rustc_type_ir_rustc_type_ir_src_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/relate/solver_relating.rs
    pub mod rustc_type_ir_src_relate_solver_relating {
        include!("processed_rustc_type_ir_src_relate_solver_relating.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/fast_reject.rs
    pub mod rustc_type_ir_rustc_type_ir_src_fast_reject {
        include!("processed_rustc_type_ir_rustc_type_ir_src_fast_reject.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ty_kind/closure.rs
    pub mod rustc_type_ir_src_ty_kind_closure {
        include!("processed_rustc_type_ir_src_ty_kind_closure.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/macros.rs
    pub mod rustc_type_ir_rustc_type_ir_src_macros {
        include!("processed_rustc_type_ir_rustc_type_ir_src_macros.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/interner.rs
    pub mod rustc_type_ir_rustc_type_ir_src_interner {
        include!("processed_rustc_type_ir_rustc_type_ir_src_interner.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/walk.rs
    pub mod rustc_type_ir_rustc_type_ir_src_walk {
        include!("processed_rustc_type_ir_rustc_type_ir_src_walk.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/lib.rs
    pub mod rustc_type_ir_rustc_type_ir_src_lib {
        include!("processed_rustc_type_ir_rustc_type_ir_src_lib.rs");
    }
}

// 44: rustc_abi (11 files)
pub mod included_rustc_abi {
    // Source: ../rust/compiler/rustc_abi/src/extern_abi/tests.rs
    pub mod rustc_abi_src_extern_abi_tests {
        include!("processed_rustc_abi_src_extern_abi_tests.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/layout/coroutine.rs
    pub mod rustc_abi_src_layout_coroutine {
        include!("processed_rustc_abi_src_layout_coroutine.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/callconv.rs
    pub mod rustc_abi_rustc_abi_src_callconv {
        include!("processed_rustc_abi_rustc_abi_src_callconv.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/layout/simple.rs
    pub mod rustc_abi_src_layout_simple {
        include!("processed_rustc_abi_src_layout_simple.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/tests.rs
    pub mod rustc_abi_rustc_abi_src_tests {
        include!("processed_rustc_abi_rustc_abi_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/canon_abi.rs
    pub mod rustc_abi_rustc_abi_src_canon_abi {
        include!("processed_rustc_abi_rustc_abi_src_canon_abi.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/callconv/reg.rs
    pub mod rustc_abi_src_callconv_reg {
        include!("processed_rustc_abi_src_callconv_reg.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/layout.rs
    pub mod rustc_abi_rustc_abi_src_layout {
        include!("processed_rustc_abi_rustc_abi_src_layout.rs");
    }
    // Source: ../rust/compiler/rustc_abi/src/layout/ty.rs
    pub mod rustc_abi_src_layout_ty {
        include!("processed_rustc_abi_src_layout_ty.rs");
    }
}

// 45: rustc_transmute (8 files)
pub mod included_rustc_transmute {
    // Source: ../rust/compiler/rustc_transmute/src/lib.rs
    pub mod rustc_transmute_rustc_transmute_src_lib {
        include!("processed_rustc_transmute_rustc_transmute_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_transmute/src/layout/tree/tests.rs
    pub mod rustc_transmute_layout_tree_tests {
        include!("processed_rustc_transmute_layout_tree_tests.rs");
    }
    // Source: ../rust/compiler/rustc_transmute/src/maybe_transmutable/tests.rs
    pub mod rustc_transmute_src_maybe_transmutable_tests {
        include!("processed_rustc_transmute_src_maybe_transmutable_tests.rs");
    }
}

// 46: rustc_codegen_gcc (84 files)
pub mod included_rustc_codegen_gcc {
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/clean.rs
    pub mod rustc_codegen_gcc_build_system_src_clean {
        include!("processed_rustc_codegen_gcc_build_system_src_clean.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/closure.rs
    pub mod rustc_codegen_gcc_tests_run_closure {
        include!("processed_rustc_codegen_gcc_tests_run_closure.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/mini_core.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_mini_core {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_mini_core.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/lang_tests_common.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_tests_lang_tests_common {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_tests_lang_tests_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/alloc_system.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_alloc_system {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_alloc_system.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/volatile.rs
    pub mod rustc_codegen_gcc_tests_run_volatile {
        include!("processed_rustc_codegen_gcc_tests_run_volatile.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/abi.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_abi {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_abi.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/always_inline.rs
    pub mod rustc_codegen_gcc_tests_run_always_inline {
        include!("processed_rustc_codegen_gcc_tests_run_always_inline.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/subslice-patterns-const-eval.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_subslice_patterns_const_eval {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_subslice-patterns-const-eval.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/intrinsic/archs.rs
    pub mod rustc_codegen_gcc_src_intrinsic_archs {
        include!("processed_rustc_codegen_gcc_src_intrinsic_archs.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/gcc_util.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_gcc_util {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_gcc_util.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/abi_test.rs
    pub mod rustc_codegen_gcc_build_system_src_abi_test {
        include!("processed_rustc_codegen_gcc_build_system_src_abi_test.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/debuginfo.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_debuginfo {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_debuginfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/structs.rs
    pub mod rustc_codegen_gcc_tests_run_structs {
        include!("processed_rustc_codegen_gcc_tests_run_structs.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/config.rs
    pub mod rustc_codegen_gcc_build_system_src_config {
        include!("processed_rustc_codegen_gcc_build_system_src_config.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/attributes.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_attributes {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_attributes.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/info.rs
    pub mod rustc_codegen_gcc_build_system_src_info {
        include!("processed_rustc_codegen_gcc_build_system_src_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/mut_ref.rs
    pub mod rustc_codegen_gcc_tests_run_mut_ref {
        include!("processed_rustc_codegen_gcc_tests_run_mut_ref.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/lib.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_lib {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/abort1.rs
    pub mod rustc_codegen_gcc_tests_run_abort1 {
        include!("processed_rustc_codegen_gcc_tests_run_abort1.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/type_.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_type_ {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_type_.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/operations.rs
    pub mod rustc_codegen_gcc_tests_run_operations {
        include!("processed_rustc_codegen_gcc_tests_run_operations.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/mini_core_hello_world.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_mini_core_hello_world {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_mini_core_hello_world.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/volatile2.rs
    pub mod rustc_codegen_gcc_tests_run_volatile2 {
        include!("processed_rustc_codegen_gcc_tests_run_volatile2.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/context.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_context {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_context.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/fmt.rs
    pub mod rustc_codegen_gcc_build_system_src_fmt {
        include!("processed_rustc_codegen_gcc_build_system_src_fmt.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/intrinsic/simd.rs
    pub mod rustc_codegen_gcc_src_intrinsic_simd {
        include!("processed_rustc_codegen_gcc_src_intrinsic_simd.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/test.rs
    pub mod rustc_codegen_gcc_build_system_src_test {
        include!("processed_rustc_codegen_gcc_build_system_src_test.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/type_of.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_type_of {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_type_of.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/callee.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_callee {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_callee.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/intrinsic/llvm.rs
    pub mod rustc_codegen_gcc_src_intrinsic_llvm {
        include!("processed_rustc_codegen_gcc_src_intrinsic_llvm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/mono_item.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_mono_item {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_mono_item.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/static.rs
    pub mod rustc_codegen_gcc_tests_run_static {
        include!("processed_rustc_codegen_gcc_tests_run_static.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/condition.rs
    pub mod rustc_codegen_gcc_tests_run_condition {
        include!("processed_rustc_codegen_gcc_tests_run_condition.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/fuzz.rs
    pub mod rustc_codegen_gcc_build_system_src_fuzz {
        include!("processed_rustc_codegen_gcc_build_system_src_fuzz.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/builder.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_builder {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_builder.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/assign.rs
    pub mod rustc_codegen_gcc_tests_run_assign {
        include!("processed_rustc_codegen_gcc_tests_run_assign.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/rust_tools.rs
    pub mod rustc_codegen_gcc_build_system_src_rust_tools {
        include!("processed_rustc_codegen_gcc_build_system_src_rust_tools.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/allocator.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_allocator {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/back/lto.rs
    pub mod rustc_codegen_gcc_src_back_lto {
        include!("processed_rustc_codegen_gcc_src_back_lto.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/empty_main.rs
    pub mod rustc_codegen_gcc_tests_run_empty_main {
        include!("processed_rustc_codegen_gcc_tests_run_empty_main.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/track-caller-attribute.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_track_caller_attribute {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_track-caller-attribute.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/fuzz/reduce.rs
    pub mod rustc_codegen_gcc_src_fuzz_reduce {
        include!("processed_rustc_codegen_gcc_src_fuzz_reduce.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/build.rs
    pub mod rustc_codegen_gcc_build_system_src_build {
        include!("processed_rustc_codegen_gcc_build_system_src_build.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/consts.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_consts {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_consts.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/ptr_cast.rs
    pub mod rustc_codegen_gcc_tests_run_ptr_cast {
        include!("processed_rustc_codegen_gcc_tests_run_ptr_cast.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/int.rs
    pub mod rustc_codegen_gcc_tests_run_int {
        include!("processed_rustc_codegen_gcc_tests_run_int.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/example.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_example {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/switchint_128bit.rs
    pub mod rustc_codegen_gcc_tests_run_switchint_128bit {
        include!("processed_rustc_codegen_gcc_tests_run_switchint_128bit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/tuple.rs
    pub mod rustc_codegen_gcc_tests_run_tuple {
        include!("processed_rustc_codegen_gcc_tests_run_tuple.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/lang_tests_release.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_tests_lang_tests_release {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_tests_lang_tests_release.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/lang_tests_debug.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_tests_lang_tests_debug {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_tests_lang_tests_debug.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/hello-world/mylib/src/lib.rs
    pub mod rustc_codegen_gcc_mylib_src_lib {
        include!("processed_rustc_codegen_gcc_mylib_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/alloc_example.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_alloc_example {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_alloc_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/asm.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_asm {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/intrinsic/mod.rs
    pub mod rustc_codegen_gcc_src_intrinsic_mod {
        include!("processed_rustc_codegen_gcc_src_intrinsic_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/main.rs
    pub mod rustc_codegen_gcc_build_system_src_main {
        include!("processed_rustc_codegen_gcc_build_system_src_main.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/declare.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_declare {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_declare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/clone_gcc.rs
    pub mod rustc_codegen_gcc_build_system_src_clone_gcc {
        include!("processed_rustc_codegen_gcc_build_system_src_clone_gcc.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/exit_code.rs
    pub mod rustc_codegen_gcc_tests_run_exit_code {
        include!("processed_rustc_codegen_gcc_tests_run_exit_code.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/slice.rs
    pub mod rustc_codegen_gcc_tests_run_slice {
        include!("processed_rustc_codegen_gcc_tests_run_slice.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/abort2.rs
    pub mod rustc_codegen_gcc_tests_run_abort2 {
        include!("processed_rustc_codegen_gcc_tests_run_abort2.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/array.rs
    pub mod rustc_codegen_gcc_tests_run_array {
        include!("processed_rustc_codegen_gcc_tests_run_array.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/std_example.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_std_example {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_std_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/hello-world/src/main.rs
    pub mod rustc_codegen_gcc_hello_world_src_main {
        include!("processed_rustc_codegen_gcc_hello_world_src_main.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/float.rs
    pub mod rustc_codegen_gcc_tests_run_float {
        include!("processed_rustc_codegen_gcc_tests_run_float.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/int.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_int {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_int.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/gep.rs
    pub mod rustc_codegen_gcc_tests_run_gep {
        include!("processed_rustc_codegen_gcc_tests_run_gep.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/back/write.rs
    pub mod rustc_codegen_gcc_src_back_write {
        include!("processed_rustc_codegen_gcc_src_back_write.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/common.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_common {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/packed_u128.rs
    pub mod rustc_codegen_gcc_tests_run_packed_u128 {
        include!("processed_rustc_codegen_gcc_tests_run_packed_u128.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/asm.rs
    pub mod rustc_codegen_gcc_tests_run_asm {
        include!("processed_rustc_codegen_gcc_tests_run_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/arbitrary_self_types_pointers_and_wrappers.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_arbitrary_self_types_pointers_and_wrappers {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_arbitrary_self_types_pointers_and_wrappers.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/return-tuple.rs
    pub mod rustc_codegen_gcc_tests_run_return_tuple {
        include!("processed_rustc_codegen_gcc_tests_run_return-tuple.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/int_overflow.rs
    pub mod rustc_codegen_gcc_tests_run_int_overflow {
        include!("processed_rustc_codegen_gcc_tests_run_int_overflow.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/prepare.rs
    pub mod rustc_codegen_gcc_build_system_src_prepare {
        include!("processed_rustc_codegen_gcc_build_system_src_prepare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/exit.rs
    pub mod rustc_codegen_gcc_tests_run_exit {
        include!("processed_rustc_codegen_gcc_tests_run_exit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/base.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_base {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/example/dst-field-align.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_example_dst_field_align {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_example_dst-field-align.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/build_system/src/rustc_info.rs
    pub mod rustc_codegen_gcc_build_system_src_rustc_info {
        include!("processed_rustc_codegen_gcc_build_system_src_rustc_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/coverageinfo.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_coverageinfo {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_coverageinfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/tests/run/fun_ptr.rs
    pub mod rustc_codegen_gcc_tests_run_fun_ptr {
        include!("processed_rustc_codegen_gcc_tests_run_fun_ptr.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_gcc/src/errors.rs
    pub mod rustc_codegen_gcc_rustc_codegen_gcc_src_errors {
        include!("processed_rustc_codegen_gcc_rustc_codegen_gcc_src_errors.rs");
    }
}

// 47: rustc_feature (6 files)
pub mod included_rustc_feature {
    // Source: ../rust/compiler/rustc_feature/src/removed.rs
    pub mod rustc_feature_rustc_feature_src_removed {
        include!("processed_rustc_feature_rustc_feature_src_removed.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/builtin_attrs.rs
    pub mod rustc_feature_rustc_feature_src_builtin_attrs {
        include!("processed_rustc_feature_rustc_feature_src_builtin_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/unstable.rs
    pub mod rustc_feature_rustc_feature_src_unstable {
        include!("processed_rustc_feature_rustc_feature_src_unstable.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/tests.rs
    pub mod rustc_feature_rustc_feature_src_tests {
        include!("processed_rustc_feature_rustc_feature_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_feature/src/accepted.rs
    pub mod rustc_feature_rustc_feature_src_accepted {
        include!("processed_rustc_feature_rustc_feature_src_accepted.rs");
    }
}

// 48: rustc_codegen_llvm (49 files)
pub mod included_rustc_codegen_llvm {
    // Source: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/mod.rs
    pub mod rustc_codegen_llvm_src_coverageinfo_mod {
        include!("processed_rustc_codegen_llvm_src_coverageinfo_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/errors.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_errors {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/context.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_context {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_context.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/abi.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_abi {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_abi.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/metadata/type_map.rs
    pub mod rustc_codegen_llvm_debuginfo_metadata_type_map {
        include!("processed_rustc_codegen_llvm_debuginfo_metadata_type_map.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/llvm/mod.rs
    pub mod rustc_codegen_llvm_src_llvm_mod {
        include!("processed_rustc_codegen_llvm_src_llvm_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/builder.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_builder {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_builder.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/mod.rs
    pub mod rustc_codegen_llvm_src_debuginfo_mod {
        include!("processed_rustc_codegen_llvm_src_debuginfo_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/namespace.rs
    pub mod rustc_codegen_llvm_src_debuginfo_namespace {
        include!("processed_rustc_codegen_llvm_src_debuginfo_namespace.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/builder/gpu_offload.rs
    pub mod rustc_codegen_llvm_src_builder_gpu_offload {
        include!("processed_rustc_codegen_llvm_src_builder_gpu_offload.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/builder/autodiff.rs
    pub mod rustc_codegen_llvm_src_builder_autodiff {
        include!("processed_rustc_codegen_llvm_src_builder_autodiff.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/callee.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_callee {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_callee.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/lib.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_lib {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/asm.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_asm {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/type_of.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_type_of {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_type_of.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen/spans.rs
    pub mod rustc_codegen_llvm_coverageinfo_mapgen_spans {
        include!("processed_rustc_codegen_llvm_coverageinfo_mapgen_spans.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/type_.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_type_ {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_type_.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/cpp_like.rs
    pub mod rustc_codegen_llvm_metadata_enums_cpp_like {
        include!("processed_rustc_codegen_llvm_metadata_enums_cpp_like.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/llvm/enzyme_ffi.rs
    pub mod rustc_codegen_llvm_src_llvm_enzyme_ffi {
        include!("processed_rustc_codegen_llvm_src_llvm_enzyme_ffi.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/ffi.rs
    pub mod rustc_codegen_llvm_src_coverageinfo_ffi {
        include!("processed_rustc_codegen_llvm_src_coverageinfo_ffi.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/back/archive.rs
    pub mod rustc_codegen_llvm_src_back_archive {
        include!("processed_rustc_codegen_llvm_src_back_archive.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/common.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_common {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/llvm_cov.rs
    pub mod rustc_codegen_llvm_src_coverageinfo_llvm_cov {
        include!("processed_rustc_codegen_llvm_src_coverageinfo_llvm_cov.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/declare.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_declare {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_declare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/gdb.rs
    pub mod rustc_codegen_llvm_src_debuginfo_gdb {
        include!("processed_rustc_codegen_llvm_src_debuginfo_gdb.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs
    pub mod rustc_codegen_llvm_src_debuginfo_metadata {
        include!("processed_rustc_codegen_llvm_src_debuginfo_metadata.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/create_scope_map.rs
    pub mod rustc_codegen_llvm_src_debuginfo_create_scope_map {
        include!("processed_rustc_codegen_llvm_src_debuginfo_create_scope_map.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/intrinsic.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_intrinsic {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/native.rs
    pub mod rustc_codegen_llvm_metadata_enums_native {
        include!("processed_rustc_codegen_llvm_metadata_enums_native.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/mono_item.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_mono_item {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_mono_item.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen/unused.rs
    pub mod rustc_codegen_llvm_coverageinfo_mapgen_unused {
        include!("processed_rustc_codegen_llvm_coverageinfo_mapgen_unused.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/back/lto.rs
    pub mod rustc_codegen_llvm_src_back_lto {
        include!("processed_rustc_codegen_llvm_src_back_lto.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen.rs
    pub mod rustc_codegen_llvm_src_coverageinfo_mapgen {
        include!("processed_rustc_codegen_llvm_src_coverageinfo_mapgen.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/llvm_util.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_llvm_util {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_llvm_util.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/attributes.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_attributes {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_attributes.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/allocator.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_allocator {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/dwarf_const.rs
    pub mod rustc_codegen_llvm_src_debuginfo_dwarf_const {
        include!("processed_rustc_codegen_llvm_src_debuginfo_dwarf_const.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/back/owned_target_machine.rs
    pub mod rustc_codegen_llvm_src_back_owned_target_machine {
        include!("processed_rustc_codegen_llvm_src_back_owned_target_machine.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/va_arg.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_va_arg {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_va_arg.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/back/profiling.rs
    pub mod rustc_codegen_llvm_src_back_profiling {
        include!("processed_rustc_codegen_llvm_src_back_profiling.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/utils.rs
    pub mod rustc_codegen_llvm_src_debuginfo_utils {
        include!("processed_rustc_codegen_llvm_src_debuginfo_utils.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/value.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_value {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_value.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/base.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_base {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/back/write.rs
    pub mod rustc_codegen_llvm_src_back_write {
        include!("processed_rustc_codegen_llvm_src_back_write.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/llvm/ffi.rs
    pub mod rustc_codegen_llvm_src_llvm_ffi {
        include!("processed_rustc_codegen_llvm_src_llvm_ffi.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/debuginfo/metadata/enums/mod.rs
    pub mod rustc_codegen_llvm_metadata_enums_mod {
        include!("processed_rustc_codegen_llvm_metadata_enums_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/mapgen/covfun.rs
    pub mod rustc_codegen_llvm_coverageinfo_mapgen_covfun {
        include!("processed_rustc_codegen_llvm_coverageinfo_mapgen_covfun.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/llvm/diagnostic.rs
    pub mod rustc_codegen_llvm_src_llvm_diagnostic {
        include!("processed_rustc_codegen_llvm_src_llvm_diagnostic.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_llvm/src/consts.rs
    pub mod rustc_codegen_llvm_rustc_codegen_llvm_src_consts {
        include!("processed_rustc_codegen_llvm_rustc_codegen_llvm_src_consts.rs");
    }
}

// 49: rustc_fs_util (1 files)
pub mod included_rustc_fs_util {
    // Source: ../rust/compiler/rustc_fs_util/src/lib.rs
    pub mod rustc_fs_util_rustc_fs_util_src_lib {
        include!("processed_rustc_fs_util_rustc_fs_util_src_lib.rs");
    }
}

// 50: rustc_parse (24 files)
pub mod included_rustc_parse {
    // Source: ../rust/compiler/rustc_parse/src/parser/token_type.rs
    pub mod rustc_parse_src_parser_token_type {
        include!("processed_rustc_parse_src_parser_token_type.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/tests.rs
    pub mod rustc_parse_src_parser_tests {
        include!("processed_rustc_parse_src_parser_tests.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/generics.rs
    pub mod rustc_parse_src_parser_generics {
        include!("processed_rustc_parse_src_parser_generics.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lexer/unescape_error_reporting.rs
    pub mod rustc_parse_src_lexer_unescape_error_reporting {
        include!("processed_rustc_parse_src_lexer_unescape_error_reporting.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/asm.rs
    pub mod rustc_parse_src_parser_asm {
        include!("processed_rustc_parse_src_parser_asm.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/tokenstream/tests.rs
    pub mod rustc_parse_parser_tokenstream_tests {
        include!("processed_rustc_parse_parser_tokenstream_tests.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lexer/unicode_chars.rs
    pub mod rustc_parse_src_lexer_unicode_chars {
        include!("processed_rustc_parse_src_lexer_unicode_chars.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/diagnostics.rs
    pub mod rustc_parse_src_parser_diagnostics {
        include!("processed_rustc_parse_src_parser_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/attr_wrapper.rs
    pub mod rustc_parse_src_parser_attr_wrapper {
        include!("processed_rustc_parse_src_parser_attr_wrapper.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/item.rs
    pub mod rustc_parse_src_parser_item {
        include!("processed_rustc_parse_src_parser_item.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/nonterminal.rs
    pub mod rustc_parse_src_parser_nonterminal {
        include!("processed_rustc_parse_src_parser_nonterminal.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lib.rs
    pub mod rustc_parse_rustc_parse_src_lib {
        include!("processed_rustc_parse_rustc_parse_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lexer/mod.rs
    pub mod rustc_parse_src_lexer_mod {
        include!("processed_rustc_parse_src_lexer_mod.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/stmt.rs
    pub mod rustc_parse_src_parser_stmt {
        include!("processed_rustc_parse_src_parser_stmt.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lexer/diagnostics.rs
    pub mod rustc_parse_src_lexer_diagnostics {
        include!("processed_rustc_parse_src_lexer_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/cfg_select.rs
    pub mod rustc_parse_src_parser_cfg_select {
        include!("processed_rustc_parse_src_parser_cfg_select.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/attr.rs
    pub mod rustc_parse_src_parser_attr {
        include!("processed_rustc_parse_src_parser_attr.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/expr.rs
    pub mod rustc_parse_src_parser_expr {
        include!("processed_rustc_parse_src_parser_expr.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/errors.rs
    pub mod rustc_parse_rustc_parse_src_errors {
        include!("processed_rustc_parse_rustc_parse_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/ty.rs
    pub mod rustc_parse_src_parser_ty {
        include!("processed_rustc_parse_src_parser_ty.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/pat.rs
    pub mod rustc_parse_src_parser_pat {
        include!("processed_rustc_parse_src_parser_pat.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/lexer/tokentrees.rs
    pub mod rustc_parse_src_lexer_tokentrees {
        include!("processed_rustc_parse_src_lexer_tokentrees.rs");
    }
    // Source: ../rust/compiler/rustc_parse/src/parser/path.rs
    pub mod rustc_parse_src_parser_path {
        include!("processed_rustc_parse_src_parser_path.rs");
    }
}

// 51: rustc_builtin_macros (49 files)
pub mod included_rustc_builtin_macros {
    // Source: ../rust/compiler/rustc_builtin_macros/src/standard_library_imports.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_standard_library_imports {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_standard_library_imports.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/derive.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_derive {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_derive.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/partial_eq.rs
    pub mod rustc_builtin_macros_deriving_cmp_partial_eq {
        include!("processed_rustc_builtin_macros_deriving_cmp_partial_eq.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/concat.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_concat {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_concat.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/coerce_pointee.rs
    pub mod rustc_builtin_macros_src_deriving_coerce_pointee {
        include!("processed_rustc_builtin_macros_src_deriving_coerce_pointee.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/pattern_type.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_pattern_type {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_pattern_type.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/iter.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_iter {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_iter.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/trace_macros.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_trace_macros {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_trace_macros.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/assert.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_assert {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_assert.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/autodiff.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_autodiff {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_autodiff.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/compile_error.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_compile_error {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_compile_error.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/asm.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_asm {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_asm.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/alloc_error_handler.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_alloc_error_handler {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_alloc_error_handler.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/edition_panic.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_edition_panic {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_edition_panic.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/debug.rs
    pub mod rustc_builtin_macros_src_deriving_debug {
        include!("processed_rustc_builtin_macros_src_deriving_debug.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs
    pub mod rustc_builtin_macros_format_foreign_printf_tests {
        include!("processed_rustc_builtin_macros_format_foreign_printf_tests.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/util.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_util {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_util.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/eq.rs
    pub mod rustc_builtin_macros_deriving_cmp_eq {
        include!("processed_rustc_builtin_macros_deriving_cmp_eq.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/bounds.rs
    pub mod rustc_builtin_macros_src_deriving_bounds {
        include!("processed_rustc_builtin_macros_src_deriving_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/env.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_env {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_env.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/hash.rs
    pub mod rustc_builtin_macros_src_deriving_hash {
        include!("processed_rustc_builtin_macros_src_deriving_hash.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/errors.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_errors {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/clone.rs
    pub mod rustc_builtin_macros_src_deriving_clone {
        include!("processed_rustc_builtin_macros_src_deriving_clone.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs
    pub mod rustc_builtin_macros_deriving_generic_mod {
        include!("processed_rustc_builtin_macros_deriving_generic_mod.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg_accessible.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg_accessible {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg_accessible.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/from.rs
    pub mod rustc_builtin_macros_src_deriving_from {
        include!("processed_rustc_builtin_macros_src_deriving_from.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/default.rs
    pub mod rustc_builtin_macros_src_deriving_default {
        include!("processed_rustc_builtin_macros_src_deriving_default.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/format_foreign/shell/tests.rs
    pub mod rustc_builtin_macros_format_foreign_shell_tests {
        include!("processed_rustc_builtin_macros_format_foreign_shell_tests.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/lib.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_lib {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/concat_bytes.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_concat_bytes {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_concat_bytes.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/assert/context.rs
    pub mod rustc_builtin_macros_src_assert_context {
        include!("processed_rustc_builtin_macros_src_assert_context.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/generic/ty.rs
    pub mod rustc_builtin_macros_deriving_generic_ty {
        include!("processed_rustc_builtin_macros_deriving_generic_ty.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/contracts.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_contracts {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_contracts.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg_select.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg_select {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg_select.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/ord.rs
    pub mod rustc_builtin_macros_deriving_cmp_ord {
        include!("processed_rustc_builtin_macros_deriving_cmp_ord.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/partial_ord.rs
    pub mod rustc_builtin_macros_deriving_cmp_partial_ord {
        include!("processed_rustc_builtin_macros_deriving_cmp_partial_ord.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/proc_macro_harness.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_proc_macro_harness {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_proc_macro_harness.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/mod.rs
    pub mod rustc_builtin_macros_src_deriving_mod {
        include!("processed_rustc_builtin_macros_src_deriving_mod.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/source_util.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_source_util {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_source_util.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/define_opaque.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_define_opaque {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_define_opaque.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg_eval.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg_eval {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg_eval.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cmdline_attrs.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cmdline_attrs {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cmdline_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/test_harness.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_test_harness {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_test_harness.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/format.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_format {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_format.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/log_syntax.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_log_syntax {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_log_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/global_allocator.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_global_allocator {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_global_allocator.rs");
    }
}

// 52: rustc_ast_passes (4 files)
pub mod included_rustc_ast_passes {
    // Source: ../rust/compiler/rustc_ast_passes/src/errors.rs
    pub mod rustc_ast_passes_rustc_ast_passes_src_errors {
        include!("processed_rustc_ast_passes_rustc_ast_passes_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_ast_passes/src/feature_gate.rs
    pub mod rustc_ast_passes_rustc_ast_passes_src_feature_gate {
        include!("processed_rustc_ast_passes_rustc_ast_passes_src_feature_gate.rs");
    }
    // Source: ../rust/compiler/rustc_ast_passes/src/ast_validation.rs
    pub mod rustc_ast_passes_rustc_ast_passes_src_ast_validation {
        include!("processed_rustc_ast_passes_rustc_ast_passes_src_ast_validation.rs");
    }
    // Source: ../rust/compiler/rustc_ast_passes/src/lib.rs
    pub mod rustc_ast_passes_rustc_ast_passes_src_lib {
        include!("processed_rustc_ast_passes_rustc_ast_passes_src_lib.rs");
    }
}

// 53: rustc_arena (2 files)
pub mod included_rustc_arena {
    // Source: ../rust/compiler/rustc_arena/src/tests.rs
    pub mod rustc_arena_rustc_arena_src_tests {
        include!("processed_rustc_arena_rustc_arena_src_tests.rs");
    }
}

// 54: rustc_expand (19 files)
pub mod included_rustc_expand {
    // Source: ../rust/compiler/rustc_expand/src/base.rs
    pub mod rustc_expand_rustc_expand_src_base {
        include!("processed_rustc_expand_rustc_expand_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/macro_check.rs
    pub mod rustc_expand_src_mbe_macro_check {
        include!("processed_rustc_expand_src_mbe_macro_check.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe.rs
    pub mod rustc_expand_rustc_expand_src_mbe {
        include!("processed_rustc_expand_rustc_expand_src_mbe.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/diagnostics.rs
    pub mod rustc_expand_src_mbe_diagnostics {
        include!("processed_rustc_expand_src_mbe_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/transcribe.rs
    pub mod rustc_expand_src_mbe_transcribe {
        include!("processed_rustc_expand_src_mbe_transcribe.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/macro_parser.rs
    pub mod rustc_expand_src_mbe_macro_parser {
        include!("processed_rustc_expand_src_mbe_macro_parser.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/quoted.rs
    pub mod rustc_expand_src_mbe_quoted {
        include!("processed_rustc_expand_src_mbe_quoted.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/expand.rs
    pub mod rustc_expand_rustc_expand_src_expand {
        include!("processed_rustc_expand_rustc_expand_src_expand.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/lib.rs
    pub mod rustc_expand_rustc_expand_src_lib {
        include!("processed_rustc_expand_rustc_expand_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/proc_macro.rs
    pub mod rustc_expand_rustc_expand_src_proc_macro {
        include!("processed_rustc_expand_rustc_expand_src_proc_macro.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/metavar_expr.rs
    pub mod rustc_expand_src_mbe_metavar_expr {
        include!("processed_rustc_expand_src_mbe_metavar_expr.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/build.rs
    pub mod rustc_expand_rustc_expand_src_build {
        include!("processed_rustc_expand_rustc_expand_src_build.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/errors.rs
    pub mod rustc_expand_rustc_expand_src_errors {
        include!("processed_rustc_expand_rustc_expand_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/mbe/macro_rules.rs
    pub mod rustc_expand_src_mbe_macro_rules {
        include!("processed_rustc_expand_src_mbe_macro_rules.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/proc_macro_server.rs
    pub mod rustc_expand_rustc_expand_src_proc_macro_server {
        include!("processed_rustc_expand_rustc_expand_src_proc_macro_server.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/module.rs
    pub mod rustc_expand_rustc_expand_src_module {
        include!("processed_rustc_expand_rustc_expand_src_module.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/placeholders.rs
    pub mod rustc_expand_rustc_expand_src_placeholders {
        include!("processed_rustc_expand_rustc_expand_src_placeholders.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/stats.rs
    pub mod rustc_expand_rustc_expand_src_stats {
        include!("processed_rustc_expand_rustc_expand_src_stats.rs");
    }
    // Source: ../rust/compiler/rustc_expand/src/config.rs
    pub mod rustc_expand_rustc_expand_src_config {
        include!("processed_rustc_expand_rustc_expand_src_config.rs");
    }
}

// 55: rustc_ast_pretty (11 files)
pub mod included_rustc_ast_pretty {
    // Source: ../rust/compiler/rustc_ast_pretty/src/helpers.rs
    pub mod rustc_ast_pretty_rustc_ast_pretty_src_helpers {
        include!("processed_rustc_ast_pretty_rustc_ast_pretty_src_helpers.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/state.rs
    pub mod rustc_ast_pretty_src_pprust_state {
        include!("processed_rustc_ast_pretty_src_pprust_state.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pp/convenience.rs
    pub mod rustc_ast_pretty_src_pp_convenience {
        include!("processed_rustc_ast_pretty_src_pp_convenience.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pp/ring.rs
    pub mod rustc_ast_pretty_src_pp_ring {
        include!("processed_rustc_ast_pretty_src_pp_ring.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/state/fixup.rs
    pub mod rustc_ast_pretty_pprust_state_fixup {
        include!("processed_rustc_ast_pretty_pprust_state_fixup.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/state/item.rs
    pub mod rustc_ast_pretty_pprust_state_item {
        include!("processed_rustc_ast_pretty_pprust_state_item.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/lib.rs
    pub mod rustc_ast_pretty_rustc_ast_pretty_src_lib {
        include!("processed_rustc_ast_pretty_rustc_ast_pretty_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/tests.rs
    pub mod rustc_ast_pretty_src_pprust_tests {
        include!("processed_rustc_ast_pretty_src_pprust_tests.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pprust/state/expr.rs
    pub mod rustc_ast_pretty_pprust_state_expr {
        include!("processed_rustc_ast_pretty_pprust_state_expr.rs");
    }
    // Source: ../rust/compiler/rustc_ast_pretty/src/pp.rs
    pub mod rustc_ast_pretty_rustc_ast_pretty_src_pp {
        include!("processed_rustc_ast_pretty_rustc_ast_pretty_src_pp.rs");
    }
}

// 56: rustc_ty_utils (17 files)
pub mod included_rustc_ty_utils {
    // Source: ../rust/compiler/rustc_ty_utils/src/representability.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_representability {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_representability.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/implied_bounds.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_implied_bounds {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_implied_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/abi.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_abi {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_abi.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/sig_types.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_sig_types {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_sig_types.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/opaque_types.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_opaque_types {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/needs_drop.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_needs_drop {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_needs_drop.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/ty.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_ty {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_ty.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/errors.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_errors {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/structural_match.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_structural_match {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_structural_match.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/layout.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_layout {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_layout.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/consts.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_consts {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_consts.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/common_traits.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_common_traits {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_common_traits.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/assoc.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_assoc {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_assoc.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/lib.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_lib {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/nested_bodies.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_nested_bodies {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_nested_bodies.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/instance.rs
    pub mod rustc_ty_utils_rustc_ty_utils_src_instance {
        include!("processed_rustc_ty_utils_rustc_ty_utils_src_instance.rs");
    }
    // Source: ../rust/compiler/rustc_ty_utils/src/layout/invariant.rs
    pub mod rustc_ty_utils_src_layout_invariant {
        include!("processed_rustc_ty_utils_src_layout_invariant.rs");
    }
}

// 57: rustc_const_eval (40 files)
pub mod included_rustc_const_eval {
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/mod.rs
    pub mod rustc_const_eval_src_check_consts_mod {
        include!("processed_rustc_const_eval_src_check_consts_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/errors.rs
    pub mod rustc_const_eval_rustc_const_eval_src_errors {
        include!("processed_rustc_const_eval_rustc_const_eval_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/memory.rs
    pub mod rustc_const_eval_src_interpret_memory {
        include!("processed_rustc_const_eval_src_interpret_memory.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/mod.rs
    pub mod rustc_const_eval_src_util_mod {
        include!("processed_rustc_const_eval_src_util_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/qualifs.rs
    pub mod rustc_const_eval_src_check_consts_qualifs {
        include!("processed_rustc_const_eval_src_check_consts_qualifs.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/intrinsics.rs
    pub mod rustc_const_eval_src_interpret_intrinsics {
        include!("processed_rustc_const_eval_src_interpret_intrinsics.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/cast.rs
    pub mod rustc_const_eval_src_interpret_cast {
        include!("processed_rustc_const_eval_src_interpret_cast.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/traits.rs
    pub mod rustc_const_eval_src_interpret_traits {
        include!("processed_rustc_const_eval_src_interpret_traits.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/place.rs
    pub mod rustc_const_eval_src_interpret_place {
        include!("processed_rustc_const_eval_src_interpret_place.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/lib.rs
    pub mod rustc_const_eval_rustc_const_eval_src_lib {
        include!("processed_rustc_const_eval_rustc_const_eval_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/check.rs
    pub mod rustc_const_eval_src_check_consts_check {
        include!("processed_rustc_const_eval_src_check_consts_check.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/machine.rs
    pub mod rustc_const_eval_src_interpret_machine {
        include!("processed_rustc_const_eval_src_interpret_machine.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/operand.rs
    pub mod rustc_const_eval_src_interpret_operand {
        include!("processed_rustc_const_eval_src_interpret_operand.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/ops.rs
    pub mod rustc_const_eval_src_check_consts_ops {
        include!("processed_rustc_const_eval_src_check_consts_ops.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/eval_queries.rs
    pub mod rustc_const_eval_src_const_eval_eval_queries {
        include!("processed_rustc_const_eval_src_const_eval_eval_queries.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/validity.rs
    pub mod rustc_const_eval_src_interpret_validity {
        include!("processed_rustc_const_eval_src_interpret_validity.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/compare_types.rs
    pub mod rustc_const_eval_src_util_compare_types {
        include!("processed_rustc_const_eval_src_util_compare_types.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/check_validity_requirement.rs
    pub mod rustc_const_eval_src_util_check_validity_requirement {
        include!("processed_rustc_const_eval_src_util_check_validity_requirement.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/post_drop_elaboration.rs
    pub mod rustc_const_eval_src_check_consts_post_drop_elaboration {
        include!("processed_rustc_const_eval_src_check_consts_post_drop_elaboration.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/stack.rs
    pub mod rustc_const_eval_src_interpret_stack {
        include!("processed_rustc_const_eval_src_interpret_stack.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/projection.rs
    pub mod rustc_const_eval_src_interpret_projection {
        include!("processed_rustc_const_eval_src_interpret_projection.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/mod.rs
    pub mod rustc_const_eval_src_interpret_mod {
        include!("processed_rustc_const_eval_src_interpret_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/mod.rs
    pub mod rustc_const_eval_src_const_eval_mod {
        include!("processed_rustc_const_eval_src_const_eval_mod.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/call.rs
    pub mod rustc_const_eval_src_interpret_call {
        include!("processed_rustc_const_eval_src_interpret_call.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/intern.rs
    pub mod rustc_const_eval_src_interpret_intern {
        include!("processed_rustc_const_eval_src_interpret_intern.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/caller_location.rs
    pub mod rustc_const_eval_src_util_caller_location {
        include!("processed_rustc_const_eval_src_util_caller_location.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/error.rs
    pub mod rustc_const_eval_src_const_eval_error {
        include!("processed_rustc_const_eval_src_const_eval_error.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/check_consts/resolver.rs
    pub mod rustc_const_eval_src_check_consts_resolver {
        include!("processed_rustc_const_eval_src_check_consts_resolver.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/type_name.rs
    pub mod rustc_const_eval_src_util_type_name {
        include!("processed_rustc_const_eval_src_util_type_name.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/dummy_machine.rs
    pub mod rustc_const_eval_src_const_eval_dummy_machine {
        include!("processed_rustc_const_eval_src_const_eval_dummy_machine.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/visitor.rs
    pub mod rustc_const_eval_src_interpret_visitor {
        include!("processed_rustc_const_eval_src_interpret_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/step.rs
    pub mod rustc_const_eval_src_interpret_step {
        include!("processed_rustc_const_eval_src_interpret_step.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/fn_queries.rs
    pub mod rustc_const_eval_src_const_eval_fn_queries {
        include!("processed_rustc_const_eval_src_const_eval_fn_queries.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/eval_context.rs
    pub mod rustc_const_eval_src_interpret_eval_context {
        include!("processed_rustc_const_eval_src_interpret_eval_context.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/util.rs
    pub mod rustc_const_eval_src_interpret_util {
        include!("processed_rustc_const_eval_src_interpret_util.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/util/alignment.rs
    pub mod rustc_const_eval_src_util_alignment {
        include!("processed_rustc_const_eval_src_util_alignment.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/machine.rs
    pub mod rustc_const_eval_src_const_eval_machine {
        include!("processed_rustc_const_eval_src_const_eval_machine.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/operator.rs
    pub mod rustc_const_eval_src_interpret_operator {
        include!("processed_rustc_const_eval_src_interpret_operator.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/interpret/discriminant.rs
    pub mod rustc_const_eval_src_interpret_discriminant {
        include!("processed_rustc_const_eval_src_interpret_discriminant.rs");
    }
    // Source: ../rust/compiler/rustc_const_eval/src/const_eval/valtrees.rs
    pub mod rustc_const_eval_src_const_eval_valtrees {
        include!("processed_rustc_const_eval_src_const_eval_valtrees.rs");
    }
}

// 58: rustc_graphviz (2 files)
pub mod included_rustc_graphviz {
    // Source: ../rust/compiler/rustc_graphviz/src/tests.rs
    pub mod rustc_graphviz_rustc_graphviz_src_tests {
        include!("processed_rustc_graphviz_rustc_graphviz_src_tests.rs");
    }
}

// 59: rustc_codegen_cranelift (74 files)
pub mod included_rustc_codegen_cranelift {
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/mod.rs
    pub mod rustc_codegen_cranelift_src_abi_mod {
        include!("processed_rustc_codegen_cranelift_src_abi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/track-caller-attribute.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_track_caller_attribute {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_track-caller-attribute.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/dst-field-align.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_dst_field_align {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_dst-field-align.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/base.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_base {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/config.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_config {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_config.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/global_asm.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_global_asm {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_global_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm_aarch64.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm_aarch64 {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm_aarch64.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/subslice-patterns-const-eval.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_subslice_patterns_const_eval {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_subslice-patterns-const-eval.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/toolchain.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_toolchain {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_toolchain.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/utils.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_utils {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_utils.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/linkage.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_linkage {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_linkage.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/object.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_object {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_object.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/mod.rs
    pub mod rustc_codegen_cranelift_src_driver_mod {
        include!("processed_rustc_codegen_cranelift_src_driver_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/cargo-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_cargo_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_cargo-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/simd.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_simd {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_simd.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/mini_core.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/raw-dylib.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_raw_dylib {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_raw-dylib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm_x86.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm_x86 {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm_x86.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/tests.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_tests {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/example.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_example {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/emit.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_emit {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_emit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/gen_block_iterate.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_gen_block_iterate {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_gen_block_iterate.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/unwind.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_unwind {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_unwind.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/rustdoc-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustdoc_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustdoc-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/shared_utils.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_shared_utils {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_shared_utils.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/codegen_i128.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_i128 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_i128.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/lib.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_lib {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/std_example.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_std_example {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_std_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/path.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_path {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_path.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/float-minmax-pass.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_float_minmax_pass {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_float-minmax-pass.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/build_sysroot.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_sysroot {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_sysroot.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/value_and_place.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_value_and_place {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_value_and_place.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/unsize.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_unsize {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_unsize.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/mini_core_hello_world.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core_hello_world {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core_hello_world.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/constant.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_constant {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_constant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/mod.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_mod {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/main.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_main {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_main.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/concurrency_limiter.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_concurrency_limiter {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_concurrency_limiter.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/abi_cafe.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_abi_cafe {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_abi_cafe.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/types.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_types {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_types.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/build_backend.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_backend {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_backend.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/line_info.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_line_info {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_line_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/cast.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_cast {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_cast.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/prepare.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_prepare {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_prepare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/issue-72793.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue_72793 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue-72793.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/common.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_common {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/aot.rs
    pub mod rustc_codegen_cranelift_src_driver_aot {
        include!("processed_rustc_codegen_cranelift_src_driver_aot.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/discriminant.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_discriminant {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_discriminant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/vtable.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_vtable {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/codegen_f16_f128.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_f16_f128 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_f16_f128.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/neon.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_neon {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_neon.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/filter_profile.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_filter_profile {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_filter_profile.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/rustc-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustc_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustc-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/comments.rs
    pub mod rustc_codegen_cranelift_src_abi_comments {
        include!("processed_rustc_codegen_cranelift_src_abi_comments.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/num.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_num {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_num.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/pass_mode.rs
    pub mod rustc_codegen_cranelift_src_abi_pass_mode {
        include!("processed_rustc_codegen_cranelift_src_abi_pass_mode.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/analyze.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_analyze {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_analyze.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/pointer.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_pointer {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_pointer.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/rustc_info.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_rustc_info {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_rustc_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/allocator.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_allocator {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/compiler_builtins.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_compiler_builtins {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_compiler_builtins.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/inline_asm.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_inline_asm {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_inline_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/arbitrary_self_types_pointers_and_wrappers.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_arbitrary_self_types_pointers_and_wrappers {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_arbitrary_self_types_pointers_and_wrappers.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/main_shim.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_main_shim {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_main_shim.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/bench.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_bench {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_bench.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/issue-59326.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue_59326 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue-59326.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/pretty_clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_pretty_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_pretty_clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/optimize/peephole.rs
    pub mod rustc_codegen_cranelift_src_optimize_peephole {
        include!("processed_rustc_codegen_cranelift_src_optimize_peephole.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/config.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_config {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_config.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/jit.rs
    pub mod rustc_codegen_cranelift_src_driver_jit {
        include!("processed_rustc_codegen_cranelift_src_driver_jit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/unwind_module.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_unwind_module {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_unwind_module.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/returning.rs
    pub mod rustc_codegen_cranelift_src_abi_returning {
        include!("processed_rustc_codegen_cranelift_src_abi_returning.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/mod.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_mod {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_mod.rs");
    }
}

// 60: rustc_privacy (2 files)
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

// 61: rustc_hir_id (1 files)
pub mod included_rustc_hir_id {
    // Source: ../rust/compiler/rustc_hir_id/src/lib.rs
    pub mod rustc_hir_id_rustc_hir_id_src_lib {
        include!("processed_rustc_hir_id_rustc_hir_id_src_lib.rs");
    }
}

// 62: rustc_public (26 files)
pub mod included_rustc_public {
    // Source: ../rust/compiler/rustc_public/src/alloc.rs
    pub mod rustc_public_rustc_public_src_alloc {
        include!("processed_rustc_public_rustc_public_src_alloc.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/internal.rs
    pub mod rustc_public_unstable_convert_internal {
        include!("processed_rustc_public_unstable_convert_internal.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/pretty.rs
    pub mod rustc_public_src_mir_pretty {
        include!("processed_rustc_public_src_mir_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/target.rs
    pub mod rustc_public_rustc_public_src_target {
        include!("processed_rustc_public_rustc_public_src_target.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/mono.rs
    pub mod rustc_public_src_mir_mono {
        include!("processed_rustc_public_src_mir_mono.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/stable/ty.rs
    pub mod rustc_public_convert_stable_ty {
        include!("processed_rustc_public_convert_stable_ty.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/alloc.rs
    pub mod rustc_public_src_mir_alloc {
        include!("processed_rustc_public_src_mir_alloc.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/stable/mir.rs
    pub mod rustc_public_convert_stable_mir {
        include!("processed_rustc_public_convert_stable_mir.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/internal_cx/mod.rs
    pub mod rustc_public_unstable_internal_cx_mod {
        include!("processed_rustc_public_unstable_internal_cx_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/abi.rs
    pub mod rustc_public_rustc_public_src_abi {
        include!("processed_rustc_public_rustc_public_src_abi.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/rustc_internal/pretty.rs
    pub mod rustc_public_src_rustc_internal_pretty {
        include!("processed_rustc_public_src_rustc_internal_pretty.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/visitor.rs
    pub mod rustc_public_rustc_public_src_visitor {
        include!("processed_rustc_public_rustc_public_src_visitor.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/error.rs
    pub mod rustc_public_rustc_public_src_error {
        include!("processed_rustc_public_rustc_public_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/compiler_interface.rs
    pub mod rustc_public_rustc_public_src_compiler_interface {
        include!("processed_rustc_public_rustc_public_src_compiler_interface.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/mod.rs
    pub mod rustc_public_src_unstable_mod {
        include!("processed_rustc_public_src_unstable_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/ty.rs
    pub mod rustc_public_rustc_public_src_ty {
        include!("processed_rustc_public_rustc_public_src_ty.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/body.rs
    pub mod rustc_public_src_mir_body {
        include!("processed_rustc_public_src_mir_body.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/crate_def.rs
    pub mod rustc_public_rustc_public_src_crate_def {
        include!("processed_rustc_public_rustc_public_src_crate_def.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/rustc_internal/mod.rs
    pub mod rustc_public_src_rustc_internal_mod {
        include!("processed_rustc_public_src_rustc_internal_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/mod.rs
    pub mod rustc_public_unstable_convert_mod {
        include!("processed_rustc_public_unstable_convert_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir.rs
    pub mod rustc_public_rustc_public_src_mir {
        include!("processed_rustc_public_rustc_public_src_mir.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/stable/mod.rs
    pub mod rustc_public_convert_stable_mod {
        include!("processed_rustc_public_convert_stable_mod.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/lib.rs
    pub mod rustc_public_rustc_public_src_lib {
        include!("processed_rustc_public_rustc_public_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/internal_cx/helpers.rs
    pub mod rustc_public_unstable_internal_cx_helpers {
        include!("processed_rustc_public_unstable_internal_cx_helpers.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/unstable/convert/stable/abi.rs
    pub mod rustc_public_convert_stable_abi {
        include!("processed_rustc_public_convert_stable_abi.rs");
    }
    // Source: ../rust/compiler/rustc_public/src/mir/visit.rs
    pub mod rustc_public_src_mir_visit {
        include!("processed_rustc_public_src_mir_visit.rs");
    }
}

// 63: rustc_borrowck (61 files)
pub mod included_rustc_borrowck {
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/loan_invalidations.rs
    pub mod rustc_borrowck_polonius_legacy_loan_invalidations {
        include!("processed_rustc_borrowck_polonius_legacy_loan_invalidations.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/find_all_local_uses.rs
    pub mod rustc_borrowck_src_diagnostics_find_all_local_uses {
        include!("processed_rustc_borrowck_src_diagnostics_find_all_local_uses.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/mod.rs
    pub mod rustc_borrowck_src_region_infer_mod {
        include!("processed_rustc_borrowck_src_region_infer_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/places_conflict.rs
    pub mod rustc_borrowck_rustc_borrowck_src_places_conflict {
        include!("processed_rustc_borrowck_rustc_borrowck_src_places_conflict.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/loan_kills.rs
    pub mod rustc_borrowck_polonius_legacy_loan_kills {
        include!("processed_rustc_borrowck_polonius_legacy_loan_kills.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/dump_mir.rs
    pub mod rustc_borrowck_src_region_infer_dump_mir {
        include!("processed_rustc_borrowck_src_region_infer_dump_mir.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/var_name.rs
    pub mod rustc_borrowck_src_diagnostics_var_name {
        include!("processed_rustc_borrowck_src_diagnostics_var_name.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/liveness/local_use_map.rs
    pub mod rustc_borrowck_type_check_liveness_local_use_map {
        include!("processed_rustc_borrowck_type_check_liveness_local_use_map.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/loan_liveness.rs
    pub mod rustc_borrowck_src_polonius_loan_liveness {
        include!("processed_rustc_borrowck_src_polonius_loan_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/constraints/graph.rs
    pub mod rustc_borrowck_src_constraints_graph {
        include!("processed_rustc_borrowck_src_constraints_graph.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/opaque_types/region_ctxt.rs
    pub mod rustc_borrowck_region_infer_opaque_types_region_ctxt {
        include!("processed_rustc_borrowck_region_infer_opaque_types_region_ctxt.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/canonical.rs
    pub mod rustc_borrowck_src_type_check_canonical {
        include!("processed_rustc_borrowck_src_type_check_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/path_utils.rs
    pub mod rustc_borrowck_rustc_borrowck_src_path_utils {
        include!("processed_rustc_borrowck_rustc_borrowck_src_path_utils.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/mod.rs
    pub mod rustc_borrowck_src_diagnostics_mod {
        include!("processed_rustc_borrowck_src_diagnostics_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/facts.rs
    pub mod rustc_borrowck_polonius_legacy_facts {
        include!("processed_rustc_borrowck_polonius_legacy_facts.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/dump.rs
    pub mod rustc_borrowck_src_polonius_dump {
        include!("processed_rustc_borrowck_src_polonius_dump.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/borrowck_errors.rs
    pub mod rustc_borrowck_rustc_borrowck_src_borrowck_errors {
        include!("processed_rustc_borrowck_rustc_borrowck_src_borrowck_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/liveness/mod.rs
    pub mod rustc_borrowck_type_check_liveness_mod {
        include!("processed_rustc_borrowck_type_check_liveness_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/region_name.rs
    pub mod rustc_borrowck_src_diagnostics_region_name {
        include!("processed_rustc_borrowck_src_diagnostics_region_name.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/used_muts.rs
    pub mod rustc_borrowck_rustc_borrowck_src_used_muts {
        include!("processed_rustc_borrowck_rustc_borrowck_src_used_muts.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/universal_regions.rs
    pub mod rustc_borrowck_rustc_borrowck_src_universal_regions {
        include!("processed_rustc_borrowck_rustc_borrowck_src_universal_regions.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/accesses.rs
    pub mod rustc_borrowck_polonius_legacy_accesses {
        include!("processed_rustc_borrowck_polonius_legacy_accesses.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/relate_tys.rs
    pub mod rustc_borrowck_src_type_check_relate_tys {
        include!("processed_rustc_borrowck_src_type_check_relate_tys.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/place_ext.rs
    pub mod rustc_borrowck_rustc_borrowck_src_place_ext {
        include!("processed_rustc_borrowck_rustc_borrowck_src_place_ext.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/typeck_constraints.rs
    pub mod rustc_borrowck_src_polonius_typeck_constraints {
        include!("processed_rustc_borrowck_src_polonius_typeck_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/explain_borrow.rs
    pub mod rustc_borrowck_src_diagnostics_explain_borrow {
        include!("processed_rustc_borrowck_src_diagnostics_explain_borrow.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/renumber.rs
    pub mod rustc_borrowck_rustc_borrowck_src_renumber {
        include!("processed_rustc_borrowck_rustc_borrowck_src_renumber.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/nll.rs
    pub mod rustc_borrowck_rustc_borrowck_src_nll {
        include!("processed_rustc_borrowck_rustc_borrowck_src_nll.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs
    pub mod rustc_borrowck_src_diagnostics_conflict_errors {
        include!("processed_rustc_borrowck_src_diagnostics_conflict_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/location.rs
    pub mod rustc_borrowck_polonius_legacy_location {
        include!("processed_rustc_borrowck_polonius_legacy_location.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/opaque_types/member_constraints.rs
    pub mod rustc_borrowck_region_infer_opaque_types_member_constraints {
        include!("processed_rustc_borrowck_region_infer_opaque_types_member_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/handle_placeholders.rs
    pub mod rustc_borrowck_rustc_borrowck_src_handle_placeholders {
        include!("processed_rustc_borrowck_rustc_borrowck_src_handle_placeholders.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/constraints/mod.rs
    pub mod rustc_borrowck_src_constraints_mod {
        include!("processed_rustc_borrowck_src_constraints_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/mod.rs
    pub mod rustc_borrowck_polonius_legacy_mod {
        include!("processed_rustc_borrowck_polonius_legacy_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/opaque_types/mod.rs
    pub mod rustc_borrowck_region_infer_opaque_types_mod {
        include!("processed_rustc_borrowck_region_infer_opaque_types_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/opaque_types.rs
    pub mod rustc_borrowck_src_diagnostics_opaque_types {
        include!("processed_rustc_borrowck_src_diagnostics_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/outlives_suggestion.rs
    pub mod rustc_borrowck_src_diagnostics_outlives_suggestion {
        include!("processed_rustc_borrowck_src_diagnostics_outlives_suggestion.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/root_cx.rs
    pub mod rustc_borrowck_rustc_borrowck_src_root_cx {
        include!("processed_rustc_borrowck_rustc_borrowck_src_root_cx.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/input_output.rs
    pub mod rustc_borrowck_src_type_check_input_output {
        include!("processed_rustc_borrowck_src_type_check_input_output.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/values.rs
    pub mod rustc_borrowck_src_region_infer_values {
        include!("processed_rustc_borrowck_src_region_infer_values.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/free_region_relations.rs
    pub mod rustc_borrowck_src_type_check_free_region_relations {
        include!("processed_rustc_borrowck_src_type_check_free_region_relations.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/session_diagnostics.rs
    pub mod rustc_borrowck_rustc_borrowck_src_session_diagnostics {
        include!("processed_rustc_borrowck_rustc_borrowck_src_session_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/prefixes.rs
    pub mod rustc_borrowck_rustc_borrowck_src_prefixes {
        include!("processed_rustc_borrowck_rustc_borrowck_src_prefixes.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/dataflow.rs
    pub mod rustc_borrowck_rustc_borrowck_src_dataflow {
        include!("processed_rustc_borrowck_rustc_borrowck_src_dataflow.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/consumers.rs
    pub mod rustc_borrowck_rustc_borrowck_src_consumers {
        include!("processed_rustc_borrowck_rustc_borrowck_src_consumers.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/move_errors.rs
    pub mod rustc_borrowck_src_diagnostics_move_errors {
        include!("processed_rustc_borrowck_src_diagnostics_move_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/bound_region_errors.rs
    pub mod rustc_borrowck_src_diagnostics_bound_region_errors {
        include!("processed_rustc_borrowck_src_diagnostics_bound_region_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/def_use.rs
    pub mod rustc_borrowck_rustc_borrowck_src_def_use {
        include!("processed_rustc_borrowck_rustc_borrowck_src_def_use.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/region_errors.rs
    pub mod rustc_borrowck_src_diagnostics_region_errors {
        include!("processed_rustc_borrowck_src_diagnostics_region_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/mod.rs
    pub mod rustc_borrowck_src_polonius_mod {
        include!("processed_rustc_borrowck_src_polonius_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/liveness/trace.rs
    pub mod rustc_borrowck_type_check_liveness_trace {
        include!("processed_rustc_borrowck_type_check_liveness_trace.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs
    pub mod rustc_borrowck_src_diagnostics_mutability_errors {
        include!("processed_rustc_borrowck_src_diagnostics_mutability_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/liveness_constraints.rs
    pub mod rustc_borrowck_src_polonius_liveness_constraints {
        include!("processed_rustc_borrowck_src_polonius_liveness_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/constraint_conversion.rs
    pub mod rustc_borrowck_src_type_check_constraint_conversion {
        include!("processed_rustc_borrowck_src_type_check_constraint_conversion.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/borrow_set.rs
    pub mod rustc_borrowck_rustc_borrowck_src_borrow_set {
        include!("processed_rustc_borrowck_rustc_borrowck_src_borrow_set.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/reverse_sccs.rs
    pub mod rustc_borrowck_src_region_infer_reverse_sccs {
        include!("processed_rustc_borrowck_src_region_infer_reverse_sccs.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/constraints.rs
    pub mod rustc_borrowck_src_polonius_constraints {
        include!("processed_rustc_borrowck_src_polonius_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/lib.rs
    pub mod rustc_borrowck_rustc_borrowck_src_lib {
        include!("processed_rustc_borrowck_rustc_borrowck_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/mod.rs
    pub mod rustc_borrowck_src_type_check_mod {
        include!("processed_rustc_borrowck_src_type_check_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/find_use.rs
    pub mod rustc_borrowck_src_diagnostics_find_use {
        include!("processed_rustc_borrowck_src_diagnostics_find_use.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/graphviz.rs
    pub mod rustc_borrowck_src_region_infer_graphviz {
        include!("processed_rustc_borrowck_src_region_infer_graphviz.rs");
    }
}

// 64: rustc_ast_lowering (12 files)
pub mod included_rustc_ast_lowering {
    // Source: ../rust/compiler/rustc_ast_lowering/src/item.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_item {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_item.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/delegation.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_delegation {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_delegation.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/pat.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_pat {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_pat.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/stability.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_stability {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_stability.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/asm.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_asm {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_asm.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/expr.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_expr {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_expr.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/index.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_index {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_index.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/block.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_block {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_block.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/errors.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_errors {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/lib.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_lib {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/path.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_path {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_path.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/format.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_format {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_format.rs");
    }
}

// 65: rustc_monomorphize (9 files)
pub mod included_rustc_monomorphize {
    // Source: ../rust/compiler/rustc_monomorphize/src/util.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_util {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_util.rs");
    }
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
    // Source: ../rust/compiler/rustc_monomorphize/src/collector.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_collector {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_collector.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/collector/autodiff.rs
    pub mod rustc_monomorphize_src_collector_autodiff {
        include!("processed_rustc_monomorphize_src_collector_autodiff.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/move_check.rs
    pub mod rustc_monomorphize_src_mono_checks_move_check {
        include!("processed_rustc_monomorphize_src_mono_checks_move_check.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/lib.rs
    pub mod rustc_monomorphize_rustc_monomorphize_src_lib {
        include!("processed_rustc_monomorphize_rustc_monomorphize_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_monomorphize/src/mono_checks/mod.rs
    pub mod rustc_monomorphize_src_mono_checks_mod {
        include!("processed_rustc_monomorphize_src_mono_checks_mod.rs");
    }
}

// 66: rustc_hir (23 files)
pub mod included_rustc_hir {
    // Source: ../rust/compiler/rustc_hir/src/lints.rs
    pub mod rustc_hir_rustc_hir_src_lints {
        include!("processed_rustc_hir_rustc_hir_src_lints.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/def.rs
    pub mod rustc_hir_rustc_hir_src_def {
        include!("processed_rustc_hir_rustc_hir_src_def.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/stable_hash_impls.rs
    pub mod rustc_hir_rustc_hir_src_stable_hash_impls {
        include!("processed_rustc_hir_rustc_hir_src_stable_hash_impls.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/tests.rs
    pub mod rustc_hir_rustc_hir_src_tests {
        include!("processed_rustc_hir_rustc_hir_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/definitions.rs
    pub mod rustc_hir_rustc_hir_src_definitions {
        include!("processed_rustc_hir_rustc_hir_src_definitions.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/hir/tests.rs
    pub mod rustc_hir_src_hir_tests {
        include!("processed_rustc_hir_src_hir_tests.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/attrs/pretty_printing.rs
    pub mod rustc_hir_src_attrs_pretty_printing {
        include!("processed_rustc_hir_src_attrs_pretty_printing.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/attrs/data_structures.rs
    pub mod rustc_hir_src_attrs_data_structures {
        include!("processed_rustc_hir_src_attrs_data_structures.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/pat_util.rs
    pub mod rustc_hir_rustc_hir_src_pat_util {
        include!("processed_rustc_hir_rustc_hir_src_pat_util.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/stability.rs
    pub mod rustc_hir_rustc_hir_src_stability {
        include!("processed_rustc_hir_rustc_hir_src_stability.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/limit.rs
    pub mod rustc_hir_rustc_hir_src_limit {
        include!("processed_rustc_hir_rustc_hir_src_limit.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/target.rs
    pub mod rustc_hir_rustc_hir_src_target {
        include!("processed_rustc_hir_rustc_hir_src_target.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/lang_items.rs
    pub mod rustc_hir_rustc_hir_src_lang_items {
        include!("processed_rustc_hir_rustc_hir_src_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/arena.rs
    pub mod rustc_hir_rustc_hir_src_arena {
        include!("processed_rustc_hir_rustc_hir_src_arena.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/attrs/encode_cross_crate.rs
    pub mod rustc_hir_src_attrs_encode_cross_crate {
        include!("processed_rustc_hir_src_attrs_encode_cross_crate.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/def_path_hash_map.rs
    pub mod rustc_hir_rustc_hir_src_def_path_hash_map {
        include!("processed_rustc_hir_rustc_hir_src_def_path_hash_map.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/intravisit.rs
    pub mod rustc_hir_rustc_hir_src_intravisit {
        include!("processed_rustc_hir_rustc_hir_src_intravisit.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/version.rs
    pub mod rustc_hir_rustc_hir_src_version {
        include!("processed_rustc_hir_rustc_hir_src_version.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/weak_lang_items.rs
    pub mod rustc_hir_rustc_hir_src_weak_lang_items {
        include!("processed_rustc_hir_rustc_hir_src_weak_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/attrs/mod.rs
    pub mod rustc_hir_src_attrs_mod {
        include!("processed_rustc_hir_src_attrs_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir/src/diagnostic_items.rs
    pub mod rustc_hir_rustc_hir_src_diagnostic_items {
        include!("processed_rustc_hir_rustc_hir_src_diagnostic_items.rs");
    }
}

// 67: rustc_pattern_analysis (14 files)
pub mod included_rustc_pattern_analysis {
    // Source: ../rust/compiler/rustc_pattern_analysis/src/constructor.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_constructor {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_constructor.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/usefulness.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_usefulness {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_usefulness.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/tests/intersection.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_tests_intersection {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_tests_intersection.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/lib.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_lib {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/rustc/print.rs
    pub mod rustc_pattern_analysis_src_rustc_print {
        include!("processed_rustc_pattern_analysis_src_rustc_print.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/pat_column.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_pat_column {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_pat_column.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/lints.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_lints {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_lints.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/errors.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_errors {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/rustc.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_rustc {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_rustc.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/tests/common/mod.rs
    pub mod rustc_pattern_analysis_tests_common_mod {
        include!("processed_rustc_pattern_analysis_tests_common_mod.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/pat.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_pat {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_pat.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/tests/exhaustiveness.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_tests_exhaustiveness {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_tests_exhaustiveness.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/src/checks.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_src_checks {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_src_checks.rs");
    }
    // Source: ../rust/compiler/rustc_pattern_analysis/tests/complexity.rs
    pub mod rustc_pattern_analysis_rustc_pattern_analysis_tests_complexity {
        include!("processed_rustc_pattern_analysis_rustc_pattern_analysis_tests_complexity.rs");
    }
}

// 68: rustc_hir_analysis (51 files)
pub mod included_rustc_hir_analysis {
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/explicit.rs
    pub mod rustc_hir_analysis_src_outlives_explicit {
        include!("processed_rustc_hir_analysis_src_outlives_explicit.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/lint.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_lint {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_lint.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/autoderef.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_autoderef {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_autoderef.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/impl_wf_check/min_specialization.rs
    pub mod rustc_hir_analysis_src_impl_wf_check_min_specialization {
        include!("processed_rustc_hir_analysis_src_impl_wf_check_min_specialization.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/dump.rs
    pub mod rustc_hir_analysis_src_collect_dump {
        include!("processed_rustc_hir_analysis_src_collect_dump.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/errors/precise_captures.rs
    pub mod rustc_hir_analysis_src_errors_precise_captures {
        include!("processed_rustc_hir_analysis_src_errors_precise_captures.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/implicit_infer.rs
    pub mod rustc_hir_analysis_src_outlives_implicit_infer {
        include!("processed_rustc_hir_analysis_src_outlives_implicit_infer.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/inherent_impls.rs
    pub mod rustc_hir_analysis_src_coherence_inherent_impls {
        include!("processed_rustc_hir_analysis_src_coherence_inherent_impls.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/compare_impl_item.rs
    pub mod rustc_hir_analysis_src_check_compare_impl_item {
        include!("processed_rustc_hir_analysis_src_check_compare_impl_item.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/resolve_bound_vars.rs
    pub mod rustc_hir_analysis_src_collect_resolve_bound_vars {
        include!("processed_rustc_hir_analysis_src_collect_resolve_bound_vars.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/predicates_of.rs
    pub mod rustc_hir_analysis_src_collect_predicates_of {
        include!("processed_rustc_hir_analysis_src_collect_predicates_of.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_wf_check.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_hir_wf_check {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_hir_wf_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/mod.rs
    pub mod rustc_hir_analysis_src_outlives_mod {
        include!("processed_rustc_hir_analysis_src_outlives_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/mod.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_mod {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/constrained_generic_params.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_constrained_generic_params {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_constrained_generic_params.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_collect {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_collect.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/region.rs
    pub mod rustc_hir_analysis_src_check_region {
        include!("processed_rustc_hir_analysis_src_check_region.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/errors/wrong_number_of_generic_args.rs
    pub mod rustc_hir_analysis_src_errors_wrong_number_of_generic_args {
        include!("processed_rustc_hir_analysis_src_errors_wrong_number_of_generic_args.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/errors.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_errors {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/compare_impl_item/refine.rs
    pub mod rustc_hir_analysis_check_compare_impl_item_refine {
        include!("processed_rustc_hir_analysis_check_compare_impl_item_refine.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/delegation.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_delegation {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_delegation.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/solve.rs
    pub mod rustc_hir_analysis_src_variance_solve {
        include!("processed_rustc_hir_analysis_src_variance_solve.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/cmse.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_cmse {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_cmse.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/intrinsic.rs
    pub mod rustc_hir_analysis_src_check_intrinsic {
        include!("processed_rustc_hir_analysis_src_check_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/builtin.rs
    pub mod rustc_hir_analysis_src_coherence_builtin {
        include!("processed_rustc_hir_analysis_src_coherence_builtin.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/dyn_compatibility.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_dyn_compatibility {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_dyn_compatibility.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/lib.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_lib {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/utils.rs
    pub mod rustc_hir_analysis_src_outlives_utils {
        include!("processed_rustc_hir_analysis_src_outlives_utils.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/mod.rs
    pub mod rustc_hir_analysis_src_coherence_mod {
        include!("processed_rustc_hir_analysis_src_coherence_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/terms.rs
    pub mod rustc_hir_analysis_src_variance_terms {
        include!("processed_rustc_hir_analysis_src_variance_terms.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/impl_wf_check.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_impl_wf_check {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_impl_wf_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/generics.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_generics {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_generics.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/type_of.rs
    pub mod rustc_hir_analysis_src_collect_type_of {
        include!("processed_rustc_hir_analysis_src_collect_type_of.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/orphan.rs
    pub mod rustc_hir_analysis_src_coherence_orphan {
        include!("processed_rustc_hir_analysis_src_coherence_orphan.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/generics_of.rs
    pub mod rustc_hir_analysis_src_collect_generics_of {
        include!("processed_rustc_hir_analysis_src_collect_generics_of.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/type_of/opaque.rs
    pub mod rustc_hir_analysis_collect_type_of_opaque {
        include!("processed_rustc_hir_analysis_collect_type_of_opaque.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/always_applicable.rs
    pub mod rustc_hir_analysis_src_check_always_applicable {
        include!("processed_rustc_hir_analysis_src_check_always_applicable.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/entry.rs
    pub mod rustc_hir_analysis_src_check_entry {
        include!("processed_rustc_hir_analysis_src_check_entry.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/unsafety.rs
    pub mod rustc_hir_analysis_src_coherence_unsafety {
        include!("processed_rustc_hir_analysis_src_coherence_unsafety.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/errors.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_errors {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/bounds.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_bounds {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check_unused.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_check_unused {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_check_unused.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/dump.rs
    pub mod rustc_hir_analysis_src_outlives_dump {
        include!("processed_rustc_hir_analysis_src_outlives_dump.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/constraints.rs
    pub mod rustc_hir_analysis_src_variance_constraints {
        include!("processed_rustc_hir_analysis_src_variance_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/mod.rs
    pub mod rustc_hir_analysis_src_variance_mod {
        include!("processed_rustc_hir_analysis_src_variance_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/inherent_impls_overlap.rs
    pub mod rustc_hir_analysis_src_coherence_inherent_impls_overlap {
        include!("processed_rustc_hir_analysis_src_coherence_inherent_impls_overlap.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/dump.rs
    pub mod rustc_hir_analysis_src_variance_dump {
        include!("processed_rustc_hir_analysis_src_variance_dump.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/item_bounds.rs
    pub mod rustc_hir_analysis_src_collect_item_bounds {
        include!("processed_rustc_hir_analysis_src_collect_item_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/check.rs
    pub mod rustc_hir_analysis_src_check_check {
        include!("processed_rustc_hir_analysis_src_check_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/mod.rs
    pub mod rustc_hir_analysis_src_check_mod {
        include!("processed_rustc_hir_analysis_src_check_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/wfcheck.rs
    pub mod rustc_hir_analysis_src_check_wfcheck {
        include!("processed_rustc_hir_analysis_src_check_wfcheck.rs");
    }
}

// 69: rustc_attr_parsing (38 files)
pub mod included_rustc_attr_parsing {
    // Source: ../rust/compiler/rustc_attr_parsing/src/session_diagnostics.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_session_diagnostics {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_session_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/prelude.rs
    pub mod rustc_attr_parsing_src_attributes_prelude {
        include!("processed_rustc_attr_parsing_src_attributes_prelude.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/prototype.rs
    pub mod rustc_attr_parsing_src_attributes_prototype {
        include!("processed_rustc_attr_parsing_src_attributes_prototype.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/parser.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_parser {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_parser.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/no_implicit_prelude.rs
    pub mod rustc_attr_parsing_src_attributes_no_implicit_prelude {
        include!("processed_rustc_attr_parsing_src_attributes_no_implicit_prelude.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/rustc_internal.rs
    pub mod rustc_attr_parsing_src_attributes_rustc_internal {
        include!("processed_rustc_attr_parsing_src_attributes_rustc_internal.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/cfg_old.rs
    pub mod rustc_attr_parsing_src_attributes_cfg_old {
        include!("processed_rustc_attr_parsing_src_attributes_cfg_old.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/cfg.rs
    pub mod rustc_attr_parsing_src_attributes_cfg {
        include!("processed_rustc_attr_parsing_src_attributes_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/interface.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_interface {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_interface.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/codegen_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_codegen_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_codegen_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/mod.rs
    pub mod rustc_attr_parsing_src_attributes_mod {
        include!("processed_rustc_attr_parsing_src_attributes_mod.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/body.rs
    pub mod rustc_attr_parsing_src_attributes_body {
        include!("processed_rustc_attr_parsing_src_attributes_body.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/stability.rs
    pub mod rustc_attr_parsing_src_attributes_stability {
        include!("processed_rustc_attr_parsing_src_attributes_stability.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/allow_unstable.rs
    pub mod rustc_attr_parsing_src_attributes_allow_unstable {
        include!("processed_rustc_attr_parsing_src_attributes_allow_unstable.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/context.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_context {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_context.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/util.rs
    pub mod rustc_attr_parsing_src_attributes_util {
        include!("processed_rustc_attr_parsing_src_attributes_util.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/target_checking.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_target_checking {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_target_checking.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/transparency.rs
    pub mod rustc_attr_parsing_src_attributes_transparency {
        include!("processed_rustc_attr_parsing_src_attributes_transparency.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/repr.rs
    pub mod rustc_attr_parsing_src_attributes_repr {
        include!("processed_rustc_attr_parsing_src_attributes_repr.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/validate_attr.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_validate_attr {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_validate_attr.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/inline.rs
    pub mod rustc_attr_parsing_src_attributes_inline {
        include!("processed_rustc_attr_parsing_src_attributes_inline.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/loop_match.rs
    pub mod rustc_attr_parsing_src_attributes_loop_match {
        include!("processed_rustc_attr_parsing_src_attributes_loop_match.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/crate_level.rs
    pub mod rustc_attr_parsing_src_attributes_crate_level {
        include!("processed_rustc_attr_parsing_src_attributes_crate_level.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/macro_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_macro_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_macro_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/semantics.rs
    pub mod rustc_attr_parsing_src_attributes_semantics {
        include!("processed_rustc_attr_parsing_src_attributes_semantics.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/confusables.rs
    pub mod rustc_attr_parsing_src_attributes_confusables {
        include!("processed_rustc_attr_parsing_src_attributes_confusables.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/lib.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_lib {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/traits.rs
    pub mod rustc_attr_parsing_src_attributes_traits {
        include!("processed_rustc_attr_parsing_src_attributes_traits.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/must_use.rs
    pub mod rustc_attr_parsing_src_attributes_must_use {
        include!("processed_rustc_attr_parsing_src_attributes_must_use.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/path.rs
    pub mod rustc_attr_parsing_src_attributes_path {
        include!("processed_rustc_attr_parsing_src_attributes_path.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/link_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_link_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_link_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/non_exhaustive.rs
    pub mod rustc_attr_parsing_src_attributes_non_exhaustive {
        include!("processed_rustc_attr_parsing_src_attributes_non_exhaustive.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/test_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_test_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_test_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/dummy.rs
    pub mod rustc_attr_parsing_src_attributes_dummy {
        include!("processed_rustc_attr_parsing_src_attributes_dummy.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/lints.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_lints {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_lints.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/deprecation.rs
    pub mod rustc_attr_parsing_src_attributes_deprecation {
        include!("processed_rustc_attr_parsing_src_attributes_deprecation.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/lint_helpers.rs
    pub mod rustc_attr_parsing_src_attributes_lint_helpers {
        include!("processed_rustc_attr_parsing_src_attributes_lint_helpers.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/proc_macro_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_proc_macro_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_proc_macro_attrs.rs");
    }
}

// 70: rustc_hashes (1 files)
pub mod included_rustc_hashes {
    // Source: ../rust/compiler/rustc_hashes/src/lib.rs
    pub mod rustc_hashes_rustc_hashes_src_lib {
        include!("processed_rustc_hashes_rustc_hashes_src_lib.rs");
    }
}

// 71: rustc_sanitizers (8 files)
pub mod included_rustc_sanitizers {
    // Source: ../rust/compiler/rustc_sanitizers/src/lib.rs
    pub mod rustc_sanitizers_rustc_sanitizers_src_lib {
        include!("processed_rustc_sanitizers_rustc_sanitizers_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/kcfi/typeid/mod.rs
    pub mod rustc_sanitizers_kcfi_typeid_mod {
        include!("processed_rustc_sanitizers_kcfi_typeid_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/mod.rs
    pub mod rustc_sanitizers_cfi_typeid_mod {
        include!("processed_rustc_sanitizers_cfi_typeid_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/mod.rs
    pub mod rustc_sanitizers_src_cfi_mod {
        include!("processed_rustc_sanitizers_src_cfi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/transform.rs
    pub mod rustc_sanitizers_typeid_itanium_cxx_abi_transform {
        include!("processed_rustc_sanitizers_typeid_itanium_cxx_abi_transform.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/mod.rs
    pub mod rustc_sanitizers_typeid_itanium_cxx_abi_mod {
        include!("processed_rustc_sanitizers_typeid_itanium_cxx_abi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/cfi/typeid/itanium_cxx_abi/encode.rs
    pub mod rustc_sanitizers_typeid_itanium_cxx_abi_encode {
        include!("processed_rustc_sanitizers_typeid_itanium_cxx_abi_encode.rs");
    }
    // Source: ../rust/compiler/rustc_sanitizers/src/kcfi/mod.rs
    pub mod rustc_sanitizers_src_kcfi_mod {
        include!("processed_rustc_sanitizers_src_kcfi_mod.rs");
    }
}

// 72: rustc_errors (22 files)
pub mod included_rustc_errors {
    // Source: ../rust/compiler/rustc_errors/src/snippet.rs
    pub mod rustc_errors_rustc_errors_src_snippet {
        include!("processed_rustc_errors_rustc_errors_src_snippet.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/json/tests.rs
    pub mod rustc_errors_src_json_tests {
        include!("processed_rustc_errors_src_json_tests.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/markdown/tests/term.rs
    pub mod rustc_errors_markdown_tests_term {
        include!("processed_rustc_errors_markdown_tests_term.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/diagnostic_impls.rs
    pub mod rustc_errors_rustc_errors_src_diagnostic_impls {
        include!("processed_rustc_errors_rustc_errors_src_diagnostic_impls.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/codes.rs
    pub mod rustc_errors_rustc_errors_src_codes {
        include!("processed_rustc_errors_rustc_errors_src_codes.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/registry.rs
    pub mod rustc_errors_rustc_errors_src_registry {
        include!("processed_rustc_errors_rustc_errors_src_registry.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/decorate_diag.rs
    pub mod rustc_errors_rustc_errors_src_decorate_diag {
        include!("processed_rustc_errors_rustc_errors_src_decorate_diag.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/markdown/tests/parse.rs
    pub mod rustc_errors_markdown_tests_parse {
        include!("processed_rustc_errors_markdown_tests_parse.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/markdown/mod.rs
    pub mod rustc_errors_src_markdown_mod {
        include!("processed_rustc_errors_src_markdown_mod.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/error.rs
    pub mod rustc_errors_rustc_errors_src_error {
        include!("processed_rustc_errors_rustc_errors_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/tests.rs
    pub mod rustc_errors_rustc_errors_src_tests {
        include!("processed_rustc_errors_rustc_errors_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/timings.rs
    pub mod rustc_errors_rustc_errors_src_timings {
        include!("processed_rustc_errors_rustc_errors_src_timings.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/diagnostic.rs
    pub mod rustc_errors_rustc_errors_src_diagnostic {
        include!("processed_rustc_errors_rustc_errors_src_diagnostic.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/annotate_snippet_emitter_writer.rs
    pub mod rustc_errors_rustc_errors_src_annotate_snippet_emitter_writer {
        include!("processed_rustc_errors_rustc_errors_src_annotate_snippet_emitter_writer.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/translation.rs
    pub mod rustc_errors_rustc_errors_src_translation {
        include!("processed_rustc_errors_rustc_errors_src_translation.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/styled_buffer.rs
    pub mod rustc_errors_rustc_errors_src_styled_buffer {
        include!("processed_rustc_errors_rustc_errors_src_styled_buffer.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/lock.rs
    pub mod rustc_errors_rustc_errors_src_lock {
        include!("processed_rustc_errors_rustc_errors_src_lock.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/emitter.rs
    pub mod rustc_errors_rustc_errors_src_emitter {
        include!("processed_rustc_errors_rustc_errors_src_emitter.rs");
    }
}

// 73: rustc_trait_selection (73 files)
pub mod included_rustc_trait_selection {
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/dyn_compatibility.rs
    pub mod rustc_trait_selection_src_traits_dyn_compatibility {
        include!("processed_rustc_trait_selection_src_traits_dyn_compatibility.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/const_evaluatable.rs
    pub mod rustc_trait_selection_src_traits_const_evaluatable {
        include!("processed_rustc_trait_selection_src_traits_const_evaluatable.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/effects.rs
    pub mod rustc_trait_selection_src_traits_effects {
        include!("processed_rustc_trait_selection_src_traits_effects.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/on_unimplemented.rs
    pub mod rustc_trait_selection_error_reporting_traits_on_unimplemented {
        include!("processed_rustc_trait_selection_error_reporting_traits_on_unimplemented.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/named_anon_conflict.rs
    pub mod rustc_trait_selection_infer_nice_region_error_named_anon_conflict {
        include!("processed_rustc_trait_selection_infer_nice_region_error_named_anon_conflict.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/util.rs
    pub mod rustc_trait_selection_infer_nice_region_error_util {
        include!("processed_rustc_trait_selection_infer_nice_region_error_util.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/need_type_info.rs
    pub mod rustc_trait_selection_error_reporting_infer_need_type_info {
        include!("processed_rustc_trait_selection_error_reporting_infer_need_type_info.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/placeholder_relation.rs
    pub mod rustc_trait_selection_infer_nice_region_error_placeholder_relation {
        include!("processed_rustc_trait_selection_infer_nice_region_error_placeholder_relation.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/outlives_bounds.rs
    pub mod rustc_trait_selection_src_traits_outlives_bounds {
        include!("processed_rustc_trait_selection_src_traits_outlives_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/fulfill.rs
    pub mod rustc_trait_selection_src_solve_fulfill {
        include!("processed_rustc_trait_selection_src_solve_fulfill.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/region.rs
    pub mod rustc_trait_selection_error_reporting_infer_region {
        include!("processed_rustc_trait_selection_error_reporting_infer_region.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/opaque_types.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_opaque_types {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/suggestions.rs
    pub mod rustc_trait_selection_error_reporting_traits_suggestions {
        include!("processed_rustc_trait_selection_error_reporting_traits_suggestions.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/mod.rs
    pub mod rustc_trait_selection_src_traits_mod {
        include!("processed_rustc_trait_selection_src_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/custom.rs
    pub mod rustc_trait_selection_query_type_op_custom {
        include!("processed_rustc_trait_selection_query_type_op_custom.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/method_autoderef.rs
    pub mod rustc_trait_selection_traits_query_method_autoderef {
        include!("processed_rustc_trait_selection_traits_query_method_autoderef.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/mod.rs
    pub mod rustc_trait_selection_infer_nice_region_error_mod {
        include!("processed_rustc_trait_selection_infer_nice_region_error_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/wf.rs
    pub mod rustc_trait_selection_src_traits_wf {
        include!("processed_rustc_trait_selection_src_traits_wf.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/infer.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_infer {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_infer.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/dropck_outlives.rs
    pub mod rustc_trait_selection_traits_query_dropck_outlives {
        include!("processed_rustc_trait_selection_traits_query_dropck_outlives.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/delegate.rs
    pub mod rustc_trait_selection_src_solve_delegate {
        include!("processed_rustc_trait_selection_src_solve_delegate.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/errors/note_and_explain.rs
    pub mod rustc_trait_selection_src_errors_note_and_explain {
        include!("processed_rustc_trait_selection_src_errors_note_and_explain.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/implied_outlives_bounds.rs
    pub mod rustc_trait_selection_query_type_op_implied_outlives_bounds {
        include!("processed_rustc_trait_selection_query_type_op_implied_outlives_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/ambiguity.rs
    pub mod rustc_trait_selection_error_reporting_traits_ambiguity {
        include!("processed_rustc_trait_selection_error_reporting_traits_ambiguity.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/coherence.rs
    pub mod rustc_trait_selection_src_traits_coherence {
        include!("processed_rustc_trait_selection_src_traits_coherence.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/select/confirmation.rs
    pub mod rustc_trait_selection_traits_select_confirmation {
        include!("processed_rustc_trait_selection_traits_select_confirmation.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/mismatched_static_lifetime.rs
    pub mod rustc_trait_selection_infer_nice_region_error_mismatched_static_lifetime {
        include!("processed_rustc_trait_selection_infer_nice_region_error_mismatched_static_lifetime.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/normalize.rs
    pub mod rustc_trait_selection_src_solve_normalize {
        include!("processed_rustc_trait_selection_src_solve_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/note_and_explain.rs
    pub mod rustc_trait_selection_error_reporting_infer_note_and_explain {
        include!("processed_rustc_trait_selection_error_reporting_infer_note_and_explain.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/lib.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_lib {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/inspect/analyse.rs
    pub mod rustc_trait_selection_solve_inspect_analyse {
        include!("processed_rustc_trait_selection_solve_inspect_analyse.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/structural_normalize.rs
    pub mod rustc_trait_selection_src_traits_structural_normalize {
        include!("processed_rustc_trait_selection_src_traits_structural_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/select/_match.rs
    pub mod rustc_trait_selection_traits_select__match {
        include!("processed_rustc_trait_selection_traits_select__match.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/on_unimplemented_condition.rs
    pub mod rustc_trait_selection_error_reporting_traits_on_unimplemented_condition {
        include!("processed_rustc_trait_selection_error_reporting_traits_on_unimplemented_condition.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/fulfillment_errors.rs
    pub mod rustc_trait_selection_error_reporting_traits_fulfillment_errors {
        include!("processed_rustc_trait_selection_error_reporting_traits_fulfillment_errors.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_solve {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_solve.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/fulfill/derive_errors.rs
    pub mod rustc_trait_selection_solve_fulfill_derive_errors {
        include!("processed_rustc_trait_selection_solve_fulfill_derive_errors.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/mod.rs
    pub mod rustc_trait_selection_query_type_op_mod {
        include!("processed_rustc_trait_selection_query_type_op_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/find_anon_type.rs
    pub mod rustc_trait_selection_infer_nice_region_error_find_anon_type {
        include!("processed_rustc_trait_selection_infer_nice_region_error_find_anon_type.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/fulfill.rs
    pub mod rustc_trait_selection_src_traits_fulfill {
        include!("processed_rustc_trait_selection_src_traits_fulfill.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/specialize/specialization_graph.rs
    pub mod rustc_trait_selection_traits_specialize_specialization_graph {
        include!("processed_rustc_trait_selection_traits_specialize_specialization_graph.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/on_unimplemented_format.rs
    pub mod rustc_trait_selection_error_reporting_traits_on_unimplemented_format {
        include!("processed_rustc_trait_selection_error_reporting_traits_on_unimplemented_format.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/mod.rs
    pub mod rustc_trait_selection_error_reporting_traits_mod {
        include!("processed_rustc_trait_selection_error_reporting_traits_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/outlives.rs
    pub mod rustc_trait_selection_query_type_op_outlives {
        include!("processed_rustc_trait_selection_query_type_op_outlives.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/mod.rs
    pub mod rustc_trait_selection_src_error_reporting_mod {
        include!("processed_rustc_trait_selection_src_error_reporting_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/mod.rs
    pub mod rustc_trait_selection_traits_query_mod {
        include!("processed_rustc_trait_selection_traits_query_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/inspect.rs
    pub mod rustc_trait_selection_src_solve_inspect {
        include!("processed_rustc_trait_selection_src_solve_inspect.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/regions.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_regions {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_regions.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/select/mod.rs
    pub mod rustc_trait_selection_traits_select_mod {
        include!("processed_rustc_trait_selection_traits_select_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/trait_impl_difference.rs
    pub mod rustc_trait_selection_infer_nice_region_error_trait_impl_difference {
        include!("processed_rustc_trait_selection_infer_nice_region_error_trait_impl_difference.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/misc.rs
    pub mod rustc_trait_selection_src_traits_misc {
        include!("processed_rustc_trait_selection_src_traits_misc.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/static_impl_trait.rs
    pub mod rustc_trait_selection_infer_nice_region_error_static_impl_trait {
        include!("processed_rustc_trait_selection_infer_nice_region_error_static_impl_trait.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/normalize.rs
    pub mod rustc_trait_selection_traits_query_normalize {
        include!("processed_rustc_trait_selection_traits_query_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/project.rs
    pub mod rustc_trait_selection_src_traits_project {
        include!("processed_rustc_trait_selection_src_traits_project.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/call_kind.rs
    pub mod rustc_trait_selection_error_reporting_traits_call_kind {
        include!("processed_rustc_trait_selection_error_reporting_traits_call_kind.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/auto_trait.rs
    pub mod rustc_trait_selection_src_traits_auto_trait {
        include!("processed_rustc_trait_selection_src_traits_auto_trait.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/mod.rs
    pub mod rustc_trait_selection_error_reporting_infer_mod {
        include!("processed_rustc_trait_selection_error_reporting_infer_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/placeholder_error.rs
    pub mod rustc_trait_selection_infer_nice_region_error_placeholder_error {
        include!("processed_rustc_trait_selection_infer_nice_region_error_placeholder_error.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/ascribe_user_type.rs
    pub mod rustc_trait_selection_query_type_op_ascribe_user_type {
        include!("processed_rustc_trait_selection_query_type_op_ascribe_user_type.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/util.rs
    pub mod rustc_trait_selection_src_traits_util {
        include!("processed_rustc_trait_selection_src_traits_util.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/errors.rs
    pub mod rustc_trait_selection_rustc_trait_selection_src_errors {
        include!("processed_rustc_trait_selection_rustc_trait_selection_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/evaluate_obligation.rs
    pub mod rustc_trait_selection_traits_query_evaluate_obligation {
        include!("processed_rustc_trait_selection_traits_query_evaluate_obligation.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/solve/select.rs
    pub mod rustc_trait_selection_src_solve_select {
        include!("processed_rustc_trait_selection_src_solve_select.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/prove_predicate.rs
    pub mod rustc_trait_selection_query_type_op_prove_predicate {
        include!("processed_rustc_trait_selection_query_type_op_prove_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/engine.rs
    pub mod rustc_trait_selection_src_traits_engine {
        include!("processed_rustc_trait_selection_src_traits_engine.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/nice_region_error/different_lifetimes.rs
    pub mod rustc_trait_selection_infer_nice_region_error_different_lifetimes {
        include!("processed_rustc_trait_selection_infer_nice_region_error_different_lifetimes.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/normalize.rs
    pub mod rustc_trait_selection_src_traits_normalize {
        include!("processed_rustc_trait_selection_src_traits_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/specialize/mod.rs
    pub mod rustc_trait_selection_traits_specialize_mod {
        include!("processed_rustc_trait_selection_traits_specialize_mod.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/query/type_op/normalize.rs
    pub mod rustc_trait_selection_query_type_op_normalize {
        include!("processed_rustc_trait_selection_query_type_op_normalize.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/select/candidate_assembly.rs
    pub mod rustc_trait_selection_traits_select_candidate_assembly {
        include!("processed_rustc_trait_selection_traits_select_candidate_assembly.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/traits/vtable.rs
    pub mod rustc_trait_selection_src_traits_vtable {
        include!("processed_rustc_trait_selection_src_traits_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/infer/suggest.rs
    pub mod rustc_trait_selection_error_reporting_infer_suggest {
        include!("processed_rustc_trait_selection_error_reporting_infer_suggest.rs");
    }
    // Source: ../rust/compiler/rustc_trait_selection/src/error_reporting/traits/overflow.rs
    pub mod rustc_trait_selection_error_reporting_traits_overflow {
        include!("processed_rustc_trait_selection_error_reporting_traits_overflow.rs");
    }
}

// 74: rustc_interface (10 files)
pub mod included_rustc_interface {
    // Source: ../rust/compiler/rustc_interface/src/passes.rs
    pub mod rustc_interface_rustc_interface_src_passes {
        include!("processed_rustc_interface_rustc_interface_src_passes.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/queries.rs
    pub mod rustc_interface_rustc_interface_src_queries {
        include!("processed_rustc_interface_rustc_interface_src_queries.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/errors.rs
    pub mod rustc_interface_rustc_interface_src_errors {
        include!("processed_rustc_interface_rustc_interface_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/limits.rs
    pub mod rustc_interface_rustc_interface_src_limits {
        include!("processed_rustc_interface_rustc_interface_src_limits.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/util.rs
    pub mod rustc_interface_rustc_interface_src_util {
        include!("processed_rustc_interface_rustc_interface_src_util.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/callbacks.rs
    pub mod rustc_interface_rustc_interface_src_callbacks {
        include!("processed_rustc_interface_rustc_interface_src_callbacks.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/proc_macro_decls.rs
    pub mod rustc_interface_rustc_interface_src_proc_macro_decls {
        include!("processed_rustc_interface_rustc_interface_src_proc_macro_decls.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/interface.rs
    pub mod rustc_interface_rustc_interface_src_interface {
        include!("processed_rustc_interface_rustc_interface_src_interface.rs");
    }
    // Source: ../rust/compiler/rustc_interface/src/tests.rs
    pub mod rustc_interface_rustc_interface_src_tests {
        include!("processed_rustc_interface_rustc_interface_src_tests.rs");
    }
}

