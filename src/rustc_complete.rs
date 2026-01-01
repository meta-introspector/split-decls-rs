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
    // Source: ../rust/compiler/rustc_data_structures/src/atomic_ref.rs
    pub mod rustc_data_structures_rustc_data_structures_src_atomic_ref {
        include!("processed_rustc_data_structures_rustc_data_structures_src_atomic_ref.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/small_c_str/tests.rs
    pub mod rustc_data_structures_src_small_c_str_tests {
        include!("processed_rustc_data_structures_src_small_c_str_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/frozen.rs
    pub mod rustc_data_structures_rustc_data_structures_src_frozen {
        include!("processed_rustc_data_structures_rustc_data_structures_src_frozen.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/lock.rs
    pub mod rustc_data_structures_src_sync_lock {
        include!("processed_rustc_data_structures_src_sync_lock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/graphviz.rs
    pub mod rustc_data_structures_src_obligation_forest_graphviz {
        include!("processed_rustc_data_structures_src_obligation_forest_graphviz.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/intern/tests.rs
    pub mod rustc_data_structures_src_intern_tests {
        include!("processed_rustc_data_structures_src_intern_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/tests.rs
    pub mod rustc_data_structures_src_sorted_map_tests {
        include!("processed_rustc_data_structures_src_sorted_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/vec.rs
    pub mod rustc_data_structures_src_sync_vec {
        include!("processed_rustc_data_structures_src_sync_vec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fx.rs
    pub mod rustc_data_structures_rustc_data_structures_src_fx {
        include!("processed_rustc_data_structures_rustc_data_structures_src_fx.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stable_hasher/tests.rs
    pub mod rustc_data_structures_src_stable_hasher_tests {
        include!("processed_rustc_data_structures_src_stable_hasher_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/scc/tests.rs
    pub mod rustc_data_structures_graph_scc_tests {
        include!("processed_rustc_data_structures_graph_scc_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/iterate/tests.rs
    pub mod rustc_data_structures_graph_iterate_tests {
        include!("processed_rustc_data_structures_graph_iterate_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/set.rs
    pub mod rustc_data_structures_src_sso_set {
        include!("processed_rustc_data_structures_src_sso_set.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unsupported.rs
    pub mod rustc_data_structures_src_flock_unsupported {
        include!("processed_rustc_data_structures_src_flock_unsupported.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/parallel.rs
    pub mod rustc_data_structures_src_sync_parallel {
        include!("processed_rustc_data_structures_src_sync_parallel.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sharded.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sharded {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sharded.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/map.rs
    pub mod rustc_data_structures_src_sso_map {
        include!("processed_rustc_data_structures_src_sso_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unord.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unord {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unord.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/windows.rs
    pub mod rustc_data_structures_src_flock_windows {
        include!("processed_rustc_data_structures_src_flock_windows.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/tagged_ptr/tests.rs
    pub mod rustc_data_structures_src_tagged_ptr_tests {
        include!("processed_rustc_data_structures_src_tagged_ptr_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/lib.rs
    pub mod rustc_data_structures_rustc_data_structures_src_lib {
        include!("processed_rustc_data_structures_rustc_data_structures_src_lib.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flat_map_in_place.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flat_map_in_place {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flat_map_in_place.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/svh.rs
    pub mod rustc_data_structures_rustc_data_structures_src_svh {
        include!("processed_rustc_data_structures_rustc_data_structures_src_svh.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/fingerprint/tests.rs
    pub mod rustc_data_structures_src_fingerprint_tests {
        include!("processed_rustc_data_structures_src_fingerprint_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/linked_graph/tests.rs
    pub mod rustc_data_structures_graph_linked_graph_tests {
        include!("processed_rustc_data_structures_graph_linked_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/binary_search_util/tests.rs
    pub mod rustc_data_structures_src_binary_search_util_tests {
        include!("processed_rustc_data_structures_src_binary_search_util_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/marker.rs
    pub mod rustc_data_structures_rustc_data_structures_src_marker {
        include!("processed_rustc_data_structures_rustc_data_structures_src_marker.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/aligned.rs
    pub mod rustc_data_structures_rustc_data_structures_src_aligned {
        include!("processed_rustc_data_structures_rustc_data_structures_src_aligned.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/union_find/tests.rs
    pub mod rustc_data_structures_src_union_find_tests {
        include!("processed_rustc_data_structures_src_union_find_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/obligation_forest/tests.rs
    pub mod rustc_data_structures_src_obligation_forest_tests {
        include!("processed_rustc_data_structures_src_obligation_forest_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/profiling/tests.rs
    pub mod rustc_data_structures_src_profiling_tests {
        include!("processed_rustc_data_structures_src_profiling_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/freeze.rs
    pub mod rustc_data_structures_src_sync_freeze {
        include!("processed_rustc_data_structures_src_sync_freeze.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/snapshot_map/tests.rs
    pub mod rustc_data_structures_src_snapshot_map_tests {
        include!("processed_rustc_data_structures_src_snapshot_map_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thinvec.rs
    pub mod rustc_data_structures_rustc_data_structures_src_thinvec {
        include!("processed_rustc_data_structures_rustc_data_structures_src_thinvec.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/vec_graph/tests.rs
    pub mod rustc_data_structures_graph_vec_graph_tests {
        include!("processed_rustc_data_structures_graph_vec_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/base_n/tests.rs
    pub mod rustc_data_structures_src_base_n_tests {
        include!("processed_rustc_data_structures_src_base_n_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/dominators/tests.rs
    pub mod rustc_data_structures_graph_dominators_tests {
        include!("processed_rustc_data_structures_graph_dominators_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/packed.rs
    pub mod rustc_data_structures_rustc_data_structures_src_packed {
        include!("processed_rustc_data_structures_rustc_data_structures_src_packed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/vec_cache/tests.rs
    pub mod rustc_data_structures_src_vec_cache_tests {
        include!("processed_rustc_data_structures_src_vec_cache_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/jobserver.rs
    pub mod rustc_data_structures_rustc_data_structures_src_jobserver {
        include!("processed_rustc_data_structures_rustc_data_structures_src_jobserver.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/owned_slice/tests.rs
    pub mod rustc_data_structures_src_owned_slice_tests {
        include!("processed_rustc_data_structures_src_owned_slice_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sorted_map/index_map.rs
    pub mod rustc_data_structures_src_sorted_map_index_map {
        include!("processed_rustc_data_structures_src_sorted_map_index_map.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync.rs
    pub mod rustc_data_structures_rustc_data_structures_src_sync {
        include!("processed_rustc_data_structures_rustc_data_structures_src_sync.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/work_queue.rs
    pub mod rustc_data_structures_rustc_data_structures_src_work_queue {
        include!("processed_rustc_data_structures_rustc_data_structures_src_work_queue.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock.rs
    pub mod rustc_data_structures_rustc_data_structures_src_flock {
        include!("processed_rustc_data_structures_rustc_data_structures_src_flock.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reference.rs
    pub mod rustc_data_structures_src_graph_reference {
        include!("processed_rustc_data_structures_src_graph_reference.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/reversed.rs
    pub mod rustc_data_structures_src_graph_reversed {
        include!("processed_rustc_data_structures_src_graph_reversed.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/stack.rs
    pub mod rustc_data_structures_rustc_data_structures_src_stack {
        include!("processed_rustc_data_structures_rustc_data_structures_src_stack.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/memmap.rs
    pub mod rustc_data_structures_rustc_data_structures_src_memmap {
        include!("processed_rustc_data_structures_rustc_data_structures_src_memmap.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/transitive_relation/tests.rs
    pub mod rustc_data_structures_src_transitive_relation_tests {
        include!("processed_rustc_data_structures_src_transitive_relation_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/unix.rs
    pub mod rustc_data_structures_src_flock_unix {
        include!("processed_rustc_data_structures_src_flock_unix.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/steal.rs
    pub mod rustc_data_structures_rustc_data_structures_src_steal {
        include!("processed_rustc_data_structures_rustc_data_structures_src_steal.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/graph/tests.rs
    pub mod rustc_data_structures_src_graph_tests {
        include!("processed_rustc_data_structures_src_graph_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/temp_dir.rs
    pub mod rustc_data_structures_rustc_data_structures_src_temp_dir {
        include!("processed_rustc_data_structures_rustc_data_structures_src_temp_dir.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/flock/linux.rs
    pub mod rustc_data_structures_src_flock_linux {
        include!("processed_rustc_data_structures_src_flock_linux.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sso/mod.rs
    pub mod rustc_data_structures_src_sso_mod {
        include!("processed_rustc_data_structures_src_sso_mod.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/thousands/tests.rs
    pub mod rustc_data_structures_src_thousands_tests {
        include!("processed_rustc_data_structures_src_thousands_tests.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/unhash.rs
    pub mod rustc_data_structures_rustc_data_structures_src_unhash {
        include!("processed_rustc_data_structures_rustc_data_structures_src_unhash.rs");
    }
    // Source: ../rust/compiler/rustc_data_structures/src/sync/worker_local.rs
    pub mod rustc_data_structures_src_sync_worker_local {
        include!("processed_rustc_data_structures_src_sync_worker_local.rs");
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
    // Source: ../rust/compiler/rustc_index/src/idx.rs
    pub mod rustc_index_rustc_index_src_idx {
        include!("processed_rustc_index_rustc_index_src_idx.rs");
    }
    // Source: ../rust/compiler/rustc_index/src/slice.rs
    pub mod rustc_index_rustc_index_src_slice {
        include!("processed_rustc_index_rustc_index_src_slice.rs");
    }
}

// 4: rustc_span (17 files)
pub mod included_rustc_span {
    // Source: ../rust/compiler/rustc_span/src/symbol/tests.rs
    pub mod rustc_span_src_symbol_tests {
        include!("processed_rustc_span_src_symbol_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edit_distance/tests.rs
    pub mod rustc_span_src_edit_distance_tests {
        include!("processed_rustc_span_src_edit_distance_tests.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/edition.rs
    pub mod rustc_span_rustc_span_src_edition {
        include!("processed_rustc_span_rustc_span_src_edition.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/def_id.rs
    pub mod rustc_span_rustc_span_src_def_id {
        include!("processed_rustc_span_rustc_span_src_def_id.rs");
    }
    // Source: ../rust/compiler/rustc_span/src/source_map/tests.rs
    pub mod rustc_span_src_source_map_tests {
        include!("processed_rustc_span_src_source_map_tests.rs");
    }
