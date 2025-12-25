use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Does this expression require a semicolon to be treated
/// as a statement? The negation of this: 'can this expression
/// be used as a statement without a semicolon' -- is used
/// as an early-bail-out in the parser so that, for instance,
///     if true {...} else {...}
///      |x| 5
/// isn't parsed as (if true {...} else {...} | x) | 5
fn expr_requires_semi_to_be_stmt(e: &hir::Expr<'_>) -> bool {
    !matches!(
        e.kind,
        hir::ExprKind::If(..)
            | hir::ExprKind::Match(..)
            | hir::ExprKind::Block(..)
            | hir::ExprKind::Loop(..)
    )
}
