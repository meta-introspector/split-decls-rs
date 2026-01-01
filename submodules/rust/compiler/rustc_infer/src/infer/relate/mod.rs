// SRC: ../rust/compiler/rustc_infer/src/infer/relate/mod.rs
// This module contains the definitions of most `TypeRelation`s in the type system
// (except for some relations used for diagnostics and heuristics in the compiler).
// As well as the implementation of `Relate` for interned things (`Ty`/`Const`/etc).

pub use crate::rustc_complete::ty::relate::combine::PredicateEmittingRelation;
pub use crate::rustc_complete::ty::relate::{RelateResult, *};
