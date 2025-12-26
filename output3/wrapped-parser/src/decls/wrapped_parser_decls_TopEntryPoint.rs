use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parse the whole of the input as a given syntactic construct.
///
/// This covers two main use-cases:
///
///   * Parsing a Rust file.
///   * Parsing a result of macro expansion.
///
/// That is, for something like
///
/// ```ignore
/// quick_check! {
///    fn prop() {}
/// }
/// ```
///
/// the input to the macro will be parsed with [`PrefixEntryPoint::Item`], and
/// the result will be [`TopEntryPoint::MacroItems`].
///
/// [`TopEntryPoint::parse`] makes a guarantee that
///   * all input is consumed
///   * the result is a valid tree (there's one root node)
#[derive(Debug)]
pub enum TopEntryPoint {
    SourceFile,
    MacroStmts,
    MacroItems,
    Pattern,
    Type,
    Expr,
    /// Edge case -- macros generally don't expand to attributes, with the
    /// exception of `cfg_attr` which does!
    MetaItem,
}
