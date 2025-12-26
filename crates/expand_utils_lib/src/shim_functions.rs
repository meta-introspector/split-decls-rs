// In expand_utils_lib/src/shim_functions.rs

use rustc_ast::ast;
use rustc_ast::NodeId;
use rustc_span::Span;
use rustc_span::DUMMY_SP;
use rustc_errors::ErrorGuaranteed;
use rustc_expand_base_lib::prelude::*;
use rustc_ast::mut_visit::MutVisitor; // For shim_walk_stmt
use crate::{AstFragment, AstFragmentKind}; // Import from our lib.rs
//use crate::MacResult; // Need to import MacResult explicitly for placeholder's make_* methods.
//use crate::DummyResult; // Need to import DummyResult explicitly for placeholder's dummy method.

// shim_walk_stmt function
// Original definition in rustc_expand/src/ast_fragments_split/ast_fragments_enums.rs
pub fn shim_walk_stmt<V: MutVisitor + ?Sized>(visitor: &mut V, stmt: &mut ast::Stmt) {
    // Temporarily commenting out NodeId::PLACEHOLDER line to address E0599 separately
    let old_stmt = std::mem::replace(stmt, ast::Stmt { id: /* ast::NodeId::PLACEHOLDER */ ast::DUMMY_NODE_ID, span: DUMMY_SP, kind: ast::StmtKind::Empty });
    let new_stmts = visitor.flat_map_stmt(old_stmt);
    *stmt = new_stmts.into_iter().next().expect("flat_map_stmt unexpectedly returned no statements for shim_walk_stmt. A deeper refactoring of `rustc_expand` is likely required.");
}

// add_placeholders_to_fragment function
// Original definition in rustc_expand/src/ast_fragments_split/ast_fragments_enums.rs
pub fn add_placeholders_to_fragment(fragment: &mut AstFragment, placeholders: &[NodeId]) {
    if placeholders.is_empty() {
        return;
    }

    match fragment {
        AstFragment::Items(items) => {
            items.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::Items, *id, None).make_items().unwrap()
            }));
        },
        AstFragment::TraitItems(items) => {
            items.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::TraitItems, *id, None).make_trait_items().unwrap()
            }));
        },
        AstFragment::ImplItems(items) => {
            items.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::ImplItems, *id, None).make_impl_items().unwrap()
            }));
        },
        AstFragment::TraitImplItems(items) => {
            items.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::TraitImplItems, *id, None).make_trait_impl_items().unwrap()
            }));
        },
        AstFragment::ForeignItems(items) => {
            items.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::ForeignItems, *id, None).make_foreign_items().unwrap()
            }));
        },
        AstFragment::Stmts(stmts) => {
            stmts.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::Stmts, *id, None).make_stmts().unwrap()
            }));
        },
        AstFragment::Arms(arms) => {
            arms.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::Arms, *id, None).make_arms().unwrap()
            }));
        },
        AstFragment::ExprFields(fields) => {
            fields.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::ExprFields, *id, None).make_expr_fields().unwrap()
            }));
        },
        AstFragment::PatFields(fields) => {
            fields.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::PatFields, *id, None).make_pat_fields().unwrap()
            }));
        },
        AstFragment::GenericParams(params) => {
            params.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::GenericParams, *id, None).make_generic_params().unwrap()
            }));
        },
        AstFragment::Params(params) => {
            params.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::Params, *id, None).make_params().unwrap()
            }));
        },
        AstFragment::FieldDefs(fields) => {
            fields.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::FieldDefs, *id, None).make_field_defs().unwrap()
            }));
        },
        AstFragment::Variants(variants) => {
            variants.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::Variants, *id, None).make_variants().unwrap()
            }));
        },
        AstFragment::WherePredicates(predicates) => {
            predicates.extend(placeholders.iter().flat_map(|id| {
                placeholder(AstFragmentKind::WherePredicates, *id, None).make_where_predicates().unwrap()
            }));
        },
        // For single-item fragments, the original macro probably didn't generate `extend`
        // or it would panic. We'll panic here for now, matching the original panic.
        _ => panic!("unexpected AST fragment kind in add_placeholders_to_fragment"),
    }
}
