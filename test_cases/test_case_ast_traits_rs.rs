// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast/src/ast_traits.rs
// Error: expected square brackets
// Problematic line: line 9

use std::marker::PhantomData;

use crate::tokenstream::LazyAttrTokenStream;
use crate::{
    Arm, AssocItem, AttrItem, AttrKind, AttrVec, Attribute, Block, Crate, Expr, ExprField,
    FieldDef, ForeignItem, GenericParam, Item, NodeId, Param, Pat, PatField, Path, Stmt, StmtKind,
    Ty, Variant, Visibility, WherePredicate,
