//! This module defines the `Annotatable` enum, which is a key concept in Rust's macro expansion
//! process. It represents an Abstract Syntax Tree (AST) node that can be annotated with attributes.
//! Macros often operate on these annotatable structures, modifying or generating new code based on them.
//!
//! For n00bs: Imagine your Rust code as a tree (the AST). Each part of your code (like a function,
//! a variable declaration, or even a whole crate) is a "node" in this tree. Sometimes, you want
//! to attach special instructions (called "attributes," like `#[derive(Debug)]` or `#[test]`)
//! to these nodes. The `Annotatable` enum is a way to say "this specific type of AST node can
//! have attributes attached to it, and here's how we handle them."
//!
//! By having a common `Annotatable` type, the macro system can treat different kinds of AST nodes
//! (like items, expressions, statements, etc.) in a uniform way when it comes to attributes and
//! other transformations. This makes the macro system more flexible and powerful.

use rustc_ast::visit::{AssocCtxt, Visitor};
use rustc_ast::{self as ast, AttrVec, HasAttrs, NodeId};
use rustc_span::Span;
use rustc_ast::tokenstream::TokenStream;


/// Represents an Abstract Syntax Tree (AST) node that can be annotated with attributes.
///
/// This enum provides a unified way to handle different kinds of AST nodes (items, expressions,
/// statements, etc.) that can be targets for attributes in macro expansion. When a macro
/// expands, it often receives or produces `Annotatable` types.
///
/// For n00bs: Think of `Annotatable` as a "wrapper" that can hold many different kinds of Rust
/// code structures (like a function, a variable, an enum, etc.). The macro system uses this
/// wrapper so it doesn't have to write separate code for every single type of Rust code it
/// might want to change or look at.
///
/// When adding new variants here, remember that other parts of the macro system,
/// particularly those that visit or transform AST nodes (like `InvocationCollector`),
/// might need to be updated to handle the new variant correctly.
#[derive(Debug, Clone)]
pub enum Annotatable {
    Item(Box<ast::Item>),
    AssocItem(Box<ast::AssocItem>, AssocCtxt),
    ForeignItem(Box<ast::ForeignItem>),
    Stmt(Box<ast::Stmt>),
    Expr(Box<ast::Expr>),
    Ty(Box<ast::Ty>), // Added
    Pat(Box<ast::Pat>), // Added
    Arm(ast::Arm),
    ExprField(ast::ExprField),
    PatField(ast::PatField),
    GenericParam(ast::GenericParam),
    Param(ast::Param),
    FieldDef(ast::FieldDef),
    Variant(ast::Variant),
    WherePredicate(ast::WherePredicate),
    Crate(ast::Crate),
}

impl Annotatable {
    /// Returns the `Span` (source code location) of the annotatable item.
    ///
    /// For n00bs: The `Span` tells you exactly where in your original code
    /// this piece of the AST came from. It's like a bookmark for the compiler.
    pub fn span(&self) -> Span {
        match self {
            Annotatable::Item(item) => item.span,
            Annotatable::AssocItem(assoc_item, _) => assoc_item.span,
            Annotatable::ForeignItem(foreign_item) => foreign_item.span,
            Annotatable::Stmt(stmt) => stmt.span,
            Annotatable::Expr(expr) => expr.span,
            Annotatable::Ty(ty) => ty.span, // Added
            Annotatable::Pat(pat) => pat.span, // Added
            Annotatable::Arm(arm) => arm.span,
            Annotatable::ExprField(field) => field.span,
            Annotatable::PatField(fp) => fp.pat.span,
            Annotatable::GenericParam(gp) => gp.ident.span,
            Annotatable::Param(p) => p.span,
            Annotatable::FieldDef(sf) => sf.span,
            Annotatable::Variant(v) => v.span,
            Annotatable::WherePredicate(wp) => wp.span,
            Annotatable::Crate(c) => c.spans.inner_span,
        }
    }

    /// Visits the attributes associated with this annotatable item.
    ///
    /// This method allows you to apply a function `f` to the `AttrVec` (list of attributes)
    /// of the underlying AST node.
    ///
    /// For n00bs: If you have a `#[derive(Debug)]` on a struct, this method lets you
    /// access that `#[derive(Debug)]` part and do something with it.
    pub fn visit_attrs(&mut self, f: impl FnOnce(&mut AttrVec)) {
        match self {
            Annotatable::Item(item) => item.visit_attrs(f),
            Annotatable::AssocItem(assoc_item, _) => assoc_item.visit_attrs(f),
            Annotatable::ForeignItem(foreign_item) => foreign_item.visit_attrs(f),
            Annotatable::Stmt(stmt) => stmt.visit_attrs(f),
            Annotatable::Expr(expr) => expr.visit_attrs(f),
            Annotatable::Ty(ty) => ty.visit_attrs(f), // Added
            Annotatable::Pat(pat) => pat.visit_attrs(f), // Added
            Annotatable::Arm(arm) => arm.visit_attrs(f),
            Annotatable::ExprField(field) => field.visit_attrs(f),
            Annotatable::PatField(fp) => fp.visit_attrs(f),
            Annotatable::GenericParam(gp) => gp.visit_attrs(f),
            Annotatable::Param(p) => p.visit_attrs(f),
            Annotatable::FieldDef(sf) => sf.visit_attrs(f),
            Annotatable::Variant(v) => v.visit_attrs(f),
            Annotatable::WherePredicate(wp) => wp.visit_attrs(f),
            Annotatable::Crate(c) => c.visit_attrs(f),
        }
    }

    /// Allows a visitor to traverse the annotatable item.
    ///
    /// This is part of the AST traversal mechanism, letting `Visitor` implementations
    /// inspect the structure of the AST.
    ///
    /// For n00bs: If you want to look through all the parts of your Rust code in a structured way,
    /// a "visitor" can walk this AST tree. This method lets the visitor step into this
    /// particular `Annotatable` part.
    pub fn visit_with<'a, V: Visitor<'a>>(&'a self, visitor: &mut V) -> V::Result {
        match self {
            Annotatable::Item(item) => visitor.visit_item(item),
            Annotatable::AssocItem(item, ctxt) => visitor.visit_assoc_item(item, *ctxt),
            Annotatable::ForeignItem(foreign_item) => visitor.visit_foreign_item(foreign_item),
            Annotatable::Stmt(stmt) => visitor.visit_stmt(stmt),
            Annotatable::Expr(expr) => visitor.visit_expr(expr),
            Annotatable::Ty(ty) => visitor.visit_ty(ty), // Added
            Annotatable::Pat(pat) => visitor.visit_pat(pat), // Added
            Annotatable::Arm(arm) => visitor.visit_arm(arm),
            Annotatable::ExprField(field) => visitor.visit_expr_field(field),
            Annotatable::PatField(fp) => visitor.visit_pat_field(fp),
            Annotatable::GenericParam(gp) => visitor.visit_generic_param(gp),
            Annotatable::Param(p) => visitor.visit_param(p),
            Annotatable::FieldDef(sf) => visitor.visit_field_def(sf),
            Annotatable::Variant(v) => visitor.visit_variant(v),
            Annotatable::WherePredicate(wp) => visitor.visit_where_predicate(wp),
            Annotatable::Crate(c) => visitor.visit_crate(c),
        }
    }

    /// Converts the annotatable item into a `TokenStream`.
    ///
    /// This is typically used when a macro needs to generate new code or
    /// transform existing code represented as a `TokenStream`.
    ///
    /// For n00bs: After a macro has done its work changing the AST, it often needs
    /// to turn that changed AST back into actual Rust code text that the compiler
    /// can understand. This method helps do that conversion.
    pub fn to_tokens(&self) -> TokenStream {
        match self {
            Annotatable::Item(node) => TokenStream::from_ast(node),
            Annotatable::AssocItem(node, _) => TokenStream::from_ast(node),
            Annotatable::ForeignItem(node) => TokenStream::from_ast(node),
            Annotatable::Stmt(node) => {
                assert!(!matches!(node.kind, ast::StmtKind::Empty));
                TokenStream::from_ast(node)
            }
            Annotatable::Expr(node) => TokenStream::from_ast(node),
            Annotatable::Ty(node) => TokenStream::from_ast(node), // Added
            Annotatable::Pat(node) => TokenStream::from_ast(node), // Added
            // The following types cannot be directly converted to TokenStream using `from_ast`
            // in a general way, as they are not top-level items or expressions.
            Annotatable::Arm(..)
            | Annotatable::ExprField(..)
            | Annotatable::PatField(..)
            | Annotatable::GenericParam(..)
            | Annotatable::Param(..)
            | Annotatable::FieldDef(..)
            | Annotatable::Variant(..)
            | Annotatable::WherePredicate(..)
            | Annotatable::Crate(..) => panic!("unexpected annotatable"),
        }
    }

    /// Tries to unwrap the `Annotatable` into a `Box<ast::Item>`, panicking if it's another type.
    pub fn expect_item(self) -> Box<ast::Item> {
        match self {
            Annotatable::Item(i) => i,
            _ => panic!("expected Item"),
        }
    }

    /// Tries to unwrap the `Annotatable` into a `Box<ast::AssocItem>` (trait item), panicking if it's another type.
    pub fn expect_assoc_item(self) -> Box<ast::AssocItem> {
        match self {
            Annotatable::AssocItem(i, _) => i,
            _ => panic!("expected Associated Item"),
        }
    }

    /// Tries to unwrap the `Annotatable` into a `Box<ast::ForeignItem>`, panicking if it's another type.
    pub fn expect_foreign_item(self) -> Box<ast::ForeignItem> {
        match self {
            Annotatable::ForeignItem(i) => i,
            _ => panic!("expected foreign item"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::Stmt`, panicking if it's another type.
    pub fn expect_stmt(self) -> ast::Stmt {
        match self {
            Annotatable::Stmt(stmt) => *stmt,
            _ => panic!("expected statement"),
        }
    }

    /// Tries to unwrap the `Annotatable` into a `Box<ast::Expr>`, panicking if it's another type.
    pub fn expect_expr(self) -> Box<ast::Expr> {
        match self {
            Annotatable::Expr(expr) => expr,
            _ => panic!("expected expression"),
        }
    }

    /// Tries to unwrap the `Annotatable` into a `Box<ast::Ty>`, panicking if it's another type.
    pub fn expect_ty(self) -> Box<ast::Ty> {
        match self {
            Annotatable::Ty(ty) => ty,
            _ => panic!("expected type"),
        }
    }

    /// Tries to unwrap the `Annotatable` into a `Box<ast::Pat>`, panicking if it's another type.
    pub fn expect_pat(self) -> Box<ast::Pat> {
        match self {
            Annotatable::Pat(pat) => pat,
            _ => panic!("expected pattern"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::Arm`, panicking if it's another type.
    pub fn expect_arm(self) -> ast::Arm {
        match self {
            Annotatable::Arm(arm) => arm,
            _ => panic!("expected match arm"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::ExprField`, panicking if it's another type.
    pub fn expect_expr_field(self) -> ast::ExprField {
        match self {
            Annotatable::ExprField(field) => field,
            _ => panic!("expected field"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::PatField`, panicking if it's another type.
    pub fn expect_pat_field(self) -> ast::PatField {
        match self {
            Annotatable::PatField(fp) => fp,
            _ => panic!("expected field pattern"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::GenericParam`, panicking if it's another type.
    pub fn expect_generic_param(self) -> ast::GenericParam {
        match self {
            Annotatable::GenericParam(gp) => gp,
            _ => panic!("expected generic parameter"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::Param`, panicking if it's another type.
    pub fn expect_param(self) -> ast::Param {
        match self {
            Annotatable::Param(param) => param,
            _ => panic!("expected parameter"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::FieldDef`, panicking if it's another type.
    pub fn expect_field_def(self) -> ast::FieldDef {
        match self {
            Annotatable::FieldDef(sf) => sf,
            _ => panic!("expected struct field"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::Variant`, panicking if it's another type.
    pub fn expect_variant(self) -> ast::Variant {
        match self {
            Annotatable::Variant(v) => v,
            _ => panic!("expected variant"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::WherePredicate`, panicking if it's another type.
    pub fn expect_where_predicate(self) -> ast::WherePredicate {
        match self {
            Annotatable::WherePredicate(wp) => wp,
            _ => panic!("expected where predicate"),
        }
    }

    /// Tries to unwrap the `Annotatable` into an `ast::Crate`, panicking if it's another type.
    pub fn expect_crate(self) -> ast::Crate {
        match self {
            Annotatable::Crate(krate) => krate,
            _ => panic!("expected krate"),
        }
    }
}
