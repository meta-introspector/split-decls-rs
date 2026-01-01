# AST Trace: ../rust/compiler/rustc_codegen_llvm/src/coverageinfo/ffi.rs

Generated 12 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::rustc_complete::mir::coverage::{CounterId, CovTerm, ExpressionId};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=9

```rust
/// Must match the layout of `LLVMRustCounterKind`.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) enum CounterKind {
    Zero = 0,
    CounterValueReference = 1,
    Expression = 2,
}
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=9 | LINES=20

```rust
/// A reference to an instance of an abstract "counter" that will yield a value in a coverage
/// report. Note that `id` has different interpretations, depending on the `kind`:
///   * For `CounterKind::Zero`, `id` is assumed to be `0`
///   * For `CounterKind::CounterValueReference`,  `id` matches the `counter_id` of the injected
///     instrumentation counter (the `index` argument to the LLVM intrinsic
///     `instrprof.increment()`)
///   * For `CounterKind::Expression`, `id` is the index into the coverage map's array of
///     counter expressions.
///
/// Corresponds to struct `llvm::coverage::Counter`.
///
/// Must match the layout of `LLVMRustCounter`.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct Counter {
    // Important: The layout (order and types of fields) must match its C++ counterpart.
    pub(crate) kind: CounterKind,
    id: u32,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=13 | LINES=23

```rust
impl Counter {
    /// A `Counter` of kind `Zero`. For this counter kind, the `id` is not used.
    pub(crate) const ZERO: Self = Self { kind: CounterKind::Zero, id: 0 };

    /// Constructs a new `Counter` of kind `CounterValueReference`.
    pub(crate) fn counter_value_reference(counter_id: CounterId) -> Self {
        Self { kind: CounterKind::CounterValueReference, id: counter_id.as_u32() }
    }

    /// Constructs a new `Counter` of kind `Expression`.
    pub(crate) fn expression(expression_id: ExpressionId) -> Self {
        Self { kind: CounterKind::Expression, id: expression_id.as_u32() }
    }

    pub(crate) fn from_term(term: CovTerm) -> Self {
        match term {
            CovTerm::Zero => Self::ZERO,
            CovTerm::Counter(id) => Self::counter_value_reference(id),
            CovTerm::Expression(id) => Self::expression(id),
        }
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=10

```rust
/// Corresponds to enum `llvm::coverage::CounterExpression::ExprKind`.
///
/// Must match the layout of `LLVMRustCounterExprKind`.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) enum ExprKind {
    Subtract = 0,
    Add = 1,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=11

```rust
/// Corresponds to struct `llvm::coverage::CounterExpression`.
///
/// Must match the layout of `LLVMRustCounterExpression`.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub(crate) struct CounterExpression {
    pub(crate) kind: ExprKind,
    pub(crate) lhs: Counter,
    pub(crate) rhs: Counter,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=21

```rust
/// A span of source code coordinates to be embedded in coverage metadata.
///
/// Must match the layout of `LLVMRustCoverageSpan`.
#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct CoverageSpan {
    /// Local index into the function's local-to-global file ID table.
    /// The value at that index is itself an index into the coverage filename
    /// table in the CGU's `__llvm_covmap` section.
    pub(crate) file_id: u32,

    /// 1-based starting line of the source code span.
    pub(crate) start_line: u32,
    /// 1-based starting column of the source code span.
    pub(crate) start_col: u32,
    /// 1-based ending line of the source code span.
    pub(crate) end_line: u32,
    /// 1-based ending column of the source code span. High bit must be unset.
    pub(crate) end_col: u32,
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=14

```rust
/// Holds tables of the various region types in one struct.
///
/// Don't pass this struct across FFI; pass the individual region tables as
/// pointer/length pairs instead.
///
/// Each field name has a `_regions` suffix for improved readability after
/// exhaustive destructing, which ensures that all region types are handled.
#[derive(Clone, Debug, Default)]
pub(crate) struct Regions {
    pub(crate) code_regions: Vec<CodeRegion>,
    pub(crate) expansion_regions: Vec<ExpansionRegion>,
    pub(crate) branch_regions: Vec<BranchRegion>,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=9

```rust
impl Regions {
    /// Returns true if none of this structure's tables contain any regions.
    pub(crate) fn has_no_regions(&self) -> bool {
        let Self { code_regions, expansion_regions, branch_regions } = self;

        code_regions.is_empty() && expansion_regions.is_empty() && branch_regions.is_empty()
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=8

```rust
/// Must match the layout of `LLVMRustCoverageCodeRegion`.
#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct CodeRegion {
    pub(crate) cov_span: CoverageSpan,
    pub(crate) counter: Counter,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=8

```rust
/// Must match the layout of `LLVMRustCoverageExpansionRegion`.
#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct ExpansionRegion {
    pub(crate) cov_span: CoverageSpan,
    pub(crate) expanded_file_id: u32,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=5 | LINES=9

```rust
/// Must match the layout of `LLVMRustCoverageBranchRegion`.
#[derive(Clone, Debug)]
#[repr(C)]
pub(crate) struct BranchRegion {
    pub(crate) cov_span: CoverageSpan,
    pub(crate) true_counter: Counter,
    pub(crate) false_counter: Counter,
}
```

---
*Generated by AST tracing system*
