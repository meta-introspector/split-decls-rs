#![recursion_limit = "256"]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(yeet_expr)]
#![feature(negative_impls)]
#![feature(box_patterns)]
#![feature(decl_macro)]
#![feature(never_type)]
#![feature(try_blocks)]
#![feature(trait_alias)]
#![feature(rustc_attrs)]
#![feature(core_io_borrowed_buf)]
#![feature(assert_matches)]
#![feature(if_let_guard)]
#![feature(stmt_expr_attributes)]
#![feature(macro_metavar_expr)]
#![feature(cfg_select)]
#![feature(test)]
#![feature(type_alias_impl_trait)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
#![feature(no_core)]
#![feature(generic_atomic)]
#![feature(alloc_error_handler)]

// External rustc compiler crates
extern crate tracing;
extern crate rustc_ast;
extern crate rustc_middle;
extern crate rustc_span;
extern crate rustc_hir;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_session;
extern crate rustc_infer;
extern crate rustc_trait_selection;
extern crate rustc_abi;
extern crate rustc_index;
extern crate rustc_mir_dataflow;
extern crate alloc;
extern crate libc;
extern crate compiler_builtins;
extern crate rustc_target;
extern crate rustc_serialize;
extern crate bitflags;
extern crate derive_where;
extern crate rustc_type_ir_macros;
extern crate cranelift_codegen;
extern crate cranelift_frontend;
extern crate rustc_expand;
extern crate rustc_lint_defs;
extern crate rustc_ast_ir;
extern crate rustc_fluent_macro;
extern crate core_simd;
extern crate test_helpers;
extern crate syn;
extern crate quote;
extern crate proc_macro2;
extern crate synstructure;
extern crate memchr;
extern crate smallvec;
extern crate thin_vec;

pub mod rustc_topological;
// pub mod rustc_test;  // Disabled temporarily
pub mod symbol_resolver;
pub mod dependency_extractor;

// Explicit modules (take precedence over generated stubs)
// Removed conflicting modules: rustc_index_macros, rustc_data_structures, rustc_infer
pub mod rustc_index;  // Restored - needed for imports
pub mod rustc_serialize;  // Restored - needed for imports
pub mod fx;
pub mod sync;
pub mod stable_hasher;
pub mod graph;
pub mod source_map;
pub mod ty;
pub mod rustc_abi;
pub mod common;
pub mod test_rustc_complete_access;
pub mod test_rustc_index;

// Include the complete rustc code (actual implementations)
pub mod rustc_complete;
pub use rustc_complete::*;

// Auto-generated complete rustc includes from build.rs
// Create root-level modules that some code expects
pub mod rustc_infer { pub use crate::*; }
pub mod rustc_trait_selection { pub use crate::*; }

// Add core rustc modules that are heavily imported
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

// Common functions
pub fn bug() -> ! { panic!("bug") }
pub fn span_bug() -> ! { panic!("span_bug") }

// Common items
pub struct Span;
pub struct Symbol;
pub struct Session;
pub struct ErrorGuaranteed;
pub struct LangItem;
pub const DUMMY_SP: Span = Span;

// All other modules
pub mod errors { pub use crate::*; }
pub mod error_reporting { pub use crate::*; }
pub mod traits { pub use crate::*; }
pub mod mir { pub use crate::*; }
pub mod middle { pub struct Middle; }
pub mod query { pub struct Query; }
pub mod config { pub struct Config; }
pub mod attrs { pub struct Attrs; }
pub mod ffi { pub use crate::*; }
pub mod marker { pub use crate::*; }
pub mod regions { pub use crate::*; }
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
pub mod stable_hasher { pub use crate::*; }
pub mod ops_macros { pub use crate::*; }
pub mod simd { pub use crate::*; }
pub mod cli { pub use crate::*; }
pub mod types { pub use crate::*; }
pub mod attributes { pub use crate::*; }
pub mod context { pub use crate::*; }
pub mod target_checking { pub use crate::*; }
pub mod select { pub use crate::*; }
pub mod error { pub use crate::*; }
pub mod sys { pub use crate::*; }
pub mod io { pub use crate::*; }
pub mod cmp { pub use crate::*; }
pub mod num { pub use crate::*; }
pub mod deriving { pub use crate::*; }
pub mod sig_types { pub use crate::*; }
pub mod mystd { pub use crate::*; }
pub mod lazy { pub use crate::*; }
pub mod once { pub use crate::*; }
pub mod global { pub use crate::*; }
pub mod layout { pub use crate::*; }
