use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parse a prefix of the input as a given syntactic construct.
///
/// This is used by macro-by-example parser to implement things like `$i:item`
/// and the naming of variants follows the naming of macro fragments.
///
/// Note that this is generally non-optional -- the result is intentionally not
/// `Option<Output>`. The way MBE work, by the time we *try* to parse `$e:expr`
/// we already commit to expression. In other words, this API by design can't be
/// used to implement "rollback and try another alternative" logic.
#[derive(Debug)]
pub enum PrefixEntryPoint {
    Vis,
    Block,
    Stmt,
    Pat,
    PatTop,
    Ty,
    Expr,
    Path,
    Item,
    MetaItem,
}
