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
    // Source: ../rust/compiler/rustc_data_structures/src/sync/parallel.rs
    pub mod rustc_data_structures_src_sync_parallel {
        include!("processed_rustc_data_structures_src_sync_parallel.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/tagged_ptr/tests.rs
    pub mod rustc_data_structures_src_tagged_ptr_tests {
        include!("processed_rustc_data_structures_src_tagged_ptr_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/graphviz.rs
    pub mod rustc_data_structures_src_obligation_forest_graphviz {
        include!("processed_rustc_data_structures_src_obligation_forest_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/owned_slice/tests.rs
    pub mod rustc_data_structures_src_owned_slice_tests {
        include!("processed_rustc_data_structures_src_owned_slice_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/jobserver.rs
    pub mod rustc_data_structures_rustc_data_structures_src_jobserver {
        include!("processed_rustc_data_structures_rustc_data_structures_src_jobserver.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/svh.rs
    pub mod rustc_data_structures_rustc_data_structures_src_svh {
        include!("processed_rustc_data_structures_rustc_data_structures_src_svh.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/union_find/tests.rs
    pub mod rustc_data_structures_src_union_find_tests {
        include!("processed_rustc_data_structures_src_union_find_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/linked_graph/tests.rs
    pub mod rustc_data_structures_graph_linked_graph_tests {
        include!("processed_rustc_data_structures_graph_linked_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/tests.rs
    pub mod rustc_data_structures_src_sorted_map_tests {
        include!("processed_rustc_data_structures_src_sorted_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/dominators/tests.rs
    pub mod rustc_data_structures_graph_dominators_tests {
        include!("processed_rustc_data_structures_graph_dominators_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/profiling/tests.rs
    pub mod rustc_data_structures_src_profiling_tests {
        include!("processed_rustc_data_structures_src_profiling_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fingerprint/tests.rs
    pub mod rustc_data_structures_src_fingerprint_tests {
        include!("processed_rustc_data_structures_src_fingerprint_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/aligned.rs
    pub mod rustc_data_structures_rustc_data_structures_src_aligned {
        include!("processed_rustc_data_structures_rustc_data_structures_src_aligned.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/intern/tests.rs
    pub mod rustc_data_structures_src_intern_tests {
        include!("processed_rustc_data_structures_src_intern_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/transitive_relation/tests.rs
    pub mod rustc_data_structures_src_transitive_relation_tests {
        include!("processed_rustc_data_structures_src_transitive_relation_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/linux.rs
    pub mod rustc_data_structures_src_flock_linux {
        include!("processed_rustc_data_structures_src_flock_linux.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stable_hasher/tests.rs
    pub mod rustc_data_structures_src_stable_hasher_tests {
        include!("processed_rustc_data_structures_src_stable_hasher_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/temp_dir.rs
    pub mod rustc_data_structures_rustc_data_structures_src_temp_dir {
        include!("processed_rustc_data_structures_rustc_data_structures_src_temp_dir.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thousands/tests.rs
    pub mod rustc_data_structures_src_thousands_tests {
        include!("processed_rustc_data_structures_src_thousands_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/packed.rs
    pub mod rustc_data_structures_rustc_data_structures_src_packed {
        include!("processed_rustc_data_structures_rustc_data_structures_src_packed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/base_n/tests.rs
    pub mod rustc_data_structures_src_base_n_tests {
        include!("processed_rustc_data_structures_src_base_n_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/mod.rs
    pub mod rustc_data_structures_src_sso_mod {
        include!("processed_rustc_data_structures_src_sso_mod.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/atomic_ref.rs
    pub mod rustc_data_structures_rustc_data_structures_src_atomic_ref {
        include!("processed_rustc_data_structures_rustc_data_structures_src_atomic_ref.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/lib.rs
    pub mod rustc_data_structures_rustc_data_structures_src_lib {
        include!("processed_rustc_data_structures_rustc_data_structures_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/small_c_str/tests.rs
    pub mod rustc_data_structures_src_small_c_str_tests {
        include!("processed_rustc_data_structures_src_small_c_str_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reversed.rs
    pub mod rustc_data_structures_src_graph_reversed {
        include!("processed_rustc_data_structures_src_graph_reversed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/index_map.rs
    pub mod rustc_data_structures_src_sorted_map_index_map {
        include!("processed_rustc_data_structures_src_sorted_map_index_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/marker.rs
    pub mod rustc_data_structures_rustc_data_structures_src_marker {
        include!("processed_rustc_data_structures_rustc_data_structures_src_marker.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stack.rs
    pub mod rustc_data_structures_rustc_data_structures_src_stack {
        include!("processed_rustc_data_structures_rustc_data_structures_src_stack.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/frozen.rs
    pub mod rustc_data_structures_rustc_data_structures_src_frozen {
        include!("processed_rustc_data_structures_rustc_data_structures_src_frozen.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unsupported.rs
    pub mod rustc_data_structures_src_flock_unsupported {
        include!("processed_rustc_data_structures_src_flock_unsupported.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/iterate/tests.rs
    pub mod rustc_data_structures_graph_iterate_tests {
        include!("processed_rustc_data_structures_graph_iterate_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/worker_local.rs
    pub mod rustc_data_structures_src_sync_worker_local {
        include!("processed_rustc_data_structures_src_sync_worker_local.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/memmap.rs
    pub mod rustc_data_structures_rustc_data_structures_src_memmap {
        include!("processed_rustc_data_structures_rustc_data_structures_src_memmap.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/snapshot_map/tests.rs
    pub mod rustc_data_structures_src_snapshot_map_tests {
        include!("processed_rustc_data_structures_src_snapshot_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/vec_cache/tests.rs
    pub mod rustc_data_structures_src_vec_cache_tests {
        include!("processed_rustc_data_structures_src_vec_cache_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fx.rs
    pub mod rustc_data_structures_rustc_data_structures_src_fx {
        include!("processed_rustc_data_structures_rustc_data_structures_src_fx.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/steal.rs
    pub mod rustc_data_structures_rustc_data_structures_src_steal {
        include!("processed_rustc_data_structures_rustc_data_structures_src_steal.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/vec.rs
    pub mod rustc_data_structures_src_sync_vec {
        include!("processed_rustc_data_structures_src_sync_vec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/windows.rs
    pub mod rustc_data_structures_src_flock_windows {
        include!("processed_rustc_data_structures_src_flock_windows.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unhash.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unhash {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unhash.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/tests.rs
    pub mod rustc_data_structures_src_graph_tests {
        include!("processed_rustc_data_structures_src_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/set.rs
    pub mod rustc_data_structures_src_sso_set {
        include!("processed_rustc_data_structures_src_sso_set.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reference.rs
    pub mod rustc_data_structures_src_graph_reference {
        include!("processed_rustc_data_structures_src_graph_reference.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sync {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sync.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/binary_search_util/tests.rs
    pub mod rustc_data_structures_src_binary_search_util_tests {
        include!("processed_rustc_data_structures_src_binary_search_util_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/map.rs
    pub mod rustc_data_structures_src_sso_map {
        include!("processed_rustc_data_structures_src_sso_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/freeze.rs
    pub mod rustc_data_structures_src_sync_freeze {
        include!("processed_rustc_data_structures_src_sync_freeze.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flock {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thinvec.rs
    pub mod rustc_data_structures_rustc_data_structures_src_thinvec {
        include!("processed_rustc_data_structures_rustc_data_structures_src_thinvec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unix.rs
    pub mod rustc_data_structures_src_flock_unix {
        include!("processed_rustc_data_structures_src_flock_unix.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/tests.rs
    pub mod rustc_data_structures_src_obligation_forest_tests {
        include!("processed_rustc_data_structures_src_obligation_forest_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/work_queue.rs
    pub mod rustc_data_structures_rustc_data_structures_src_work_queue {
        include!("processed_rustc_data_structures_rustc_data_structures_src_work_queue.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flat_map_in_place.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flat_map_in_place {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flat_map_in_place.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sharded.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sharded {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sharded.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unord.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unord {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unord.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/lock.rs
    pub mod rustc_data_structures_src_sync_lock {
        include!("processed_rustc_data_structures_src_sync_lock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/vec_graph/tests.rs
    pub mod rustc_data_structures_graph_vec_graph_tests {
        include!("processed_rustc_data_structures_graph_vec_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/scc/tests.rs
    pub mod rustc_data_structures_graph_scc_tests {
        include!("processed_rustc_data_structures_graph_scc_tests.rs");
    }
}

// 3: rustc_index (9 files)
pub mod included_rustc_index {
    // Source: ../rust/compiler/rustc_index/src/slice.rs
    pub mod rustc_index_rustc_index_src_slice {
        include!("processed_rustc_index_rustc_index_src_slice.rs");
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
    // Source: ../rust/compiler/rustc_span/src/profiling.rs
    pub mod rustc_span_rustc_span_src_profiling {
        include!("processed_rustc_span_rustc_span_src_profiling.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/def_id.rs
    pub mod rustc_span_rustc_span_src_def_id {
        include!("processed_rustc_span_rustc_span_src_def_id.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/caching_source_map_view.rs
    pub mod rustc_span_rustc_span_src_caching_source_map_view {
        include!("processed_rustc_span_rustc_span_src_caching_source_map_view.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/tests.rs
    pub mod rustc_span_rustc_span_src_tests {
        include!("processed_rustc_span_rustc_span_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edit_distance/tests.rs
    pub mod rustc_span_src_edit_distance_tests {
        include!("processed_rustc_span_src_edit_distance_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/fatal_error.rs
    pub mod rustc_span_rustc_span_src_fatal_error {
        include!("processed_rustc_span_rustc_span_src_fatal_error.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/symbol/tests.rs
    pub mod rustc_span_src_symbol_tests {
        include!("processed_rustc_span_src_symbol_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/span_encoding.rs
    pub mod rustc_span_rustc_span_src_span_encoding {
        include!("processed_rustc_span_rustc_span_src_span_encoding.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edition.rs
    pub mod rustc_span_rustc_span_src_edition {
        include!("processed_rustc_span_rustc_span_src_edition.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/source_map/tests.rs
    pub mod rustc_span_src_source_map_tests {
        include!("processed_rustc_span_src_source_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/hygiene.rs
    pub mod rustc_span_rustc_span_src_hygiene {
        include!("processed_rustc_span_rustc_span_src_hygiene.rs");
    }
}

// 5: rustc_ast_passes (4 files)
pub mod included_rustc_ast_passes {
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
    // Source: ../rust/compiler/rustc_ast_passes/src/errors.rs
    pub mod rustc_ast_passes_rustc_ast_passes_src_errors {
        include!("processed_rustc_ast_passes_rustc_ast_passes_src_errors.rs");
    }
}

// 6: rustc_type_ir_macros (1 files)
pub mod included_rustc_type_ir_macros {
    // Source: ../rust/compiler/rustc_type_ir_macros/src/lib.rs
    pub mod rustc_type_ir_macros_rustc_type_ir_macros_src_lib {
        include!("processed_rustc_type_ir_macros_rustc_type_ir_macros_src_lib.rs");
    }
}

// 7: rustc_hashes (1 files)
pub mod included_rustc_hashes {
    // Source: ../rust/compiler/rustc_hashes/src/lib.rs
    pub mod rustc_hashes_rustc_hashes_src_lib {
        include!("processed_rustc_hashes_rustc_hashes_src_lib.rs");
    }
}

// 8: rustc_lint_defs (2 files)
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

// 9: rustc_error_codes (1 files)
pub mod included_rustc_error_codes {
    // Source: ../rust/compiler/rustc_error_codes/src/lib.rs
    pub mod rustc_error_codes_rustc_error_codes_src_lib {
        include!("processed_rustc_error_codes_rustc_error_codes_src_lib.rs");
    }
}

// 10: rustc_attr_parsing (38 files)
pub mod included_rustc_attr_parsing {
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/util.rs
    pub mod rustc_attr_parsing_src_attributes_util {
        include!("processed_rustc_attr_parsing_src_attributes_util.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/loop_match.rs
    pub mod rustc_attr_parsing_src_attributes_loop_match {
        include!("processed_rustc_attr_parsing_src_attributes_loop_match.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/codegen_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_codegen_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_codegen_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/traits.rs
    pub mod rustc_attr_parsing_src_attributes_traits {
        include!("processed_rustc_attr_parsing_src_attributes_traits.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/transparency.rs
    pub mod rustc_attr_parsing_src_attributes_transparency {
        include!("processed_rustc_attr_parsing_src_attributes_transparency.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/confusables.rs
    pub mod rustc_attr_parsing_src_attributes_confusables {
        include!("processed_rustc_attr_parsing_src_attributes_confusables.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/deprecation.rs
    pub mod rustc_attr_parsing_src_attributes_deprecation {
        include!("processed_rustc_attr_parsing_src_attributes_deprecation.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/test_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_test_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_test_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/non_exhaustive.rs
    pub mod rustc_attr_parsing_src_attributes_non_exhaustive {
        include!("processed_rustc_attr_parsing_src_attributes_non_exhaustive.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/mod.rs
    pub mod rustc_attr_parsing_src_attributes_mod {
        include!("processed_rustc_attr_parsing_src_attributes_mod.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/repr.rs
    pub mod rustc_attr_parsing_src_attributes_repr {
        include!("processed_rustc_attr_parsing_src_attributes_repr.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/interface.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_interface {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_interface.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/must_use.rs
    pub mod rustc_attr_parsing_src_attributes_must_use {
        include!("processed_rustc_attr_parsing_src_attributes_must_use.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/context.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_context {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_context.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/inline.rs
    pub mod rustc_attr_parsing_src_attributes_inline {
        include!("processed_rustc_attr_parsing_src_attributes_inline.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/semantics.rs
    pub mod rustc_attr_parsing_src_attributes_semantics {
        include!("processed_rustc_attr_parsing_src_attributes_semantics.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/proc_macro_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_proc_macro_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_proc_macro_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/rustc_internal.rs
    pub mod rustc_attr_parsing_src_attributes_rustc_internal {
        include!("processed_rustc_attr_parsing_src_attributes_rustc_internal.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/path.rs
    pub mod rustc_attr_parsing_src_attributes_path {
        include!("processed_rustc_attr_parsing_src_attributes_path.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/lint_helpers.rs
    pub mod rustc_attr_parsing_src_attributes_lint_helpers {
        include!("processed_rustc_attr_parsing_src_attributes_lint_helpers.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/stability.rs
    pub mod rustc_attr_parsing_src_attributes_stability {
        include!("processed_rustc_attr_parsing_src_attributes_stability.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/macro_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_macro_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_macro_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/dummy.rs
    pub mod rustc_attr_parsing_src_attributes_dummy {
        include!("processed_rustc_attr_parsing_src_attributes_dummy.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/allow_unstable.rs
    pub mod rustc_attr_parsing_src_attributes_allow_unstable {
        include!("processed_rustc_attr_parsing_src_attributes_allow_unstable.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/target_checking.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_target_checking {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_target_checking.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/cfg.rs
    pub mod rustc_attr_parsing_src_attributes_cfg {
        include!("processed_rustc_attr_parsing_src_attributes_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/lints.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_lints {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_lints.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/validate_attr.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_validate_attr {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_validate_attr.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/session_diagnostics.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_session_diagnostics {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_session_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/crate_level.rs
    pub mod rustc_attr_parsing_src_attributes_crate_level {
        include!("processed_rustc_attr_parsing_src_attributes_crate_level.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/prelude.rs
    pub mod rustc_attr_parsing_src_attributes_prelude {
        include!("processed_rustc_attr_parsing_src_attributes_prelude.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/prototype.rs
    pub mod rustc_attr_parsing_src_attributes_prototype {
        include!("processed_rustc_attr_parsing_src_attributes_prototype.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/body.rs
    pub mod rustc_attr_parsing_src_attributes_body {
        include!("processed_rustc_attr_parsing_src_attributes_body.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/cfg_old.rs
    pub mod rustc_attr_parsing_src_attributes_cfg_old {
        include!("processed_rustc_attr_parsing_src_attributes_cfg_old.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/no_implicit_prelude.rs
    pub mod rustc_attr_parsing_src_attributes_no_implicit_prelude {
        include!("processed_rustc_attr_parsing_src_attributes_no_implicit_prelude.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/lib.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_lib {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/parser.rs
    pub mod rustc_attr_parsing_rustc_attr_parsing_src_parser {
        include!("processed_rustc_attr_parsing_rustc_attr_parsing_src_parser.rs");
    }
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/link_attrs.rs
    pub mod rustc_attr_parsing_src_attributes_link_attrs {
        include!("processed_rustc_attr_parsing_src_attributes_link_attrs.rs");
    }
}

// 11: rustc_passes (20 files)
pub mod included_rustc_passes {
    // Source: ../rust/compiler/rustc_passes/src/layout_test.rs
    pub mod rustc_passes_rustc_passes_src_layout_test {
        include!("processed_rustc_passes_rustc_passes_src_layout_test.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/errors.rs
    pub mod rustc_passes_rustc_passes_src_errors {
        include!("processed_rustc_passes_rustc_passes_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/input_stats.rs
    pub mod rustc_passes_rustc_passes_src_input_stats {
        include!("processed_rustc_passes_rustc_passes_src_input_stats.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/lang_items.rs
    pub mod rustc_passes_rustc_passes_src_lang_items {
        include!("processed_rustc_passes_rustc_passes_src_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/check_attr.rs
    pub mod rustc_passes_rustc_passes_src_check_attr {
        include!("processed_rustc_passes_rustc_passes_src_check_attr.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/check_export.rs
    pub mod rustc_passes_rustc_passes_src_check_export {
        include!("processed_rustc_passes_rustc_passes_src_check_export.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/stability.rs
    pub mod rustc_passes_rustc_passes_src_stability {
        include!("processed_rustc_passes_rustc_passes_src_stability.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/debugger_visualizer.rs
    pub mod rustc_passes_rustc_passes_src_debugger_visualizer {
        include!("processed_rustc_passes_rustc_passes_src_debugger_visualizer.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/lib.rs
    pub mod rustc_passes_rustc_passes_src_lib {
        include!("processed_rustc_passes_rustc_passes_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/hir_id_validator.rs
    pub mod rustc_passes_rustc_passes_src_hir_id_validator {
        include!("processed_rustc_passes_rustc_passes_src_hir_id_validator.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/diagnostic_items.rs
    pub mod rustc_passes_rustc_passes_src_diagnostic_items {
        include!("processed_rustc_passes_rustc_passes_src_diagnostic_items.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/weak_lang_items.rs
    pub mod rustc_passes_rustc_passes_src_weak_lang_items {
        include!("processed_rustc_passes_rustc_passes_src_weak_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/entry.rs
    pub mod rustc_passes_rustc_passes_src_entry {
        include!("processed_rustc_passes_rustc_passes_src_entry.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/reachable.rs
    pub mod rustc_passes_rustc_passes_src_reachable {
        include!("processed_rustc_passes_rustc_passes_src_reachable.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/liveness/rwu_table.rs
    pub mod rustc_passes_src_liveness_rwu_table {
        include!("processed_rustc_passes_src_liveness_rwu_table.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/abi_test.rs
    pub mod rustc_passes_rustc_passes_src_abi_test {
        include!("processed_rustc_passes_rustc_passes_src_abi_test.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/dead.rs
    pub mod rustc_passes_rustc_passes_src_dead {
        include!("processed_rustc_passes_rustc_passes_src_dead.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/upvars.rs
    pub mod rustc_passes_rustc_passes_src_upvars {
        include!("processed_rustc_passes_rustc_passes_src_upvars.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/liveness.rs
    pub mod rustc_passes_rustc_passes_src_liveness {
        include!("processed_rustc_passes_rustc_passes_src_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_passes/src/lib_features.rs
    pub mod rustc_passes_rustc_passes_src_lib_features {
        include!("processed_rustc_passes_rustc_passes_src_lib_features.rs");
    }
}

// 12: rustc_query_impl (3 files)
pub mod included_rustc_query_impl {
    // Source: ../rust/compiler/rustc_query_impl/src/profiling_support.rs
    pub mod rustc_query_impl_rustc_query_impl_src_profiling_support {
        include!("processed_rustc_query_impl_rustc_query_impl_src_profiling_support.rs");
    }
    // Source: ../rust/compiler/rustc_query_impl/src/lib.rs
    pub mod rustc_query_impl_rustc_query_impl_src_lib {
        include!("processed_rustc_query_impl_rustc_query_impl_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_query_impl/src/plumbing.rs
    pub mod rustc_query_impl_rustc_query_impl_src_plumbing {
        include!("processed_rustc_query_impl_rustc_query_impl_src_plumbing.rs");
    }
}

// 13: rustc_builtin_macros (49 files)
pub mod included_rustc_builtin_macros {
    // Source: ../rust/compiler/rustc_builtin_macros/src/test_harness.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_test_harness {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_test_harness.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/ord.rs
    pub mod rustc_builtin_macros_deriving_cmp_ord {
        include!("processed_rustc_builtin_macros_deriving_cmp_ord.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/bounds.rs
    pub mod rustc_builtin_macros_src_deriving_bounds {
        include!("processed_rustc_builtin_macros_src_deriving_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/autodiff.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_autodiff {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_autodiff.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/env.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_env {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_env.rs");
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
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/clone.rs
    pub mod rustc_builtin_macros_src_deriving_clone {
        include!("processed_rustc_builtin_macros_src_deriving_clone.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/format_foreign/shell/tests.rs
    pub mod rustc_builtin_macros_format_foreign_shell_tests {
        include!("processed_rustc_builtin_macros_format_foreign_shell_tests.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/hash.rs
    pub mod rustc_builtin_macros_src_deriving_hash {
        include!("processed_rustc_builtin_macros_src_deriving_hash.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/from.rs
    pub mod rustc_builtin_macros_src_deriving_from {
        include!("processed_rustc_builtin_macros_src_deriving_from.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/debug.rs
    pub mod rustc_builtin_macros_src_deriving_debug {
        include!("processed_rustc_builtin_macros_src_deriving_debug.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/assert.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_assert {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_assert.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/lib.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_lib {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/standard_library_imports.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_standard_library_imports {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_standard_library_imports.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/generic/ty.rs
    pub mod rustc_builtin_macros_deriving_generic_ty {
        include!("processed_rustc_builtin_macros_deriving_generic_ty.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/mod.rs
    pub mod rustc_builtin_macros_src_deriving_mod {
        include!("processed_rustc_builtin_macros_src_deriving_mod.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg_accessible.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg_accessible {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg_accessible.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/contracts.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_contracts {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_contracts.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/alloc_error_handler.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_alloc_error_handler {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_alloc_error_handler.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/proc_macro_harness.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_proc_macro_harness {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_proc_macro_harness.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/trace_macros.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_trace_macros {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_trace_macros.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/partial_eq.rs
    pub mod rustc_builtin_macros_deriving_cmp_partial_eq {
        include!("processed_rustc_builtin_macros_deriving_cmp_partial_eq.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/iter.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_iter {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_iter.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/errors.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_errors {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/compile_error.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_compile_error {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_compile_error.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/eq.rs
    pub mod rustc_builtin_macros_deriving_cmp_eq {
        include!("processed_rustc_builtin_macros_deriving_cmp_eq.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/generic/mod.rs
    pub mod rustc_builtin_macros_deriving_generic_mod {
        include!("processed_rustc_builtin_macros_deriving_generic_mod.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/util.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_util {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_util.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/asm.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_asm {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_asm.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/edition_panic.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_edition_panic {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_edition_panic.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/source_util.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_source_util {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_source_util.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/assert/context.rs
    pub mod rustc_builtin_macros_src_assert_context {
        include!("processed_rustc_builtin_macros_src_assert_context.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/concat.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_concat {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_concat.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/log_syntax.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_log_syntax {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_log_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/cfg_select.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_cfg_select {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_cfg_select.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/format.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_format {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_format.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/derive.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_derive {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_derive.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/pattern_type.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_pattern_type {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_pattern_type.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/format_foreign/printf/tests.rs
    pub mod rustc_builtin_macros_format_foreign_printf_tests {
        include!("processed_rustc_builtin_macros_format_foreign_printf_tests.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/concat_bytes.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_concat_bytes {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_concat_bytes.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/coerce_pointee.rs
    pub mod rustc_builtin_macros_src_deriving_coerce_pointee {
        include!("processed_rustc_builtin_macros_src_deriving_coerce_pointee.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/cmp/partial_ord.rs
    pub mod rustc_builtin_macros_deriving_cmp_partial_ord {
        include!("processed_rustc_builtin_macros_deriving_cmp_partial_ord.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/deriving/default.rs
    pub mod rustc_builtin_macros_src_deriving_default {
        include!("processed_rustc_builtin_macros_src_deriving_default.rs");
    }
    // Source: ../rust/compiler/rustc_builtin_macros/src/global_allocator.rs
    pub mod rustc_builtin_macros_rustc_builtin_macros_src_global_allocator {
        include!("processed_rustc_builtin_macros_rustc_builtin_macros_src_global_allocator.rs");
    }
}

// 14: rustc_codegen_cranelift (74 files)
pub mod included_rustc_codegen_cranelift {
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/shared_utils.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_shared_utils {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_shared_utils.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/base.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_base {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_base.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/codegen_i128.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_i128 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_i128.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/float-minmax-pass.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_float_minmax_pass {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_float-minmax-pass.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/gen_block_iterate.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_gen_block_iterate {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_gen_block_iterate.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/value_and_place.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_value_and_place {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_value_and_place.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/bench.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_bench {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_bench.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/emit.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_emit {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_emit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/cast.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_cast {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_cast.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/returning.rs
    pub mod rustc_codegen_cranelift_src_abi_returning {
        include!("processed_rustc_codegen_cranelift_src_abi_returning.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/compiler_builtins.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_compiler_builtins {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_compiler_builtins.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/vtable.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_vtable {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_vtable.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/aot.rs
    pub mod rustc_codegen_cranelift_src_driver_aot {
        include!("processed_rustc_codegen_cranelift_src_driver_aot.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/mod.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_mod {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/mod.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_mod {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/main_shim.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_main_shim {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_main_shim.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/global_asm.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_global_asm {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_global_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/jit.rs
    pub mod rustc_codegen_cranelift_src_driver_jit {
        include!("processed_rustc_codegen_cranelift_src_driver_jit.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/linkage.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_linkage {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_linkage.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/raw-dylib.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_raw_dylib {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_raw-dylib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/inline_asm.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_inline_asm {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_inline_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/config.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_config {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_config.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/toolchain.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_toolchain {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_toolchain.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/track-caller-attribute.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_track_caller_attribute {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_track-caller-attribute.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/codegen_f16_f128.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_f16_f128 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_codegen_f16_f128.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm_aarch64.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm_aarch64 {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm_aarch64.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/issue-72793.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue_72793 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue-72793.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/pretty_clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_pretty_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_pretty_clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/concurrency_limiter.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_concurrency_limiter {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_concurrency_limiter.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/unwind_module.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_unwind_module {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_unwind_module.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/tests.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_tests {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/lib.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_lib {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/discriminant.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_discriminant {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_discriminant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/abi_cafe.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_abi_cafe {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_abi_cafe.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/pass_mode.rs
    pub mod rustc_codegen_cranelift_src_abi_pass_mode {
        include!("processed_rustc_codegen_cranelift_src_abi_pass_mode.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/build_backend.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_backend {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_backend.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/neon.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_neon {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_neon.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/object.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_object {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_object.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/driver/mod.rs
    pub mod rustc_codegen_cranelift_src_driver_mod {
        include!("processed_rustc_codegen_cranelift_src_driver_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/std_example.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_std_example {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_std_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/constant.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_constant {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_constant.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/optimize/peephole.rs
    pub mod rustc_codegen_cranelift_src_optimize_peephole {
        include!("processed_rustc_codegen_cranelift_src_optimize_peephole.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/pointer.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_pointer {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_pointer.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/comments.rs
    pub mod rustc_codegen_cranelift_src_abi_comments {
        include!("processed_rustc_codegen_cranelift_src_abi_comments.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/issue-59326.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue_59326 {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_issue-59326.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/unwind.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_unwind {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_unwind.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/path.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_path {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_path.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/allocator.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_allocator {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_allocator.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/config.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_config {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_config.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/arbitrary_self_types_pointers_and_wrappers.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_arbitrary_self_types_pointers_and_wrappers {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_arbitrary_self_types_pointers_and_wrappers.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/dst-field-align.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_dst_field_align {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_dst-field-align.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/unsize.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_unsize {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_unsize.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/line_info.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_line_info {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_line_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/common.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_common {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/analyze.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_analyze {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_analyze.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/cargo-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_cargo_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_cargo-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/build_sysroot.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_sysroot {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_build_sysroot.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/main.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_main {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_main.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/subslice-patterns-const-eval.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_subslice_patterns_const_eval {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_subslice-patterns-const-eval.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/filter_profile.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_filter_profile {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_filter_profile.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/example.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_example {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_example.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/num.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_src_num {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_src_num.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/mini_core.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/rustc_info.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_rustc_info {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_rustc_info.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/debuginfo/types.rs
    pub mod rustc_codegen_cranelift_src_debuginfo_types {
        include!("processed_rustc_codegen_cranelift_src_debuginfo_types.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/utils.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_utils {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_utils.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/simd.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_simd {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_simd.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/example/mini_core_hello_world.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core_hello_world {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_example_mini_core_hello_world.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/rustc-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustc_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustc-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/abi/mod.rs
    pub mod rustc_codegen_cranelift_src_abi_mod {
        include!("processed_rustc_codegen_cranelift_src_abi_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/build_system/prepare.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_prepare {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_build_system_prepare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/scripts/rustdoc-clif.rs
    pub mod rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustdoc_clif {
        include!("processed_rustc_codegen_cranelift_rustc_codegen_cranelift_scripts_rustdoc-clif.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_cranelift/src/intrinsics/llvm_x86.rs
    pub mod rustc_codegen_cranelift_src_intrinsics_llvm_x86 {
        include!("processed_rustc_codegen_cranelift_src_intrinsics_llvm_x86.rs");
    }
}

// 15: rustc_index_macros (2 files)
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

// 16: rustc_symbol_mangling (7 files)
pub mod included_rustc_symbol_mangling {
    // Source: ../rust/compiler/rustc_symbol_mangling/src/test.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_test {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_test.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/errors.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_errors {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/hashed.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_hashed {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_hashed.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/legacy.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_legacy {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_legacy.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/v0.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_v0 {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_v0.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/export.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_export {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_export.rs");
    }
    // Source: ../rust/compiler/rustc_symbol_mangling/src/lib.rs
    pub mod rustc_symbol_mangling_rustc_symbol_mangling_src_lib {
        include!("processed_rustc_symbol_mangling_rustc_symbol_mangling_src_lib.rs");
    }
}

// 17: rustc_log (1 files)
pub mod included_rustc_log {
    // Source: ../rust/compiler/rustc_log/src/lib.rs
    pub mod rustc_log_rustc_log_src_lib {
        include!("processed_rustc_log_rustc_log_src_lib.rs");
    }
}

// 18: rustc_fs_util (1 files)
pub mod included_rustc_fs_util {
    // Source: ../rust/compiler/rustc_fs_util/src/lib.rs
    pub mod rustc_fs_util_rustc_fs_util_src_lib {
        include!("processed_rustc_fs_util_rustc_fs_util_src_lib.rs");
    }
}

// 19: rustc_errors (22 files)
pub mod included_rustc_errors {
    // Source: ../rust/compiler/rustc_errors/src/lock.rs
    pub mod rustc_errors_rustc_errors_src_lock {
        include!("processed_rustc_errors_rustc_errors_src_lock.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/markdown/mod.rs
    pub mod rustc_errors_src_markdown_mod {
        include!("processed_rustc_errors_src_markdown_mod.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/diagnostic.rs
    pub mod rustc_errors_rustc_errors_src_diagnostic {
        include!("processed_rustc_errors_rustc_errors_src_diagnostic.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/snippet.rs
    pub mod rustc_errors_rustc_errors_src_snippet {
        include!("processed_rustc_errors_rustc_errors_src_snippet.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/decorate_diag.rs
    pub mod rustc_errors_rustc_errors_src_decorate_diag {
        include!("processed_rustc_errors_rustc_errors_src_decorate_diag.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/codes.rs
    pub mod rustc_errors_rustc_errors_src_codes {
        include!("processed_rustc_errors_rustc_errors_src_codes.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/markdown/tests/term.rs
    pub mod rustc_errors_markdown_tests_term {
        include!("processed_rustc_errors_markdown_tests_term.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/styled_buffer.rs
    pub mod rustc_errors_rustc_errors_src_styled_buffer {
        include!("processed_rustc_errors_rustc_errors_src_styled_buffer.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/translation.rs
    pub mod rustc_errors_rustc_errors_src_translation {
        include!("processed_rustc_errors_rustc_errors_src_translation.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/error.rs
    pub mod rustc_errors_rustc_errors_src_error {
        include!("processed_rustc_errors_rustc_errors_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/tests.rs
    pub mod rustc_errors_rustc_errors_src_tests {
        include!("processed_rustc_errors_rustc_errors_src_tests.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/annotate_snippet_emitter_writer.rs
    pub mod rustc_errors_rustc_errors_src_annotate_snippet_emitter_writer {
        include!("processed_rustc_errors_rustc_errors_src_annotate_snippet_emitter_writer.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/diagnostic_impls.rs
    pub mod rustc_errors_rustc_errors_src_diagnostic_impls {
        include!("processed_rustc_errors_rustc_errors_src_diagnostic_impls.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/timings.rs
    pub mod rustc_errors_rustc_errors_src_timings {
        include!("processed_rustc_errors_rustc_errors_src_timings.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/emitter.rs
    pub mod rustc_errors_rustc_errors_src_emitter {
        include!("processed_rustc_errors_rustc_errors_src_emitter.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/markdown/tests/parse.rs
    pub mod rustc_errors_markdown_tests_parse {
        include!("processed_rustc_errors_markdown_tests_parse.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/json/tests.rs
    pub mod rustc_errors_src_json_tests {
        include!("processed_rustc_errors_src_json_tests.rs");
    }
    // Source: ../rust/compiler/rustc_errors/src/registry.rs
    pub mod rustc_errors_rustc_errors_src_registry {
        include!("processed_rustc_errors_rustc_errors_src_registry.rs");
    }
}

// 20: rustc_driver_impl (6 files)
pub mod included_rustc_driver_impl {
    // Source: ../rust/compiler/rustc_driver_impl/src/signal_handler.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_signal_handler {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_signal_handler.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/session_diagnostics.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_session_diagnostics {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_session_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/args.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_args {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_args.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/print.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_print {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_print.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/lib.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_lib {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_driver_impl/src/pretty.rs
    pub mod rustc_driver_impl_rustc_driver_impl_src_pretty {
        include!("processed_rustc_driver_impl_rustc_driver_impl_src_pretty.rs");
    }
}

// 21: rustc_borrowck (61 files)
pub mod included_rustc_borrowck {
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/explain_borrow.rs
    pub mod rustc_borrowck_src_diagnostics_explain_borrow {
        include!("processed_rustc_borrowck_src_diagnostics_explain_borrow.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/borrow_set.rs
    pub mod rustc_borrowck_rustc_borrowck_src_borrow_set {
        include!("processed_rustc_borrowck_rustc_borrowck_src_borrow_set.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/loan_invalidations.rs
    pub mod rustc_borrowck_polonius_legacy_loan_invalidations {
        include!("processed_rustc_borrowck_polonius_legacy_loan_invalidations.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/liveness/local_use_map.rs
    pub mod rustc_borrowck_type_check_liveness_local_use_map {
        include!("processed_rustc_borrowck_type_check_liveness_local_use_map.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/dump_mir.rs
    pub mod rustc_borrowck_src_region_infer_dump_mir {
        include!("processed_rustc_borrowck_src_region_infer_dump_mir.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/region_name.rs
    pub mod rustc_borrowck_src_diagnostics_region_name {
        include!("processed_rustc_borrowck_src_diagnostics_region_name.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/constraints/graph.rs
    pub mod rustc_borrowck_src_constraints_graph {
        include!("processed_rustc_borrowck_src_constraints_graph.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/place_ext.rs
    pub mod rustc_borrowck_rustc_borrowck_src_place_ext {
        include!("processed_rustc_borrowck_rustc_borrowck_src_place_ext.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/loan_liveness.rs
    pub mod rustc_borrowck_src_polonius_loan_liveness {
        include!("processed_rustc_borrowck_src_polonius_loan_liveness.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/relate_tys.rs
    pub mod rustc_borrowck_src_type_check_relate_tys {
        include!("processed_rustc_borrowck_src_type_check_relate_tys.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/liveness/mod.rs
    pub mod rustc_borrowck_type_check_liveness_mod {
        include!("processed_rustc_borrowck_type_check_liveness_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/typeck_constraints.rs
    pub mod rustc_borrowck_src_polonius_typeck_constraints {
        include!("processed_rustc_borrowck_src_polonius_typeck_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/values.rs
    pub mod rustc_borrowck_src_region_infer_values {
        include!("processed_rustc_borrowck_src_region_infer_values.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/mod.rs
    pub mod rustc_borrowck_src_diagnostics_mod {
        include!("processed_rustc_borrowck_src_diagnostics_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/opaque_types/member_constraints.rs
    pub mod rustc_borrowck_region_infer_opaque_types_member_constraints {
        include!("processed_rustc_borrowck_region_infer_opaque_types_member_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/constraints.rs
    pub mod rustc_borrowck_src_polonius_constraints {
        include!("processed_rustc_borrowck_src_polonius_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/free_region_relations.rs
    pub mod rustc_borrowck_src_type_check_free_region_relations {
        include!("processed_rustc_borrowck_src_type_check_free_region_relations.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/canonical.rs
    pub mod rustc_borrowck_src_type_check_canonical {
        include!("processed_rustc_borrowck_src_type_check_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/input_output.rs
    pub mod rustc_borrowck_src_type_check_input_output {
        include!("processed_rustc_borrowck_src_type_check_input_output.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/opaque_types/mod.rs
    pub mod rustc_borrowck_region_infer_opaque_types_mod {
        include!("processed_rustc_borrowck_region_infer_opaque_types_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/dataflow.rs
    pub mod rustc_borrowck_rustc_borrowck_src_dataflow {
        include!("processed_rustc_borrowck_rustc_borrowck_src_dataflow.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/constraint_conversion.rs
    pub mod rustc_borrowck_src_type_check_constraint_conversion {
        include!("processed_rustc_borrowck_src_type_check_constraint_conversion.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/opaque_types.rs
    pub mod rustc_borrowck_src_diagnostics_opaque_types {
        include!("processed_rustc_borrowck_src_diagnostics_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/var_name.rs
    pub mod rustc_borrowck_src_diagnostics_var_name {
        include!("processed_rustc_borrowck_src_diagnostics_var_name.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/used_muts.rs
    pub mod rustc_borrowck_rustc_borrowck_src_used_muts {
        include!("processed_rustc_borrowck_rustc_borrowck_src_used_muts.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/mod.rs
    pub mod rustc_borrowck_src_polonius_mod {
        include!("processed_rustc_borrowck_src_polonius_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/mod.rs
    pub mod rustc_borrowck_polonius_legacy_mod {
        include!("processed_rustc_borrowck_polonius_legacy_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/consumers.rs
    pub mod rustc_borrowck_rustc_borrowck_src_consumers {
        include!("processed_rustc_borrowck_rustc_borrowck_src_consumers.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/path_utils.rs
    pub mod rustc_borrowck_rustc_borrowck_src_path_utils {
        include!("processed_rustc_borrowck_rustc_borrowck_src_path_utils.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/mod.rs
    pub mod rustc_borrowck_src_region_infer_mod {
        include!("processed_rustc_borrowck_src_region_infer_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/liveness/trace.rs
    pub mod rustc_borrowck_type_check_liveness_trace {
        include!("processed_rustc_borrowck_type_check_liveness_trace.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/find_use.rs
    pub mod rustc_borrowck_src_diagnostics_find_use {
        include!("processed_rustc_borrowck_src_diagnostics_find_use.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/renumber.rs
    pub mod rustc_borrowck_rustc_borrowck_src_renumber {
        include!("processed_rustc_borrowck_rustc_borrowck_src_renumber.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/places_conflict.rs
    pub mod rustc_borrowck_rustc_borrowck_src_places_conflict {
        include!("processed_rustc_borrowck_rustc_borrowck_src_places_conflict.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/graphviz.rs
    pub mod rustc_borrowck_src_region_infer_graphviz {
        include!("processed_rustc_borrowck_src_region_infer_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/conflict_errors.rs
    pub mod rustc_borrowck_src_diagnostics_conflict_errors {
        include!("processed_rustc_borrowck_src_diagnostics_conflict_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/mutability_errors.rs
    pub mod rustc_borrowck_src_diagnostics_mutability_errors {
        include!("processed_rustc_borrowck_src_diagnostics_mutability_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/handle_placeholders.rs
    pub mod rustc_borrowck_rustc_borrowck_src_handle_placeholders {
        include!("processed_rustc_borrowck_rustc_borrowck_src_handle_placeholders.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/accesses.rs
    pub mod rustc_borrowck_polonius_legacy_accesses {
        include!("processed_rustc_borrowck_polonius_legacy_accesses.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/find_all_local_uses.rs
    pub mod rustc_borrowck_src_diagnostics_find_all_local_uses {
        include!("processed_rustc_borrowck_src_diagnostics_find_all_local_uses.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/liveness_constraints.rs
    pub mod rustc_borrowck_src_polonius_liveness_constraints {
        include!("processed_rustc_borrowck_src_polonius_liveness_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/region_errors.rs
    pub mod rustc_borrowck_src_diagnostics_region_errors {
        include!("processed_rustc_borrowck_src_diagnostics_region_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/universal_regions.rs
    pub mod rustc_borrowck_rustc_borrowck_src_universal_regions {
        include!("processed_rustc_borrowck_rustc_borrowck_src_universal_regions.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/outlives_suggestion.rs
    pub mod rustc_borrowck_src_diagnostics_outlives_suggestion {
        include!("processed_rustc_borrowck_src_diagnostics_outlives_suggestion.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/session_diagnostics.rs
    pub mod rustc_borrowck_rustc_borrowck_src_session_diagnostics {
        include!("processed_rustc_borrowck_rustc_borrowck_src_session_diagnostics.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/type_check/mod.rs
    pub mod rustc_borrowck_src_type_check_mod {
        include!("processed_rustc_borrowck_src_type_check_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/constraints/mod.rs
    pub mod rustc_borrowck_src_constraints_mod {
        include!("processed_rustc_borrowck_src_constraints_mod.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/opaque_types/region_ctxt.rs
    pub mod rustc_borrowck_region_infer_opaque_types_region_ctxt {
        include!("processed_rustc_borrowck_region_infer_opaque_types_region_ctxt.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/lib.rs
    pub mod rustc_borrowck_rustc_borrowck_src_lib {
        include!("processed_rustc_borrowck_rustc_borrowck_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/root_cx.rs
    pub mod rustc_borrowck_rustc_borrowck_src_root_cx {
        include!("processed_rustc_borrowck_rustc_borrowck_src_root_cx.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/borrowck_errors.rs
    pub mod rustc_borrowck_rustc_borrowck_src_borrowck_errors {
        include!("processed_rustc_borrowck_rustc_borrowck_src_borrowck_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/bound_region_errors.rs
    pub mod rustc_borrowck_src_diagnostics_bound_region_errors {
        include!("processed_rustc_borrowck_src_diagnostics_bound_region_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/nll.rs
    pub mod rustc_borrowck_rustc_borrowck_src_nll {
        include!("processed_rustc_borrowck_rustc_borrowck_src_nll.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/def_use.rs
    pub mod rustc_borrowck_rustc_borrowck_src_def_use {
        include!("processed_rustc_borrowck_rustc_borrowck_src_def_use.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/prefixes.rs
    pub mod rustc_borrowck_rustc_borrowck_src_prefixes {
        include!("processed_rustc_borrowck_rustc_borrowck_src_prefixes.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/diagnostics/move_errors.rs
    pub mod rustc_borrowck_src_diagnostics_move_errors {
        include!("processed_rustc_borrowck_src_diagnostics_move_errors.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/region_infer/reverse_sccs.rs
    pub mod rustc_borrowck_src_region_infer_reverse_sccs {
        include!("processed_rustc_borrowck_src_region_infer_reverse_sccs.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/location.rs
    pub mod rustc_borrowck_polonius_legacy_location {
        include!("processed_rustc_borrowck_polonius_legacy_location.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/loan_kills.rs
    pub mod rustc_borrowck_polonius_legacy_loan_kills {
        include!("processed_rustc_borrowck_polonius_legacy_loan_kills.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/dump.rs
    pub mod rustc_borrowck_src_polonius_dump {
        include!("processed_rustc_borrowck_src_polonius_dump.rs");
    }
    // Source: ../rust/compiler/rustc_borrowck/src/polonius/legacy/facts.rs
    pub mod rustc_borrowck_polonius_legacy_facts {
        include!("processed_rustc_borrowck_polonius_legacy_facts.rs");
    }
}

// 22: rustc_type_ir (39 files)
pub mod included_rustc_type_ir {
    // Source: ../rust/compiler/rustc_type_ir/src/solve/mod.rs
    pub mod rustc_type_ir_src_solve_mod {
        include!("processed_rustc_type_ir_src_solve_mod.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/binder.rs
    pub mod rustc_type_ir_rustc_type_ir_src_binder {
        include!("processed_rustc_type_ir_rustc_type_ir_src_binder.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/region_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_region_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_region_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/inherent.rs
    pub mod rustc_type_ir_rustc_type_ir_src_inherent {
        include!("processed_rustc_type_ir_rustc_type_ir_src_inherent.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/generic_arg.rs
    pub mod rustc_type_ir_rustc_type_ir_src_generic_arg {
        include!("processed_rustc_type_ir_rustc_type_ir_src_generic_arg.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/relate/solver_relating.rs
    pub mod rustc_type_ir_src_relate_solver_relating {
        include!("processed_rustc_type_ir_src_relate_solver_relating.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/search_graph/mod.rs
    pub mod rustc_type_ir_src_search_graph_mod {
        include!("processed_rustc_type_ir_src_search_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ty_info.rs
    pub mod rustc_type_ir_rustc_type_ir_src_ty_info {
        include!("processed_rustc_type_ir_rustc_type_ir_src_ty_info.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/lift.rs
    pub mod rustc_type_ir_rustc_type_ir_src_lift {
        include!("processed_rustc_type_ir_rustc_type_ir_src_lift.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/canonical.rs
    pub mod rustc_type_ir_rustc_type_ir_src_canonical {
        include!("processed_rustc_type_ir_rustc_type_ir_src_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/error.rs
    pub mod rustc_type_ir_rustc_type_ir_src_error {
        include!("processed_rustc_type_ir_rustc_type_ir_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/solve/inspect.rs
    pub mod rustc_type_ir_src_solve_inspect {
        include!("processed_rustc_type_ir_src_solve_inspect.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/flags.rs
    pub mod rustc_type_ir_rustc_type_ir_src_flags {
        include!("processed_rustc_type_ir_rustc_type_ir_src_flags.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/relate/combine.rs
    pub mod rustc_type_ir_src_relate_combine {
        include!("processed_rustc_type_ir_src_relate_combine.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ty_kind/closure.rs
    pub mod rustc_type_ir_src_ty_kind_closure {
        include!("processed_rustc_type_ir_src_ty_kind_closure.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/outlives.rs
    pub mod rustc_type_ir_rustc_type_ir_src_outlives {
        include!("processed_rustc_type_ir_rustc_type_ir_src_outlives.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/fold.rs
    pub mod rustc_type_ir_rustc_type_ir_src_fold {
        include!("processed_rustc_type_ir_rustc_type_ir_src_fold.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ty_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_ty_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_ty_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/data_structures/delayed_map.rs
    pub mod rustc_type_ir_src_data_structures_delayed_map {
        include!("processed_rustc_type_ir_src_data_structures_delayed_map.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/predicate.rs
    pub mod rustc_type_ir_rustc_type_ir_src_predicate {
        include!("processed_rustc_type_ir_rustc_type_ir_src_predicate.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/elaborate.rs
    pub mod rustc_type_ir_rustc_type_ir_src_elaborate {
        include!("processed_rustc_type_ir_rustc_type_ir_src_elaborate.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/ir_print.rs
    pub mod rustc_type_ir_rustc_type_ir_src_ir_print {
        include!("processed_rustc_type_ir_rustc_type_ir_src_ir_print.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/relate.rs
    pub mod rustc_type_ir_rustc_type_ir_src_relate {
        include!("processed_rustc_type_ir_rustc_type_ir_src_relate.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/const_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_const_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_const_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/search_graph/stack.rs
    pub mod rustc_type_ir_src_search_graph_stack {
        include!("processed_rustc_type_ir_src_search_graph_stack.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/search_graph/global_cache.rs
    pub mod rustc_type_ir_src_search_graph_global_cache {
        include!("processed_rustc_type_ir_src_search_graph_global_cache.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/upcast.rs
    pub mod rustc_type_ir_rustc_type_ir_src_upcast {
        include!("processed_rustc_type_ir_rustc_type_ir_src_upcast.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/fast_reject.rs
    pub mod rustc_type_ir_rustc_type_ir_src_fast_reject {
        include!("processed_rustc_type_ir_rustc_type_ir_src_fast_reject.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/opaque_ty.rs
    pub mod rustc_type_ir_rustc_type_ir_src_opaque_ty {
        include!("processed_rustc_type_ir_rustc_type_ir_src_opaque_ty.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/interner.rs
    pub mod rustc_type_ir_rustc_type_ir_src_interner {
        include!("processed_rustc_type_ir_rustc_type_ir_src_interner.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/infer_ctxt.rs
    pub mod rustc_type_ir_rustc_type_ir_src_infer_ctxt {
        include!("processed_rustc_type_ir_rustc_type_ir_src_infer_ctxt.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/walk.rs
    pub mod rustc_type_ir_rustc_type_ir_src_walk {
        include!("processed_rustc_type_ir_rustc_type_ir_src_walk.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/lang_items.rs
    pub mod rustc_type_ir_rustc_type_ir_src_lang_items {
        include!("processed_rustc_type_ir_rustc_type_ir_src_lang_items.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/visit.rs
    pub mod rustc_type_ir_rustc_type_ir_src_visit {
        include!("processed_rustc_type_ir_rustc_type_ir_src_visit.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/lib.rs
    pub mod rustc_type_ir_rustc_type_ir_src_lib {
        include!("processed_rustc_type_ir_rustc_type_ir_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/pattern.rs
    pub mod rustc_type_ir_rustc_type_ir_src_pattern {
        include!("processed_rustc_type_ir_rustc_type_ir_src_pattern.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/predicate_kind.rs
    pub mod rustc_type_ir_rustc_type_ir_src_predicate_kind {
        include!("processed_rustc_type_ir_rustc_type_ir_src_predicate_kind.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/macros.rs
    pub mod rustc_type_ir_rustc_type_ir_src_macros {
        include!("processed_rustc_type_ir_rustc_type_ir_src_macros.rs");
    }
    // Source: ../rust/compiler/rustc_type_ir/src/data_structures/mod.rs
    pub mod rustc_type_ir_src_data_structures_mod {
        include!("processed_rustc_type_ir_src_data_structures_mod.rs");
    }
}

// 23: rustc_ast_lowering (12 files)
pub mod included_rustc_ast_lowering {
    // Source: ../rust/compiler/rustc_ast_lowering/src/lib.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_lib {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/block.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_block {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_block.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/stability.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_stability {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_stability.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/format.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_format {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_format.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/asm.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_asm {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_asm.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/pat.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_pat {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_pat.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/index.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_index {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_index.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/errors.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_errors {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/expr.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_expr {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_expr.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/path.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_path {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_path.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/item.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_item {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_item.rs");
    }
    // Source: ../rust/compiler/rustc_ast_lowering/src/delegation.rs
    pub mod rustc_ast_lowering_rustc_ast_lowering_src_delegation {
        include!("processed_rustc_ast_lowering_rustc_ast_lowering_src_delegation.rs");
    }
}

// 24: rustc (2 files)
pub mod included_rustc {
    // Source: ../rust/compiler/rustc/build.rs
    pub mod rustc_compiler_rustc_build {
        include!("processed_rustc_compiler_rustc_build.rs");
    }
    // Source: ../rust/compiler/rustc/src/main.rs
    pub mod rustc_rustc_src_main {
        include!("processed_rustc_rustc_src_main.rs");
    }
}

// 25: rustc_privacy (2 files)
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

// 26: rustc_query_system (19 files)
pub mod included_rustc_query_system {
    // Source: ../rust/compiler/rustc_query_system/src/error.rs
    pub mod rustc_query_system_rustc_query_system_src_error {
        include!("processed_rustc_query_system_rustc_query_system_src_error.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/lib.rs
    pub mod rustc_query_system_rustc_query_system_src_lib {
        include!("processed_rustc_query_system_rustc_query_system_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/mod.rs
    pub mod rustc_query_system_src_dep_graph_mod {
        include!("processed_rustc_query_system_src_dep_graph_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/graph.rs
    pub mod rustc_query_system_src_dep_graph_graph {
        include!("processed_rustc_query_system_src_dep_graph_graph.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/debug.rs
    pub mod rustc_query_system_src_dep_graph_debug {
        include!("processed_rustc_query_system_src_dep_graph_debug.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/serialized.rs
    pub mod rustc_query_system_src_dep_graph_serialized {
        include!("processed_rustc_query_system_src_dep_graph_serialized.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/plumbing.rs
    pub mod rustc_query_system_src_query_plumbing {
        include!("processed_rustc_query_system_src_query_plumbing.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/edges.rs
    pub mod rustc_query_system_src_dep_graph_edges {
        include!("processed_rustc_query_system_src_dep_graph_edges.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/values.rs
    pub mod rustc_query_system_rustc_query_system_src_values {
        include!("processed_rustc_query_system_rustc_query_system_src_values.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/job.rs
    pub mod rustc_query_system_src_query_job {
        include!("processed_rustc_query_system_src_query_job.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/mod.rs
    pub mod rustc_query_system_src_query_mod {
        include!("processed_rustc_query_system_src_query_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/query.rs
    pub mod rustc_query_system_src_dep_graph_query {
        include!("processed_rustc_query_system_src_dep_graph_query.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/cache.rs
    pub mod rustc_query_system_rustc_query_system_src_cache {
        include!("processed_rustc_query_system_rustc_query_system_src_cache.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/config.rs
    pub mod rustc_query_system_src_query_config {
        include!("processed_rustc_query_system_src_query_config.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/hcx.rs
    pub mod rustc_query_system_src_ich_hcx {
        include!("processed_rustc_query_system_src_ich_hcx.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/dep_graph/dep_node.rs
    pub mod rustc_query_system_src_dep_graph_dep_node {
        include!("processed_rustc_query_system_src_dep_graph_dep_node.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/impls_syntax.rs
    pub mod rustc_query_system_src_ich_impls_syntax {
        include!("processed_rustc_query_system_src_ich_impls_syntax.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/ich/mod.rs
    pub mod rustc_query_system_src_ich_mod {
        include!("processed_rustc_query_system_src_ich_mod.rs");
    }
    // Source: ../rust/compiler/rustc_query_system/src/query/caches.rs
    pub mod rustc_query_system_src_query_caches {
        include!("processed_rustc_query_system_src_query_caches.rs");
    }
}

// 27: rustc_hir_analysis (51 files)
pub mod included_rustc_hir_analysis {
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/unsafety.rs
    pub mod rustc_hir_analysis_src_coherence_unsafety {
        include!("processed_rustc_hir_analysis_src_coherence_unsafety.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/mod.rs
    pub mod rustc_hir_analysis_src_check_mod {
        include!("processed_rustc_hir_analysis_src_check_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check_unused.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_check_unused {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_check_unused.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/always_applicable.rs
    pub mod rustc_hir_analysis_src_check_always_applicable {
        include!("processed_rustc_hir_analysis_src_check_always_applicable.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/region.rs
    pub mod rustc_hir_analysis_src_check_region {
        include!("processed_rustc_hir_analysis_src_check_region.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/autoderef.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_autoderef {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_autoderef.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/errors/wrong_number_of_generic_args.rs
    pub mod rustc_hir_analysis_src_errors_wrong_number_of_generic_args {
        include!("processed_rustc_hir_analysis_src_errors_wrong_number_of_generic_args.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/item_bounds.rs
    pub mod rustc_hir_analysis_src_collect_item_bounds {
        include!("processed_rustc_hir_analysis_src_collect_item_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/delegation.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_delegation {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_delegation.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/compare_impl_item/refine.rs
    pub mod rustc_hir_analysis_check_compare_impl_item_refine {
        include!("processed_rustc_hir_analysis_check_compare_impl_item_refine.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/compare_impl_item.rs
    pub mod rustc_hir_analysis_src_check_compare_impl_item {
        include!("processed_rustc_hir_analysis_src_check_compare_impl_item.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/solve.rs
    pub mod rustc_hir_analysis_src_variance_solve {
        include!("processed_rustc_hir_analysis_src_variance_solve.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/type_of.rs
    pub mod rustc_hir_analysis_src_collect_type_of {
        include!("processed_rustc_hir_analysis_src_collect_type_of.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/dump.rs
    pub mod rustc_hir_analysis_src_variance_dump {
        include!("processed_rustc_hir_analysis_src_variance_dump.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/impl_wf_check.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_impl_wf_check {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_impl_wf_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/wfcheck.rs
    pub mod rustc_hir_analysis_src_check_wfcheck {
        include!("processed_rustc_hir_analysis_src_check_wfcheck.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/lib.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_lib {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/dump.rs
    pub mod rustc_hir_analysis_src_outlives_dump {
        include!("processed_rustc_hir_analysis_src_outlives_dump.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/mod.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_mod {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/generics.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_generics {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_generics.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/orphan.rs
    pub mod rustc_hir_analysis_src_coherence_orphan {
        include!("processed_rustc_hir_analysis_src_coherence_orphan.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/utils.rs
    pub mod rustc_hir_analysis_src_outlives_utils {
        include!("processed_rustc_hir_analysis_src_outlives_utils.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/constrained_generic_params.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_constrained_generic_params {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_constrained_generic_params.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/cmse.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_cmse {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_cmse.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/builtin.rs
    pub mod rustc_hir_analysis_src_coherence_builtin {
        include!("processed_rustc_hir_analysis_src_coherence_builtin.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/inherent_impls_overlap.rs
    pub mod rustc_hir_analysis_src_coherence_inherent_impls_overlap {
        include!("processed_rustc_hir_analysis_src_coherence_inherent_impls_overlap.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/mod.rs
    pub mod rustc_hir_analysis_src_outlives_mod {
        include!("processed_rustc_hir_analysis_src_outlives_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/check.rs
    pub mod rustc_hir_analysis_src_check_check {
        include!("processed_rustc_hir_analysis_src_check_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/implicit_infer.rs
    pub mod rustc_hir_analysis_src_outlives_implicit_infer {
        include!("processed_rustc_hir_analysis_src_outlives_implicit_infer.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/lint.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_lint {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_lint.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/terms.rs
    pub mod rustc_hir_analysis_src_variance_terms {
        include!("processed_rustc_hir_analysis_src_variance_terms.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/impl_wf_check/min_specialization.rs
    pub mod rustc_hir_analysis_src_impl_wf_check_min_specialization {
        include!("processed_rustc_hir_analysis_src_impl_wf_check_min_specialization.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/mod.rs
    pub mod rustc_hir_analysis_src_variance_mod {
        include!("processed_rustc_hir_analysis_src_variance_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/generics_of.rs
    pub mod rustc_hir_analysis_src_collect_generics_of {
        include!("processed_rustc_hir_analysis_src_collect_generics_of.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/mod.rs
    pub mod rustc_hir_analysis_src_coherence_mod {
        include!("processed_rustc_hir_analysis_src_coherence_mod.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/outlives/explicit.rs
    pub mod rustc_hir_analysis_src_outlives_explicit {
        include!("processed_rustc_hir_analysis_src_outlives_explicit.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/errors.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_errors {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/intrinsic.rs
    pub mod rustc_hir_analysis_src_check_intrinsic {
        include!("processed_rustc_hir_analysis_src_check_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/errors/precise_captures.rs
    pub mod rustc_hir_analysis_src_errors_precise_captures {
        include!("processed_rustc_hir_analysis_src_errors_precise_captures.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/variance/constraints.rs
    pub mod rustc_hir_analysis_src_variance_constraints {
        include!("processed_rustc_hir_analysis_src_variance_constraints.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/bounds.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_bounds {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/check/entry.rs
    pub mod rustc_hir_analysis_src_check_entry {
        include!("processed_rustc_hir_analysis_src_check_entry.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/errors.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_errors {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_errors.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_ty_lowering/dyn_compatibility.rs
    pub mod rustc_hir_analysis_src_hir_ty_lowering_dyn_compatibility {
        include!("processed_rustc_hir_analysis_src_hir_ty_lowering_dyn_compatibility.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/type_of/opaque.rs
    pub mod rustc_hir_analysis_collect_type_of_opaque {
        include!("processed_rustc_hir_analysis_collect_type_of_opaque.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/predicates_of.rs
    pub mod rustc_hir_analysis_src_collect_predicates_of {
        include!("processed_rustc_hir_analysis_src_collect_predicates_of.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/dump.rs
    pub mod rustc_hir_analysis_src_collect_dump {
        include!("processed_rustc_hir_analysis_src_collect_dump.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect/resolve_bound_vars.rs
    pub mod rustc_hir_analysis_src_collect_resolve_bound_vars {
        include!("processed_rustc_hir_analysis_src_collect_resolve_bound_vars.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/hir_wf_check.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_hir_wf_check {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_hir_wf_check.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/collect.rs
    pub mod rustc_hir_analysis_rustc_hir_analysis_src_collect {
        include!("processed_rustc_hir_analysis_rustc_hir_analysis_src_collect.rs");
    }
    // Source: ../rust/compiler/rustc_hir_analysis/src/coherence/inherent_impls.rs
    pub mod rustc_hir_analysis_src_coherence_inherent_impls {
        include!("processed_rustc_hir_analysis_src_coherence_inherent_impls.rs");
    }
}

// 28: rustc_traits (9 files)
pub mod included_rustc_traits {
    // Source: ../rust/compiler/rustc_traits/src/lib.rs
    pub mod rustc_traits_rustc_traits_src_lib {
        include!("processed_rustc_traits_rustc_traits_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/type_op.rs
    pub mod rustc_traits_rustc_traits_src_type_op {
        include!("processed_rustc_traits_rustc_traits_src_type_op.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/normalize_projection_ty.rs
    pub mod rustc_traits_rustc_traits_src_normalize_projection_ty {
        include!("processed_rustc_traits_rustc_traits_src_normalize_projection_ty.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/evaluate_obligation.rs
    pub mod rustc_traits_rustc_traits_src_evaluate_obligation {
        include!("processed_rustc_traits_rustc_traits_src_evaluate_obligation.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/normalize_erasing_regions.rs
    pub mod rustc_traits_rustc_traits_src_normalize_erasing_regions {
        include!("processed_rustc_traits_rustc_traits_src_normalize_erasing_regions.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/codegen.rs
    pub mod rustc_traits_rustc_traits_src_codegen {
        include!("processed_rustc_traits_rustc_traits_src_codegen.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/dropck_outlives.rs
    pub mod rustc_traits_rustc_traits_src_dropck_outlives {
        include!("processed_rustc_traits_rustc_traits_src_dropck_outlives.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/implied_outlives_bounds.rs
    pub mod rustc_traits_rustc_traits_src_implied_outlives_bounds {
        include!("processed_rustc_traits_rustc_traits_src_implied_outlives_bounds.rs");
    }
    // Source: ../rust/compiler/rustc_traits/src/coroutine_witnesses.rs
    pub mod rustc_traits_rustc_traits_src_coroutine_witnesses {
        include!("processed_rustc_traits_rustc_traits_src_coroutine_witnesses.rs");
    }
}

// 29: rustc_error_messages (2 files)
pub mod included_rustc_error_messages {
    // Source: ../rust/compiler/rustc_error_messages/src/diagnostic_impls.rs
    pub mod rustc_error_messages_rustc_error_messages_src_diagnostic_impls {
        include!("processed_rustc_error_messages_rustc_error_messages_src_diagnostic_impls.rs");
    }
    // Source: ../rust/compiler/rustc_error_messages/src/lib.rs
    pub mod rustc_error_messages_rustc_error_messages_src_lib {
        include!("processed_rustc_error_messages_rustc_error_messages_src_lib.rs");
    }
}

// 30: rustc_next_trait_solver (24 files)
pub mod included_rustc_next_trait_solver {
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/inspect/mod.rs
    pub mod rustc_next_trait_solver_solve_inspect_mod {
        include!("processed_rustc_next_trait_solver_solve_inspect_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/canonicalizer.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_canonicalizer {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_canonicalizer.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/search_graph.rs
    pub mod rustc_next_trait_solver_src_solve_search_graph {
        include!("processed_rustc_next_trait_solver_src_solve_search_graph.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/effect_goals.rs
    pub mod rustc_next_trait_solver_src_solve_effect_goals {
        include!("processed_rustc_next_trait_solver_src_solve_effect_goals.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/assembly/mod.rs
    pub mod rustc_next_trait_solver_solve_assembly_mod {
        include!("processed_rustc_next_trait_solver_solve_assembly_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/alias_relate.rs
    pub mod rustc_next_trait_solver_src_solve_alias_relate {
        include!("processed_rustc_next_trait_solver_src_solve_alias_relate.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/assembly/structural_traits.rs
    pub mod rustc_next_trait_solver_solve_assembly_structural_traits {
        include!("processed_rustc_next_trait_solver_solve_assembly_structural_traits.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/free_alias.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_free_alias {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_free_alias.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/trait_goals.rs
    pub mod rustc_next_trait_solver_src_solve_trait_goals {
        include!("processed_rustc_next_trait_solver_src_solve_trait_goals.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/delegate.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_delegate {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_delegate.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/eval_ctxt/probe.rs
    pub mod rustc_next_trait_solver_solve_eval_ctxt_probe {
        include!("processed_rustc_next_trait_solver_solve_eval_ctxt_probe.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/anon_const.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_anon_const {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_anon_const.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/lib.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_lib {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/opaque_types.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_opaque_types {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_opaque_types.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/eval_ctxt/mod.rs
    pub mod rustc_next_trait_solver_solve_eval_ctxt_mod {
        include!("processed_rustc_next_trait_solver_solve_eval_ctxt_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/project_goals.rs
    pub mod rustc_next_trait_solver_src_solve_project_goals {
        include!("processed_rustc_next_trait_solver_src_solve_project_goals.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/eval_ctxt/canonical.rs
    pub mod rustc_next_trait_solver_solve_eval_ctxt_canonical {
        include!("processed_rustc_next_trait_solver_solve_eval_ctxt_canonical.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/inherent.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_inherent {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_inherent.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/coherence.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_coherence {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_coherence.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/inspect/build.rs
    pub mod rustc_next_trait_solver_solve_inspect_build {
        include!("processed_rustc_next_trait_solver_solve_inspect_build.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/resolve.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_resolve {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_resolve.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/normalizes_to/mod.rs
    pub mod rustc_next_trait_solver_solve_normalizes_to_mod {
        include!("processed_rustc_next_trait_solver_solve_normalizes_to_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/solve/mod.rs
    pub mod rustc_next_trait_solver_src_solve_mod {
        include!("processed_rustc_next_trait_solver_src_solve_mod.rs");
    }
    // Source: ../rust/compiler/rustc_next_trait_solver/src/placeholder.rs
    pub mod rustc_next_trait_solver_rustc_next_trait_solver_src_placeholder {
        include!("processed_rustc_next_trait_solver_rustc_next_trait_solver_src_placeholder.rs");
    }
}

// 31: rustc_ast_ir (2 files)
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

// 32: rustc_parse_format (2 files)
pub mod included_rustc_parse_format {
    // Source: ../rust/compiler/rustc_parse_format/src/tests.rs
    pub mod rustc_parse_format_rustc_parse_format_src_tests {
        include!("processed_rustc_parse_format_rustc_parse_format_src_tests.rs");
    }
}

// 33: rustc_codegen_ssa (54 files)
pub mod included_rustc_codegen_ssa {
    // Source: ../rust/compiler/rustc_codegen_ssa/src/codegen_attrs.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_codegen_attrs {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_codegen_attrs.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/common.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_common {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_common.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/coverageinfo.rs
    pub mod rustc_codegen_ssa_src_mir_coverageinfo {
        include!("processed_rustc_codegen_ssa_src_mir_coverageinfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/asm.rs
    pub mod rustc_codegen_ssa_src_traits_asm {
        include!("processed_rustc_codegen_ssa_src_traits_asm.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/symbol_export.rs
    pub mod rustc_codegen_ssa_src_back_symbol_export {
        include!("processed_rustc_codegen_ssa_src_back_symbol_export.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/intrinsic.rs
    pub mod rustc_codegen_ssa_src_mir_intrinsic {
        include!("processed_rustc_codegen_ssa_src_mir_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/debuginfo/mod.rs
    pub mod rustc_codegen_ssa_src_debuginfo_mod {
        include!("processed_rustc_codegen_ssa_src_debuginfo_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/coverageinfo.rs
    pub mod rustc_codegen_ssa_src_traits_coverageinfo {
        include!("processed_rustc_codegen_ssa_src_traits_coverageinfo.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/statics.rs
    pub mod rustc_codegen_ssa_src_traits_statics {
        include!("processed_rustc_codegen_ssa_src_traits_statics.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/locals.rs
    pub mod rustc_codegen_ssa_src_mir_locals {
        include!("processed_rustc_codegen_ssa_src_mir_locals.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/intrinsic.rs
    pub mod rustc_codegen_ssa_src_traits_intrinsic {
        include!("processed_rustc_codegen_ssa_src_traits_intrinsic.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/assert_module_sources.rs
    pub mod rustc_codegen_ssa_rustc_codegen_ssa_src_assert_module_sources {
        include!("processed_rustc_codegen_ssa_rustc_codegen_ssa_src_assert_module_sources.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/mod.rs
    pub mod rustc_codegen_ssa_src_back_mod {
        include!("processed_rustc_codegen_ssa_src_back_mod.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/rvalue.rs
    pub mod rustc_codegen_ssa_src_mir_rvalue {
        include!("processed_rustc_codegen_ssa_src_mir_rvalue.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/debuginfo/type_names.rs
    pub mod rustc_codegen_ssa_src_debuginfo_type_names {
        include!("processed_rustc_codegen_ssa_src_debuginfo_type_names.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/misc.rs
    pub mod rustc_codegen_ssa_src_traits_misc {
        include!("processed_rustc_codegen_ssa_src_traits_misc.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/rpath/tests.rs
    pub mod rustc_codegen_ssa_back_rpath_tests {
        include!("processed_rustc_codegen_ssa_back_rpath_tests.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/link.rs
    pub mod rustc_codegen_ssa_src_back_link {
        include!("processed_rustc_codegen_ssa_src_back_link.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/place.rs
    pub mod rustc_codegen_ssa_src_mir_place {
        include!("processed_rustc_codegen_ssa_src_mir_place.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/mir/analyze.rs
    pub mod rustc_codegen_ssa_src_mir_analyze {
        include!("processed_rustc_codegen_ssa_src_mir_analyze.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/declare.rs
    pub mod rustc_codegen_ssa_src_traits_declare {
        include!("processed_rustc_codegen_ssa_src_traits_declare.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/traits/builder.rs
    pub mod rustc_codegen_ssa_src_traits_builder {
        include!("processed_rustc_codegen_ssa_src_traits_builder.rs");
    }
    // Source: ../rust/compiler/rustc_codegen_ssa/src/back/command.rs
    pub mod rustc_codegen_ssa_src_back_command {
        include!("processed_rustc_codegen_ssa_src_back_command.rs");
    }
