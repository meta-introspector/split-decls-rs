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

// 1: rustc_attr_parsing (1 files)
pub mod included_rustc_attr_parsing {
    // Source: ../rust/compiler/rustc_attr_parsing/src/attributes/no_implicit_prelude.rs
    pub mod rustc_attr_parsing_src_attributes_no_implicit_prelude {
        include!("processed_rustc_attr_parsing_src_attributes_no_implicit_prelude.rs");
    }
}

