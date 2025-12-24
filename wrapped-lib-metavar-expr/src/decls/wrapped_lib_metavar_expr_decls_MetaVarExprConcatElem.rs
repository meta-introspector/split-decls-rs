use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Indicates what is placed in a `concat` parameter. For example, literals
/// (`${concat("foo", "bar")}`) or adhoc identifiers (`${concat(foo, bar)}`).
#[derive(Debug, Decodable, Encodable, PartialEq)]
pub enum MetaVarExprConcatElem {
    /// Identifier WITHOUT a preceding dollar sign, which means that this identifier should be
    /// interpreted as a literal.
    Ident(Ident),
    /// For example, a number or a string.
    Literal(Symbol),
    /// Identifier WITH a preceding dollar sign, which means that this identifier should be
    /// expanded and interpreted as a variable.
    Var(Ident),
}
