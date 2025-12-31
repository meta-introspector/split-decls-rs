#![recursion_limit = "256"]
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
#![feature(alloc_error_handler)]

pub mod rustc_topological;
// pub mod rustc_test;  // Disabled temporarily
pub mod symbol_resolver;
pub mod dependency_extractor;

// Manually add commonly needed root-level modules
pub mod rustc_infer {
    pub mod infer { pub use crate::*; }
    pub mod traits { pub use crate::*; }
}
pub mod rustc_trait_selection {
    pub mod infer { 
        pub use crate::*; 
        pub trait InferCtxtExt {}
    }
    pub mod traits { pub use crate::*; }
    pub mod error_reporting { pub use crate::*; }
}
pub mod error_reporting {
    pub use crate::*;
    pub mod infer { pub use crate::*; }
    pub struct TypeErrCtxt;
}
pub mod traits {
    pub mod query { pub use crate::*; }
}
pub mod stable_hasher {}
pub mod rustc_data_structures {
    pub mod fx { 
        pub use crate::*; 
        pub struct FxHashSet<T>(std::collections::HashSet<T>);
        pub struct FxHashMap<K, V>(std::collections::HashMap<K, V>);
        pub struct FxIndexMap<K, V>(std::collections::HashMap<K, V>);
        pub struct FxIndexSet<T>(std::collections::HashSet<T>);
    }
    pub mod stable_hasher { 
        pub use crate::*; 
        pub struct HashStable;
        pub struct StableHasher;
    }
    pub mod unord { pub use crate::*; }
    pub mod sync { pub use crate::*; }
    pub mod stack { pub use crate::*; }
    pub mod graph { pub use crate::*; }
    pub mod profiling { pub use crate::*; }
    pub mod fingerprint { pub use crate::*; }
    pub mod small_c_str { pub use crate::*; }
    pub mod intern { pub use crate::*; }
    pub mod frozen { pub use crate::*; }
    pub mod static_assert_size { pub use crate::*; }
}
pub mod rustc_macros {
    // Common rustc_macros items that are imported
    pub use proc_macro2::TokenStream;
    pub use quote::quote;
    pub struct Diagnostic;
    pub struct LintDiagnostic;
    pub struct Subdiagnostic;
}
pub mod rustc_abi {}
pub mod rustc_index {}
pub mod rustc_codegen_ssa {
    pub mod traits { pub use crate::*; }
}
pub mod rustc_mir_dataflow {}
pub mod rustc_index {}
pub mod coverage { pub use crate::*; }
pub mod llvm { pub use crate::*; }
pub mod common { pub use crate::*; }
pub mod rustc_expand {}
pub mod deriving { pub use crate::*; }
pub mod debuginfo { pub use crate::*; }
pub mod fluent_generated { pub use crate::*; }
pub mod FnCtxt { pub use crate::*; }
pub mod attributes { pub use crate::*; }
pub mod solve { pub use crate::*; }
pub mod delegate { pub use crate::*; }
pub mod lints { pub use crate::*; }
pub mod LateContext { pub use crate::*; }
pub mod LateLintPass { pub use crate::*; }
pub mod universal_regions { 
    pub use crate::*; 
    pub struct UniversalRegions;
}
pub mod region_infer { 
    pub use crate::*; 
    pub mod values { pub use crate::*; }
}
pub mod session_diagnostics { pub use crate::*; }
pub mod Interner { pub use crate::*; }
pub mod parser { pub use crate::*; }
pub mod rustc_abi {}
pub mod rustc_hir {}
pub mod rustc_index {}
pub mod rustc_target {}
pub mod rustc_expand {}
pub mod rustc_mir_dataflow {}
pub mod rustc_type_ir {}
pub mod rustc_parse {}
pub mod rustc_hir_analysis {}
pub mod rustc_query_system {}
pub mod rustc_public_bridge {}
pub mod rustc_pattern_analysis {}
pub mod rustc_ast_pretty {}
pub mod rustc_feature {}
pub mod rustc_attr_parsing {}
pub mod rustc_hashes {}
pub mod rustc_lint_defs {}
pub mod gccjit { pub use crate::*; }
pub mod type_of { pub use crate::*; }
pub mod errors {}
pub mod infer { pub use crate::*; }

// Create rustc_complete module structure with common submodules
pub mod rustc_complete {
    pub use crate::*;
    pub mod ty { 
        pub use crate::*; 
        pub struct TyCtxt;
        pub struct Ty;
        pub mod lint { pub use crate::*; }
    pub mod edition { pub use crate::*; }
    pub mod intravisit { pub use crate::*; }
    pub mod hygiene { pub use crate::*; }
    pub mod lang_items { pub use crate::*; }
    pub mod visit { pub use crate::*; }
    pub mod definitions { pub use crate::*; }
    pub mod dep_graph { pub use crate::*; }
    pub mod hir { pub use crate::*; }
    pub mod edit_distance { pub use crate::*; }
    pub mod cstore { pub use crate::*; }
    pub mod attr { pub use crate::*; }
    pub mod symbol { pub use crate::*; }
    pub mod ast { pub use crate::*; }
    pub mod expand { pub use crate::*; }
        pub mod print { pub use crate::*; }
        pub mod layout { pub use crate::*; }
        pub struct Instance;
        pub mod error { pub use crate::*; }
    }
    pub mod def_id { 
        pub use crate::*; 
        pub struct DefId;
        pub struct LocalDefId;
    }
    pub mod def {
        pub use crate::*;
        pub struct DefKind;
    }
    pub mod mir { 
        pub use crate::*; 
        pub mod visit { pub use crate::*; }
        pub mod interpret { pub use crate::*; }
    }
    pub mod traits { 
        pub use crate::*; 
        pub mod query { 
            pub use crate::*; 
            pub struct NoSolution;
        }
    }
    pub mod query { pub use crate::*; }
    pub mod errors { pub use crate::*; }
    pub mod infer { pub use crate::*; }
    pub mod sym { pub use crate::*; }
    pub mod parse { pub use crate::*; }
    pub mod middle { pub use crate::*; }
    pub mod attrs { 
        pub use crate::*; 
        pub struct AttributeKind;
    }
    pub mod thir { pub use crate::*; }
    pub mod source_map { 
        pub use crate::*; 
        pub struct Spanned<T>(pub T);
    }
    pub mod codes { pub use crate::*; }
    pub mod config { pub use crate::*; }
    pub mod tokenstream { pub use crate::*; }
    pub mod token { pub use crate::*; }
    pub mod util { pub use crate::*; }
    pub mod kw { pub use crate::*; }
    pub struct Ident;
    pub struct Span;
    pub const DUMMY_SP: Span = Span;
    pub struct ErrorGuaranteed;
    pub struct LangItem;
    pub struct BytePos;
    pub struct Diag;
    pub struct MultiSpan;
    pub struct DiagCtxtHandle;
    pub struct Applicability;
    pub struct PResult<T>(pub T);
    pub struct Mutability;
    pub struct HirId;
    pub struct Attribute;
    pub struct Target;
}

pub mod mini_core { pub use crate::*; }
pub mod context { pub use crate::*; }
pub mod prelude { pub use crate::*; }
pub mod ty { pub use crate::*; }
pub mod builder { pub use crate::*; }

pub mod rustc_complete;

pub use symbol_resolver::*;
pub use dependency_extractor::*;
