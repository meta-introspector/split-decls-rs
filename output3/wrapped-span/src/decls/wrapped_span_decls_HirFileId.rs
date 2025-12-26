use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Input to the analyzer is a set of files, where each file is identified by
/// `FileId` and contains source code. However, another source of source code in
/// Rust are macros: each macro can be thought of as producing a "temporary
/// file". To assign an id to such a file, we use the id of the macro call that
/// produced the file. So, a `HirFileId` is either a `FileId` (source code
/// written by user), or a `MacroCallId` (source code produced by macro).
///
/// What is a `MacroCallId`? Simplifying, it's a `HirFileId` of a file
/// containing the call plus the offset of the macro call in the file. Note that
/// this is a recursive definition! However, the size_of of `HirFileId` is
/// finite (because everything bottoms out at the real `FileId`) and small
/// (`MacroCallId` uses the location interning. You can check details here:
/// <https://en.wikipedia.org/wiki/String_interning>).
///
/// Internally this holds a `salsa::Id`, but we cannot use this definition here
/// as it references things from base-db and hir-expand.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HirFileId(pub salsa::Id);
