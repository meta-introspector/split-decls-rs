use rustc_ast::{self as ast, Attribute, AttrVec, HasAttrs, HasNodeId, NodeId, mut_visit::MutVisitor};
use rustc_span::{Ident, Span, Symbol};
use smallvec::SmallVec;
use crate::prelude::{Annotatable, AstFragment, AstFragmentKind, AddSemicolon, StripUnconfigured};
use crate::ast_traits::{MacroInvocationNode, AstFragmentConverter, WalkableAstNode, CfgFalseExpandable, HasDeclaredIdents, FlattensOutputs, AttributeHooks, CfgFalseReporterContext};
use crate::invocation_collector_node::InvocationCollectorNode; // Added this import
use rustc_session::lint::Lint;
use rustc_errors::Diagnostic;


// Moved from invocation_collector_node.rs
pub struct ExpandedStmt(pub ast::Stmt);

// Define a new trait to expose visit_attrs specifically for ExpandedStmt
pub trait ExpandedStmtHasAttrs {
    fn visit_stmt_attrs(&mut self, f: impl FnOnce(&mut AttrVec));
}

// Implement the new trait for ExpandedStmt
impl ExpandedStmtHasAttrs for ExpandedStmt {
    fn visit_stmt_attrs(&mut self, f: impl FnOnce(&mut AttrVec)) {
        match &mut self.0.kind {
            ast::StmtKind::Let(local) => f(&mut local.attrs),
            ast::StmtKind::Item(item) => f(&mut item.attrs),
            ast::StmtKind::Expr(expr) | ast::StmtKind::Semi(expr) => f(&mut expr.attrs),
            ast::StmtKind::MacCall(mac) => f(&mut mac.attrs),
            ast::StmtKind::Empty => {},
        }
    }
}

// Moved from invocation_collector_node.rs
impl HasNodeId for ExpandedStmt {
    fn node_id(&self) -> NodeId {
        self.0.id
    }
    fn node_id_mut(&mut self) -> &mut NodeId {
        &mut self.0.id
    }
}

// ... (rest of the code) ...

impl CfgFalseExpandable for ExpandedStmt {
    fn expand_cfg_false<D: CfgFalseReporterContext>(&mut self, collector: &mut D, pos: usize, span: Span) {
        // Handle cfg_false logic for statement
        // Temporarily comment out the problematic part related to external `errors` module.
        // This needs further abstraction in CfgFalseReporterContext or a dedicated error type
        // within rustc_expand_base_lib.
        /*
        collector.buffer_lint_unused_attribute(
            collector.get_unused_attribute_lint(true),
            span,
            collector.get_lint_node_id(),
            errors::UnusedBuiltinAttribute {
                invoc_span: span,
                attr_name: collector.get_sym_cfg(),
                macro_name: "expansion".to_string(),
                attr_span: span,
            },
        );
        */
        self.visit_stmt_attrs(|attrs| { attrs.remove(pos); });
    }
}

