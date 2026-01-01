# AST Trace: ../rust/compiler/rustc_ty_utils/src/errors.rs

Generated 11 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
// Errors emitted by ty_utils

use crate::rustc_complete::codes::*;
use rustc_macros::{Diagnostic, Subdiagnostic};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::rustc_complete::ty::{GenericArg, Ty};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
use crate::rustc_complete::Span;

#[derive(Diagnostic)]
#[diag(ty_utils_needs_drop_overflow)]
pub(crate) struct NeedsDropOverflow<'tcx> {
    pub query_ty: Ty<'tcx>,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=12

```rust
#[derive(Diagnostic)]
#[diag(ty_utils_generic_constant_too_complex)]
#[help]
pub(crate) struct GenericConstantTooComplex {
    #[primary_span]
    pub span: Span,
    #[note(ty_utils_maybe_supported)]
    pub maybe_supported: bool,
    #[subdiagnostic]
    pub sub: GenericConstantTooComplexSub,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=6 | LINES=48

```rust
#[derive(Subdiagnostic)]
pub(crate) enum GenericConstantTooComplexSub {
    #[label(ty_utils_borrow_not_supported)]
    BorrowNotSupported(#[primary_span] Span),
    #[label(ty_utils_address_and_deref_not_supported)]
    AddressAndDerefNotSupported(#[primary_span] Span),
    #[label(ty_utils_array_not_supported)]
    ArrayNotSupported(#[primary_span] Span),
    #[label(ty_utils_block_not_supported)]
    BlockNotSupported(#[primary_span] Span),
    #[label(ty_utils_never_to_any_not_supported)]
    NeverToAnyNotSupported(#[primary_span] Span),
    #[label(ty_utils_tuple_not_supported)]
    TupleNotSupported(#[primary_span] Span),
    #[label(ty_utils_index_not_supported)]
    IndexNotSupported(#[primary_span] Span),
    #[label(ty_utils_field_not_supported)]
    FieldNotSupported(#[primary_span] Span),
    #[label(ty_utils_const_block_not_supported)]
    ConstBlockNotSupported(#[primary_span] Span),
    #[label(ty_utils_adt_not_supported)]
    AdtNotSupported(#[primary_span] Span),
    #[label(ty_utils_pointer_not_supported)]
    PointerNotSupported(#[primary_span] Span),
    #[label(ty_utils_yield_not_supported)]
    YieldNotSupported(#[primary_span] Span),
    #[label(ty_utils_loop_not_supported)]
    LoopNotSupported(#[primary_span] Span),
    #[label(ty_utils_box_not_supported)]
    BoxNotSupported(#[primary_span] Span),
    #[label(ty_utils_binary_not_supported)]
    BinaryNotSupported(#[primary_span] Span),
    #[label(ty_utils_by_use_not_supported)]
    ByUseNotSupported(#[primary_span] Span),
    #[label(ty_utils_logical_op_not_supported)]
    LogicalOpNotSupported(#[primary_span] Span),
    #[label(ty_utils_assign_not_supported)]
    AssignNotSupported(#[primary_span] Span),
    #[label(ty_utils_closure_and_return_not_supported)]
    ClosureAndReturnNotSupported(#[primary_span] Span),
    #[label(ty_utils_control_flow_not_supported)]
    ControlFlowNotSupported(#[primary_span] Span),
    #[label(ty_utils_inline_asm_not_supported)]
    InlineAsmNotSupported(#[primary_span] Span),
    #[label(ty_utils_operation_not_supported)]
    OperationNotSupported(#[primary_span] Span),
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(ty_utils_unexpected_fnptr_associated_item)]
pub(crate) struct UnexpectedFnPtrAssociatedItem {
    #[primary_span]
    pub span: Span,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
#[derive(Diagnostic)]
#[diag(ty_utils_zero_length_simd_type)]
pub(crate) struct ZeroLengthSimdType<'tcx> {
    pub ty: Ty<'tcx>,
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(ty_utils_oversized_simd_type)]
pub(crate) struct OversizedSimdType<'tcx> {
    pub ty: Ty<'tcx>,
    pub max_lanes: u64,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
#[derive(Diagnostic)]
#[diag(ty_utils_non_primitive_simd_type)]
pub(crate) struct NonPrimitiveSimdType<'tcx> {
    pub ty: Ty<'tcx>,
    pub e_ty: Ty<'tcx>,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(ty_utils_impl_trait_duplicate_arg)]
pub(crate) struct DuplicateArg<'tcx> {
    pub arg: GenericArg<'tcx>,
    #[primary_span]
    #[label]
    pub span: Span,
    #[note]
    pub opaque_span: Span,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=11

```rust
#[derive(Diagnostic)]
#[diag(ty_utils_impl_trait_not_param, code = E0792)]
pub(crate) struct NotParam<'tcx> {
    pub arg: GenericArg<'tcx>,
    #[primary_span]
    #[label]
    pub span: Span,
    #[note]
    pub opaque_span: Span,
}
```

---
*Generated by AST tracing system*
