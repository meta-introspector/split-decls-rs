// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_attr_parsing/src/validate_attr.rs
// Error: expected square brackets
// Problematic line: line 7


use rustc_ast::token::Delimiter;
use rustc_ast::tokenstream::DelimSpan;
use rustc_ast::{
    self as ast, AttrArgs, Attribute, DelimArgs, MetaItem, MetaItemInner, MetaItemKind, NodeId,
    Path, Safety,
};
