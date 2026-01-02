// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_parse/src/parser/stmt.rs
// Error: expected `,`
// Error type: expected_comma
// Sample #3 of 3
// Problematic line: line 10

use rustc_ast::token::{self, Delimiter, InvisibleOrigin, MetaVarKind, TokenKind};
use rustc_ast::util::classify::{self, TrailingBrace};
use rustc_ast::visit::{Visitor, walk_expr};
use rustc_ast::{
    AttrStyle, AttrVec, Block, BlockCheckMode, DUMMY_NODE_ID, Expr, ExprKind, HasAttrs, Local,
    LocalKind, MacCall, MacCallStmt, MacStmtStyle, Recovered, Stmt, StmtKind,
};
