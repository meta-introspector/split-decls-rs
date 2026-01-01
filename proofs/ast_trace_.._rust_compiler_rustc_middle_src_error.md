# AST Trace: ../rust/compiler/rustc_middle/src/error.rs

Generated 24 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use std::path::Path;
use std::{fmt, io};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use rustc_errors::codes::*;
use rustc_errors::{DiagArgName, DiagArgValue, DiagMessage};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_macros::{Diagnostic, Subdiagnostic};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_span::{Span, Symbol};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use crate::ty::{Instance, Ty};
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(middle_drop_check_overflow, code = E0320)]
#[note]
pub(crate) struct DropCheckOverflow<'tcx> {
    #[primary_span]
    pub span: Span,
    pub ty: Ty<'tcx>,
    pub overflow_ty: Ty<'tcx>,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(middle_failed_writing_file)]
pub(crate) struct FailedWritingFile<'a> {
    pub path: &'a Path,
    pub error: io::Error,
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Diagnostic)]
#[diag(middle_opaque_hidden_type_mismatch)]
pub(crate) struct OpaqueHiddenTypeMismatch<'tcx> {
    pub self_ty: Ty<'tcx>,
    pub other_ty: Ty<'tcx>,
    #[primary_span]
    #[label]
    pub other_span: Span,
    #[subdiagnostic]
    pub sub: TypeMismatchReason,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UnsupportedUnion | COMPLEXITY=2 | LINES=7

```rust
// FIXME(autodiff): I should get used somewhere
#[derive(Diagnostic)]
#[diag(middle_unsupported_union)]
pub struct UnsupportedUnion {
    pub ty_name: String,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=AutodiffUnsafeInnerConstRef | COMPLEXITY=2 | LINES=9

```rust
// FIXME(autodiff): I should get used somewhere
#[derive(Diagnostic)]
#[diag(middle_autodiff_unsafe_inner_const_ref)]
pub struct AutodiffUnsafeInnerConstRef<'tcx> {
    #[primary_span]
    pub span: Span,
    pub ty: Ty<'tcx>,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=14

```rust
#[derive(Subdiagnostic)]
pub enum TypeMismatchReason {
    #[label(middle_conflict_types)]
    ConflictType {
        #[primary_span]
        span: Span,
    },
    #[note(middle_previous_use_here)]
    PreviousUse {
        #[primary_span]
        span: Span,
    },
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(middle_recursion_limit_reached)]
#[help]
pub(crate) struct RecursionLimitReached<'tcx> {
    pub ty: Ty<'tcx>,
    pub suggested_limit: rustc_hir::limit::Limit,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(middle_const_eval_non_int)]
pub(crate) struct ConstEvalNonIntError {
    #[primary_span]
    pub span: Span,
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(middle_strict_coherence_needs_negative_coherence)]
pub(crate) struct StrictCoherenceNeedsNegativeCoherence {
    #[primary_span]
    pub span: Span,
    #[label]
    pub attr_span: Option<Span>,
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(middle_requires_lang_item)]
pub(crate) struct RequiresLangItem {
    #[primary_span]
    pub span: Span,
    pub name: Symbol,
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(middle_const_not_used_in_type_alias)]
pub(super) struct ConstNotUsedTraitAlias {
    pub ct: String,
    #[primary_span]
    pub span: Span,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=STRUCT | NAME=CustomSubdiagnostic | COMPLEXITY=2 | LINES=5

```rust
pub struct CustomSubdiagnostic<'a> {
    pub msg: fn() -> DiagMessage,
    pub add_args: Box<dyn FnOnce(&mut dyn FnMut(DiagArgName, DiagArgValue)) + 'a>,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=label | COMPLEXITY=6 | LINES=12

```rust
impl<'a> CustomSubdiagnostic<'a> {
    pub fn label(x: fn() -> DiagMessage) -> Self {
        Self::label_and_then(x, |_| {})
    }
    pub fn label_and_then<F: FnOnce(&mut dyn FnMut(DiagArgName, DiagArgValue)) + 'a>(
        msg: fn() -> DiagMessage,
        f: F,
    ) -> Self {
        Self { msg, add_args: Box::new(move |x| f(x)) }
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=fmt | COMPLEXITY=5 | LINES=6

```rust
impl fmt::Debug for CustomSubdiagnostic<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CustomSubdiagnostic").finish_non_exhaustive()
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=6 | LINES=21

```rust
#[derive(Diagnostic)]
pub enum LayoutError<'tcx> {
    #[diag(middle_layout_unknown)]
    Unknown { ty: Ty<'tcx> },

    #[diag(middle_layout_too_generic)]
    TooGeneric { ty: Ty<'tcx> },

    #[diag(middle_layout_size_overflow)]
    Overflow { ty: Ty<'tcx> },

    #[diag(middle_layout_normalization_failure)]
    NormalizationFailure { ty: Ty<'tcx>, failure_ty: String },

    #[diag(middle_layout_cycle)]
    Cycle,

    #[diag(middle_layout_references_error)]
    ReferencesError,
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(middle_erroneous_constant)]
pub(crate) struct ErroneousConstant {
    #[primary_span]
    pub span: Span,
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=10

```rust
#[derive(Diagnostic)]
#[diag(middle_type_length_limit)]
#[help(middle_consider_type_length_limit)]
pub(crate) struct TypeLengthLimit<'tcx> {
    #[primary_span]
    pub span: Span,
    pub instance: Instance<'tcx>,
    pub type_length: usize,
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Diagnostic)]
#[diag(middle_max_num_nodes_in_valtree)]
pub(crate) struct MaxNumNodesInValtree {
    #[primary_span]
    pub span: Span,
    pub global_const_id: String,
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=9

```rust
#[derive(Diagnostic)]
#[diag(middle_invalid_const_in_valtree)]
#[note]
pub(crate) struct InvalidConstInValtree {
    #[primary_span]
    pub span: Span,
    pub global_const_id: String,
}
```

---
*Generated by AST tracing system*
