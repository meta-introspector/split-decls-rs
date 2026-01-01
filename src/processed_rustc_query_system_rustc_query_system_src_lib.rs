// SRC: ../rust/compiler/rustc_query_system/src/lib.rs
// tidy-alphabetical-start
#[allow(internal_features)]
#[feature(assert_matches)]
#[feature(core_intrinsics)]
#[feature(min_specialization)]
// tidy-alphabetical-end


pub use error::{HandleCycleError, QueryOverflow, QueryOverflowNote};
pub use values::Value;

rustc_fluent_macro::fluent_messages! { "../messages.ftl" }