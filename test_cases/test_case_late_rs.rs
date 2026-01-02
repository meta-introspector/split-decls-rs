// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_resolve/src/late.rs
// Error: expected square brackets
// Problematic line: line 14

use std::collections::hash_map::Entry;
use std::mem::{replace, swap, take};

use rustc_ast::visit::{
    AssocCtxt, BoundKind, FnCtxt, FnKind, Visitor, try_visit, visit_opt, walk_list,
};
use rustc_ast::*;
