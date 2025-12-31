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
pub mod rustc_trait_selection {}
pub mod error_reporting {
    pub mod infer { pub use crate::*; }
}
pub mod traits {
    pub mod query { pub use crate::*; }
}
pub mod stable_hasher {}
pub mod rustc_data_structures {
    pub mod fx { pub use crate::*; }
}
pub mod rustc_macros {
    // Common rustc_macros items that are imported
    pub use proc_macro2::TokenStream;
    pub use quote::quote;
}
pub mod rustc_abi {}
pub mod rustc_index {}
pub mod rustc_codegen_ssa {}
pub mod rustc_hir {}
pub mod rustc_target {}
pub mod errors {}
pub mod infer { pub use crate::*; }

// Create rustc_complete module structure with common submodules
pub mod rustc_complete {
    pub use crate::*;
    pub mod ty { 
        pub use crate::*; 
        pub struct TyCtxt;
        pub struct Ty;
        pub mod print { pub use crate::*; }
        pub mod layout { pub use crate::*; }
        pub mod error { pub use crate::*; }
    }
    pub mod def_id { 
        pub use crate::*; 
        pub struct DefId;
        pub struct LocalDefId;
    }
    pub mod mir { pub use crate::*; }
    pub mod traits { 
        pub use crate::*; 
        pub mod query { pub use crate::*; }
    }
    pub mod query { pub use crate::*; }
    pub mod def { pub use crate::*; }
    pub mod errors { pub use crate::*; }
    pub mod infer { pub use crate::*; }
    pub mod sym { pub use crate::*; }
    pub mod parse { pub use crate::*; }
    pub mod middle { pub use crate::*; }
    pub mod attrs { pub use crate::*; }
    pub struct Span;
}

pub mod mini_core { pub use crate::*; }
pub mod context { pub use crate::*; }

pub mod rustc_complete;

pub use symbol_resolver::*;
pub use dependency_extractor::*;
