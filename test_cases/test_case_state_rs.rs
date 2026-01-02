// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast_pretty/src/pprust/state.rs
// Error: expected square brackets
// Problematic line: line 17

use rustc_ast::tokenstream::{Spacing, TokenStream, TokenTree};
use rustc_ast::util::classify;
use rustc_ast::util::comments::{Comment, CommentStyle};
use rustc_ast::{
    self as ast, AttrArgs, BindingMode, BlockCheckMode, ByRef, DelimArgs, GenericArg, GenericBound,
    InlineAsmOperand, InlineAsmOptions, InlineAsmRegOrRegClass, InlineAsmTemplatePiece, PatKind,
    RangeEnd, RangeSyntax, Safety, SelfKind, Term, attr,
