# AST Trace: ../rust/compiler/rustc_incremental/src/errors.rs

Generated 41 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use std::path::{Path, PathBuf};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use rustc_macros::Diagnostic;
use rustc_span::{Ident, Span, Symbol};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_unrecognized_depnode)]
pub(crate) struct UnrecognizedDepNode {
    #[primary_span]
    pub span: Span,
    pub name: Symbol,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_missing_depnode)]
pub(crate) struct MissingDepNode {
    #[primary_span]
    pub span: Span,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_missing_if_this_changed)]
pub(crate) struct MissingIfThisChanged {
    #[primary_span]
    pub span: Span,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_ok)]
pub(crate) struct Ok {
    #[primary_span]
    pub span: Span,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(incremental_no_path)]
pub(crate) struct NoPath {
    #[primary_span]
    pub span: Span,
    pub target: Symbol,
    pub source: String,
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(incremental_assertion_auto)]
pub(crate) struct AssertionAuto<'a> {
    #[primary_span]
    pub span: Span,
    pub name: &'a str,
    pub e: &'a str,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_undefined_clean_dirty_assertions_item)]
pub(crate) struct UndefinedCleanDirtyItem {
    #[primary_span]
    pub span: Span,
    pub kind: String,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_undefined_clean_dirty_assertions)]
pub(crate) struct UndefinedCleanDirty {
    #[primary_span]
    pub span: Span,
    pub kind: String,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_repeated_depnode_label)]
pub(crate) struct RepeatedDepNodeLabel<'a> {
    #[primary_span]
    pub span: Span,
    pub label: &'a str,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_unrecognized_depnode_label)]
pub(crate) struct UnrecognizedDepNodeLabel<'a> {
    #[primary_span]
    pub span: Span,
    pub label: &'a str,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_not_dirty)]
pub(crate) struct NotDirty<'a> {
    #[primary_span]
    pub span: Span,
    pub dep_node_str: &'a str,
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_not_clean)]
pub(crate) struct NotClean<'a> {
    #[primary_span]
    pub span: Span,
    pub dep_node_str: &'a str,
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_not_loaded)]
pub(crate) struct NotLoaded<'a> {
    #[primary_span]
    pub span: Span,
    pub dep_node_str: &'a str,
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_unknown_rustc_clean_argument)]
pub(crate) struct UnknownRustcCleanArgument {
    #[primary_span]
    pub span: Span,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_no_cfg)]
pub(crate) struct NoCfg {
    #[primary_span]
    pub span: Span,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_associated_value_expected_for)]
pub(crate) struct AssociatedValueExpectedFor {
    #[primary_span]
    pub span: Span,
    pub ident: Ident,
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_associated_value_expected)]
pub(crate) struct AssociatedValueExpected {
    #[primary_span]
    pub span: Span,
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_unchecked_clean)]
pub(crate) struct UncheckedClean {
    #[primary_span]
    pub span: Span,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_delete_old)]
pub(crate) struct DeleteOld<'a> {
    pub name: &'a str,
    pub path: PathBuf,
    pub err: std::io::Error,
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_create_new)]
pub(crate) struct CreateNew<'a> {
    pub name: &'a str,
    pub path: PathBuf,
    pub err: std::io::Error,
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_write_new)]
pub(crate) struct WriteNew<'a> {
    pub name: &'a str,
    pub path: PathBuf,
    pub err: std::io::Error,
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_canonicalize_path)]
pub(crate) struct CanonicalizePath {
    pub path: PathBuf,
    pub err: std::io::Error,
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_create_incr_comp_dir)]
pub(crate) struct CreateIncrCompDir<'a> {
    pub tag: &'a str,
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Diagnostic)]
#[diag(incremental_create_lock)]
pub(crate) struct CreateLock<'a> {
    pub lock_err: std::io::Error,
    pub session_dir: &'a Path,
    #[note(incremental_lock_unsupported)]
    pub is_unsupported_lock: bool,
    #[help(incremental_cargo_help_1)]
    #[help(incremental_cargo_help_2)]
    pub is_cargo: bool,
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_delete_lock)]
pub(crate) struct DeleteLock<'a> {
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 28
**Metadata**: AST_ID=28 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Diagnostic)]
#[diag(incremental_hard_link_failed)]
pub(crate) struct HardLinkFailed<'a> {
    pub path: &'a Path,
}
```

## Block 29
**Metadata**: AST_ID=29 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_delete_partial)]
pub(crate) struct DeletePartial<'a> {
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 30
**Metadata**: AST_ID=30 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_delete_full)]
pub(crate) struct DeleteFull<'a> {
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 31
**Metadata**: AST_ID=31 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_finalize)]
pub(crate) struct Finalize<'a> {
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 32
**Metadata**: AST_ID=32 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_invalid_gc_failed)]
pub(crate) struct InvalidGcFailed<'a> {
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 33
**Metadata**: AST_ID=33 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_finalized_gc_failed)]
pub(crate) struct FinalizedGcFailed<'a> {
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 34
**Metadata**: AST_ID=34 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_session_gc_failed)]
pub(crate) struct SessionGcFailed<'a> {
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 35
**Metadata**: AST_ID=35 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=15

```rust
#[derive(Diagnostic)]
#[diag(incremental_assert_not_loaded)]
pub(crate) struct AssertNotLoaded;

#[derive(Diagnostic)]
#[diag(incremental_assert_loaded)]
pub(crate) struct AssertLoaded;

#[derive(Diagnostic)]
#[diag(incremental_delete_incompatible)]
pub(crate) struct DeleteIncompatible {
    pub path: PathBuf,
    pub err: std::io::Error,
}
```

## Block 36
**Metadata**: AST_ID=36 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_load_dep_graph)]
pub(crate) struct LoadDepGraph {
    pub path: PathBuf,
    pub err: std::io::Error,
}
```

## Block 37
**Metadata**: AST_ID=37 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_move_dep_graph)]
pub(crate) struct MoveDepGraph<'a> {
    pub from: &'a Path,
    pub to: &'a Path,
    pub err: std::io::Error,
}
```

## Block 38
**Metadata**: AST_ID=38 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_create_dep_graph)]
pub(crate) struct CreateDepGraph<'a> {
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 39
**Metadata**: AST_ID=39 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(incremental_copy_workproduct_to_cache)]
pub(crate) struct CopyWorkProductToCache<'a> {
    pub from: &'a Path,
    pub to: &'a Path,
    pub err: std::io::Error,
}
```

## Block 40
**Metadata**: AST_ID=40 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(incremental_delete_workproduct)]
pub(crate) struct DeleteWorkProduct<'a> {
    pub path: &'a Path,
    pub err: std::io::Error,
}
```

## Block 41
**Metadata**: AST_ID=41 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Diagnostic)]
#[diag(incremental_corrupt_file)]
pub(crate) struct CorruptFile<'a> {
    pub path: &'a Path,
}
```

---
*Generated by AST tracing system*
