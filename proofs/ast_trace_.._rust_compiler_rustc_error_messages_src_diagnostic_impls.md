# AST Trace: ../rust/compiler/rustc_error_messages/src/diagnostic_impls.rs

Generated 27 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use std::backtrace::Backtrace;
use std::borrow::Cow;
use std::fmt;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=7

```rust
use std::process::ExitStatus;

use rustc_ast as ast;
use rustc_ast_pretty::pprust;
use crate::rustc_complete::edition::Edition;

use crate::{DiagArgValue, IntoDiagArg};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=FUNCTION | NAME=DiagArgFromDisplay | COMPLEXITY=5 | LINES=8

```rust
pub struct DiagArgFromDisplay<'a>(pub &'a dyn fmt::Display);

impl IntoDiagArg for DiagArgFromDisplay<'_> {
    fn into_diag_arg(self, path: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        self.0.to_string().into_diag_arg(path)
    }
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl<'a> From<&'a dyn fmt::Display> for DiagArgFromDisplay<'a> {
    fn from(t: &'a dyn fmt::Display) -> Self {
        DiagArgFromDisplay(t)
    }
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=from | COMPLEXITY=5 | LINES=6

```rust
impl<'a, T: fmt::Display> From<&'a T> for DiagArgFromDisplay<'a> {
    fn from(t: &'a T) -> Self {
        DiagArgFromDisplay(t)
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl<'a, T: Clone + IntoDiagArg> IntoDiagArg for &'a T {
    fn into_diag_arg(self, path: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        self.clone().into_diag_arg(path)
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=12 | LINES=13

```rust
#[macro_export]
macro_rules! into_diag_arg_using_display {
    ($( $ty:ty ),+ $(,)?) => {
        $(
            impl $crate::IntoDiagArg for $ty {
                fn into_diag_arg(self, path: &mut Option<std::path::PathBuf>) -> $crate::DiagArgValue {
                    self.to_string().into_diag_arg(path)
                }
            }
        )+
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=19 | LINES=18

```rust
macro_rules! into_diag_arg_for_number {
    ($( $ty:ty ),+ $(,)?) => {
        $(
            impl $crate::IntoDiagArg for $ty {
                fn into_diag_arg(self, path: &mut Option<std::path::PathBuf>) -> $crate::DiagArgValue {
                    // Convert to a string if it won't fit into `Number`.
                    #[allow(irrefutable_let_patterns)]
                    if let Ok(n) = TryInto::<i32>::try_into(self) {
                        $crate::DiagArgValue::Number(n)
                    } else {
                        self.to_string().into_diag_arg(path)
                    }
                }
            }
        )+
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=10 | LINES=24

```rust
into_diag_arg_using_display!(
    ast::ParamKindOrd,
    std::io::Error,
    Box<dyn std::error::Error>,
    std::num::NonZero<u32>,
    Edition,
    crate::rustc_span::Ident,
    crate::rustc_span::MacroRulesNormalizedIdent,
    ParseIntError,
    ExitStatus,
);

into_diag_arg_for_number!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize);

impl IntoDiagArg for bool {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        if self {
            DiagArgValue::Str(Cow::Borrowed("true"))
        } else {
            DiagArgValue::Str(Cow::Borrowed("false"))
        }
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=6 | LINES=6

```rust
impl IntoDiagArg for char {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(format!("{self:?}")))
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=6 | LINES=8

```rust
impl IntoDiagArg for Vec<char> {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::StrListSepByAnd(
            self.into_iter().map(|c| Cow::Owned(format!("{c:?}"))).collect(),
        )
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for crate::rustc_span::Symbol {
    fn into_diag_arg(self, path: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        self.to_ident_string().into_diag_arg(path)
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl<'a> IntoDiagArg for &'a str {
    fn into_diag_arg(self, path: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        self.to_string().into_diag_arg(path)
    }
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for String {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self))
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl<'a> IntoDiagArg for Cow<'a, str> {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.into_owned()))
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl<'a> IntoDiagArg for &'a Path {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.display().to_string()))
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for PathBuf {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.display().to_string()))
    }
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for ast::Expr {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(pprust::expr_to_string(&self)))
    }
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for ast::Path {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(pprust::path_to_string(&self)))
    }
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for ast::token::Token {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(pprust::token_to_string(&self))
    }
}
```

## Block 21
**Metadata**: AST_ID=21 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for ast::token::TokenKind {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(pprust::token_kind_to_string(&self))
    }
}
```

## Block 22
**Metadata**: AST_ID=22 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for std::ffi::CString {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.to_string_lossy().into_owned()))
    }
}
```

## Block 23
**Metadata**: AST_ID=23 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for crate::rustc_data_structures::small_c_str::SmallCStr {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Owned(self.to_string_lossy().into_owned()))
    }
}
```

## Block 24
**Metadata**: AST_ID=24 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=8

```rust
impl IntoDiagArg for ast::Visibility {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        let s = pprust::vis_to_string(&self);
        let s = s.trim_end().to_string();
        DiagArgValue::Str(Cow::Owned(s))
    }
}
```

## Block 25
**Metadata**: AST_ID=25 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for Backtrace {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::from(self.to_string()))
    }
}
```

## Block 26
**Metadata**: AST_ID=26 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for ast::util::parser::ExprPrecedence {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Number(self as i32)
    }
}
```

## Block 27
**Metadata**: AST_ID=27 | TYPE=FUNCTION | NAME=into_diag_arg | COMPLEXITY=5 | LINES=6

```rust
impl IntoDiagArg for ast::FloatTy {
    fn into_diag_arg(self, _: &mut Option<std::path::PathBuf>) -> DiagArgValue {
        DiagArgValue::Str(Cow::Borrowed(self.name_str()))
    }
}
```

---
*Generated by AST tracing system*
