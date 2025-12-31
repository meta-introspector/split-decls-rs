// HIR datatypes. See the [rustc dev guide] for more info.
//
// [rustc dev guide]: https://rustc-dev-guide.rust-lang.org/hir.html

// tidy-alphabetical-start
#[feature(associated_type_defaults)]
#[feature(closure_track_caller)]
#[feature(debug_closure_helpers)]
#[feature(exhaustive_patterns)]
#[feature(never_type)]
#[feature(variant_count)]
#[recursion_limit = "256"]
// tidy-alphabetical-end


pub use crate::rustc_span::def_id;
pub use rustc_hir_id::{self as hir_id, *};


#[doc(no_inline)]
pub use hir::*;
pub use lang_items::{LangItem, LanguageItems};
pub use stability::*;
pub use stable_hash_impls::HashStableContext;
pub use target::{MethodKind, Target};
pub use version::*;

arena_types!(rustc_arena::declare_arena);