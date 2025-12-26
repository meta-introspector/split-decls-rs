//! This module defines the `DummyAstNode` trait, which is essential for error recovery
//! during the Rust compiler's macro expansion process.
//!
//! For n00bs: When a macro tries to change your code, sometimes things go wrong. Instead of
//! the whole compiler crashing, `DummyAstNode` provides a "fake" or "placeholder" version
//! of a code piece (an AST node). This fake piece of code allows the compiler to keep going,
//! even with an error, so it can find and report *all* the problems in your code, not just
//! the first one. It's like putting a "Coming Soon" sign where something broke, so you can
//! still look at the rest of the building.

use rustc_ast::{self as ast, NodeId, PatKind, TyKind, AstNodeWrapper, DUMMY_NODE_ID};
use rustc_span::Span;

/// A trait for AST nodes that can produce a "dummy" or placeholder version of themselves.
///
/// This is primarily used for error recovery during macro expansion. If a macro fails
/// to produce a valid AST node, a dummy node can be inserted into the AST. This allows
/// the compiler to continue with further processing and error reporting, rather than
/// stopping compilation entirely.
///
/// For n00bs: This trait gives any important piece of Rust code (like an expression,
/// a type, or a whole program "crate") the ability to say: "If I break, here's a safe,
/// empty version of myself that the compiler can use to keep working."
pub trait DummyAstNode {
    /// Creates a default, dummy instance of the AST node.
    fn dummy() -> Self;
}

// Below are implementations of `DummyAstNode` for various core AST types.
// These define what a "dummy" looks like for each specific kind of code structure.

impl DummyAstNode for ast::Crate {
    /// Returns a dummy `ast::Crate` node.
    fn dummy() -> Self {
        ast::Crate {
            attrs: Default::default(),
            items: Default::default(),
            spans: ast::ModSpans { inner_span: Span::default(), ..Default::default() }, // Using default span for now
            id: DUMMY_NODE_ID,
            is_placeholder: Default::default(),
        }
    }
}

impl DummyAstNode for ast::Ty {
    /// Returns a dummy `ast::Ty` node.
    fn dummy() -> Self {
        ast::Ty {
            id: DUMMY_NODE_ID,
            kind: TyKind::Dummy,
            span: Span::default(), // Using default span for now
            tokens: Default::default(),
        }
    }
}

impl DummyAstNode for ast::Pat {
    /// Returns a dummy `ast::Pat` (pattern) node.
    fn dummy() -> Self {
        ast::Pat {
            id: DUMMY_NODE_ID,
            kind: PatKind::Wild,
            span: Span::default(), // Using default span for now
            tokens: Default::default(),
        }
    }
}

impl DummyAstNode for ast::Expr {
    /// Returns a dummy `ast::Expr` (expression) node.
    fn dummy() -> Self {
        ast::Expr {
            id: DUMMY_NODE_ID,
            kind: ast::ExprKind::Tup(Default::default()), // A dummy empty tuple expression
            span: Span::default(), // Using default span for now
            attrs: Default::default(),
            tokens: Default::default(),
        }
    }
}

/// A tag type used to wrap an `ast::Expr` specifically for method receiver contexts.
pub struct MethodReceiverTag;

impl DummyAstNode for AstNodeWrapper<ast::Expr, MethodReceiverTag> {
    /// Returns a dummy `AstNodeWrapper` for a method receiver expression.
    fn dummy() -> Self {
        AstNodeWrapper::new(ast::Expr::dummy(), MethodReceiverTag)
    }
}
