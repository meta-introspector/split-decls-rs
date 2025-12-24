use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A meta-variable expression, for expansions based on properties of meta-variables.
#[derive(Debug, PartialEq, Encodable, Decodable)]
pub enum MetaVarExpr {
    /// Unification of two or more identifiers.
    Concat(Box<[MetaVarExprConcatElem]>),
    /// The number of repetitions of an identifier.
    Count(Ident, usize),
    /// Ignore a meta-variable for repetition without expansion.
    Ignore(Ident),
    /// The index of the repetition at a particular depth, where 0 is the innermost
    /// repetition. The `usize` is the depth.
    Index(usize),
    /// The length of the repetition at a particular depth, where 0 is the innermost
    /// repetition. The `usize` is the depth.
    Len(usize),
}
