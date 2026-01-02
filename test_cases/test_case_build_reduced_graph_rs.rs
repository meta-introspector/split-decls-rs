// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_resolve/src/build_reduced_graph.rs
// Error: expected square brackets
// Problematic line: line 12

use std::sync::Arc;

use rustc_ast::visit::{self, AssocCtxt, Visitor, WalkItemKind};
use rustc_ast::{
    self as ast, AssocItem, AssocItemKind, Block, ConstItem, Delegation, Fn, ForeignItem,
    ForeignItemKind, Inline, Item, ItemKind, NodeId, StaticItem, StmtKind, TyAlias,
};
