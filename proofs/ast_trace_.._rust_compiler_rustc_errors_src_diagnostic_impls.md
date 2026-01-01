# AST Trace: ../rust/compiler/rustc_errors/src/diagnostic_impls.rs

Generated 13 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
use std::borrow::Cow;

use rustc_abi::TargetDataLayoutErrors;
use rustc_error_messages::{DiagArgValue, IntoDiagArg};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use rustc_macros::Subdiagnostic;
use rustc_span::{Span, Symbol};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6

```rust
use crate::diagnostic::DiagLocation;
use crate::{
    Diag, DiagCtxtHandle, Diagnostic, EmissionGuarantee, Level, Subdiagnostic,
    fluent_generated as fluent,
};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for DiagLocation {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::from(self.to_string()))
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=DiagSymbolList | COMPLEXITY=5 | LINES=9

```rust
#[derive(Clone)]
pub struct DiagSymbolList<S = Symbol>(Vec<S>);

impl<S> From<Vec<S>> for DiagSymbolList<S> {
    fn from(v: Vec<S>) -> Self {
        DiagSymbolList(v)
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=from_iter | COMPLEXITY=5 | LINES=6

```rust
impl<S> FromIterator<S> for DiagSymbolList<S> {
    fn from_iter<T: IntoIterator<Item = S>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=6 | LINES=8

```rust
impl<S: std::fmt::Display> IntoDiagArg for DiagSymbolList<S> {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::StrListSepByAnd(
            self.0.into_iter().map(|sym| Cow::Owned(format!("`{sym}`"))).collect(),
        )
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=into_diag | COMPLEXITY=29 | LINES=47

```rust
impl<G: EmissionGuarantee> Diagnostic<'_, G> for TargetDataLayoutErrors<'_> {
    fn into_diag(self, dcx: DiagCtxtHandle<'_>, level: Level) -> Diag<'_, G> {
        match self {
            TargetDataLayoutErrors::InvalidAddressSpace { addr_space, err, cause } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_address_space)
                    .with_arg("addr_space", addr_space)
                    .with_arg("cause", cause)
                    .with_arg("err", err)
            }
            TargetDataLayoutErrors::InvalidBits { kind, bit, cause, err } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_bits)
                    .with_arg("kind", kind)
                    .with_arg("bit", bit)
                    .with_arg("cause", cause)
                    .with_arg("err", err)
            }
            TargetDataLayoutErrors::MissingAlignment { cause } => {
                Diag::new(dcx, level, fluent::errors_target_missing_alignment)
                    .with_arg("cause", cause)
            }
            TargetDataLayoutErrors::InvalidAlignment { cause, err } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_alignment)
                    .with_arg("cause", cause)
                    .with_arg("err_kind", err.diag_ident())
                    .with_arg("align", err.align())
            }
            TargetDataLayoutErrors::InconsistentTargetArchitecture { dl, target } => {
                Diag::new(dcx, level, fluent::errors_target_inconsistent_architecture)
                    .with_arg("dl", dl)
                    .with_arg("target", target)
            }
            TargetDataLayoutErrors::InconsistentTargetPointerWidth { pointer_size, target } => {
                Diag::new(dcx, level, fluent::errors_target_inconsistent_pointer_width)
                    .with_arg("pointer_size", pointer_size)
                    .with_arg("target", target)
            }
            TargetDataLayoutErrors::InvalidBitsSize { err } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_bits_size).with_arg("err", err)
            }
            TargetDataLayoutErrors::UnknownPointerSpecification { err } => {
                Diag::new(dcx, level, fluent::errors_target_invalid_datalayout_pointer_spec)
                    .with_arg("err", err)
            }
        }
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=STRUCT | NAME=SingleLabelManySpans | COMPLEXITY=4 | LINES=6

```rust
/// Utility struct used to apply a single label while highlighting multiple spans
pub struct SingleLabelManySpans {
    pub spans: Vec<Span>,
    pub label: &'static str,
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=add_to_diag | COMPLEXITY=5 | LINES=5

```rust
impl Subdiagnostic for SingleLabelManySpans {
    fn add_to_diag<G: EmissionGuarantee>(self, diag: &mut Diag<'_, G>) {
        diag.span_labels(self.spans, self.label);
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=ExpectedLifetimeParameter | COMPLEXITY=2 | LINES=8

```rust
#[derive(Subdiagnostic)]
#[label(errors_expected_lifetime_parameter)]
pub struct ExpectedLifetimeParameter {
    #[primary_span]
    pub span: Span,
    pub count: usize,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=STRUCT | NAME=IndicateAnonymousLifetime | COMPLEXITY=3 | LINES=9

```rust
#[derive(Subdiagnostic)]
#[suggestion(errors_indicate_anonymous_lifetime, code = "{suggestion}", style = "verbose")]
pub struct IndicateAnonymousLifetime {
    #[primary_span]
    pub span: Span,
    pub count: usize,
    pub suggestion: String,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=STRUCT | NAME=ElidedLifetimeInPathSubdiag | COMPLEXITY=2 | LINES=8

```rust
#[derive(Subdiagnostic)]
pub struct ElidedLifetimeInPathSubdiag {
    #[subdiagnostic]
    pub expected: ExpectedLifetimeParameter,
    #[subdiagnostic]
    pub indicate: Option<IndicateAnonymousLifetime>,
}
```

---
*Generated by AST tracing system*
