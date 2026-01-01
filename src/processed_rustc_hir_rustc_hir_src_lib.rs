/* FP:lib.rs-0001 */ // HIR datatypes. See the [rustc dev guide] for more info.
/* FP:lib.rs-0002 */ //
/* FP:lib.rs-0003 */ // [rustc dev guide]: https://rustc-dev-guide.rust-lang.org/hir.html
/* FP:lib.rs-0004 */ 
/* FP:lib.rs-0005 */ // tidy-alphabetical-start
/* FP:lib.rs-0006 */ #[feature(associated_type_defaults)]
/* FP:lib.rs-0007 */ #[feature(closure_track_caller)]
/* FP:lib.rs-0008 */ #[feature(debug_closure_helpers)]
/* FP:lib.rs-0009 */ #[feature(exhaustive_patterns)]
/* FP:lib.rs-0010 */ #[feature(never_type)]
/* FP:lib.rs-0011 */ #[feature(variant_count)]
/* FP:lib.rs-0012 */ #[recursion_limit = "256"]
/* FP:lib.rs-0013 */ // tidy-alphabetical-end
/* FP:lib.rs-0014 */ 
/* FP:lib.rs-0015 */ 
/* FP:lib.rs-0022 */ pub use crate::rustc_complete::def_id;
/* FP:lib.rs-0024 */ pub use rustc_hir_id::{self as hir_id, *};
/* FP:lib.rs-0035 */ 
/* FP:lib.rs-0036 */ #[cfg(test)]
/* FP:lib.rs-0038 */ 
/* FP:lib.rs-0039 */ #[doc(no_inline)]
/* FP:lib.rs-0040 */ pub use hir::*;
/* FP:lib.rs-0041 */ pub use lang_items::{LangItem, LanguageItems};
/* FP:lib.rs-0042 */ pub use stability::*;
/* FP:lib.rs-0043 */ pub use stable_hash_impls::HashStableContext;
/* FP:lib.rs-0044 */ pub use target::{MethodKind, Target};
/* FP:lib.rs-0045 */ pub use version::*;
/* FP:lib.rs-0046 */ 
/* FP:lib.rs-0047 */ arena_types!(rustc_arena::declare_arena);