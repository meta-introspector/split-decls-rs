// SRC: ../rust/compiler/rustc_index_macros/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=newtype_index | COMPLEXITY=8 | LINES=40 */
// tidy-alphabetical-start
#[cfg_attr(feature = "nightly", allow(internal_features))]
#[cfg_attr(feature = "nightly", feature(allow_internal_unstable))]
// tidy-alphabetical-end

use proc_macro::TokenStream;


/// Creates a struct type `S` that can be used as an index with
/// `IndexVec` and so on.
///
/// There are two ways of interacting with these indices:
///
/// - The `From` impls are the preferred way. So you can do
///   `S::from(v)` with a `usize` or `u32`. And you can convert back
///   to an integer with `u32::from(s)`.
///
/// - Alternatively, you can use the methods `S::new(v)` and `s.index()`
///   to create/return a value.
///
/// Internally, the index uses a u32, so the index must not exceed
/// `u32::MAX`.
///
/// The impls provided by default are Clone, Copy, PartialEq, Eq, and Hash.
///
/// Accepted attributes for customization:
/// - `#[derive(HashStable_Generic)]`/`#[derive(HashStable)]`: derives
///   `HashStable`, as normal.
/// - `#[encodable]`: derives `Encodable`/`Decodable`.
/// - `#[orderable]`: derives `PartialOrd`/`Ord`, plus step-related methods.
/// - `#[debug_format = "Foo({})"]`: derives `Debug` with particular output.
/// - `#[max = 0xFFFF_FFFD]`: specifies the max value, which allows niche
///   optimizations. The default max value is 0xFFFF_FF00.
/// - `#[gate_rustc_only]`: makes parts of the generated code nightly-only.
// #[proc_macro] - removed
#[cfg_attr(feature = "nightly", allow_internal_unstable(step_trait, rustc_attrs, trusted_step))]
pub fn newtype_index(input: TokenStream) -> TokenStream {
    newtype::newtype(input)
}