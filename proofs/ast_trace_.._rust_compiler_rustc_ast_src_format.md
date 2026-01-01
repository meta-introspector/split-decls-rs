# AST Trace: ../rust/compiler/rustc_ast/src/format.rs

Generated 18 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2

```rust
use rustc_data_structures::fx::FxHashMap;
use rustc_macros::{Decodable, Encodable, Walkable};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_span::{Ident, Span, Symbol};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=STRUCT | NAME=FormatArgs | COMPLEXITY=18 | LINES=57

```rust
use crate::Expr;
use crate::token::LitKind;

// Definitions:
//
// format_args!("hello {abc:.xyz$}!!", abc="world");
// └──────────────────────────────────────────────┘
//                     FormatArgs
//
// format_args!("hello {abc:.xyz$}!!", abc="world");
//                                     └─────────┘
//                                      argument
//
// format_args!("hello {abc:.xyz$}!!", abc="world");
//              └───────────────────┘
//                     template
//
// format_args!("hello {abc:.xyz$}!!", abc="world");
//               └────┘└─────────┘└┘
//                      pieces
//
// format_args!("hello {abc:.xyz$}!!", abc="world");
//               └────┘           └┘
//                   literal pieces
//
// format_args!("hello {abc:.xyz$}!!", abc="world");
//                     └─────────┘
//                     placeholder
//
// format_args!("hello {abc:.xyz$}!!", abc="world");
//                      └─┘  └─┘
//                      positions (could be names, numbers, empty, or `*`)

/// (Parsed) format args.
///
/// Basically the "AST" for a complete `format_args!()`.
///
/// E.g., `format_args!("hello {name}");`.
#[derive(Clone, Encodable, Decodable, Debug, Walkable)]
pub struct FormatArgs {
    pub span: Span,
    pub template: Vec<FormatArgsPiece>,
    pub arguments: FormatArguments,
    /// The raw, un-split format string literal, with no escaping or processing.
    ///
    /// Generally only useful for lints that care about the raw bytes the user wrote.
    pub uncooked_fmt_str: (LitKind, Symbol),
    /// Was the format literal written in the source?
    /// - `format!("boo")` => true,
    /// - `format!(concat!("b", "o", "o"))` => false,
    /// - `format!(include_str!("boo.txt"))` => false,
    ///
    /// If it wasn't written in the source then we have to be careful with spans pointing into it
    /// and suggestions about rewriting it.
    pub is_source_literal: bool,
}
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=9

```rust
/// A piece of a format template string.
///
/// E.g. "hello" or "{name}".
#[derive(Clone, Encodable, Decodable, Debug, Walkable)]
pub enum FormatArgsPiece {
    Literal(Symbol),
    Placeholder(FormatPlaceholder),
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=STRUCT | NAME=FormatArguments | COMPLEXITY=3 | LINES=12

```rust
/// The arguments to format_args!().
///
/// E.g. `1, 2, name="ferris", n=3`,
/// but also implicit captured arguments like `x` in `format_args!("{x}")`.
#[derive(Clone, Encodable, Decodable, Debug, Walkable)]
pub struct FormatArguments {
    arguments: Vec<FormatArgument>,
    num_unnamed_args: usize,
    num_explicit_args: usize,
    names: FxHashMap<Symbol, usize>,
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=FUNCTION | NAME=new | COMPLEXITY=24 | LINES=63

```rust
impl FormatArguments {
    pub fn new() -> Self {
        Self {
            arguments: Vec::new(),
            names: FxHashMap::default(),
            num_unnamed_args: 0,
            num_explicit_args: 0,
        }
    }

    pub fn add(&mut self, arg: FormatArgument) -> usize {
        let index = self.arguments.len();
        if let Some(name) = arg.kind.ident() {
            self.names.insert(name.name, index);
        } else if self.names.is_empty() {
            // Only count the unnamed args before the first named arg.
            // (Any later ones are errors.)
            self.num_unnamed_args += 1;
        }
        if !matches!(arg.kind, FormatArgumentKind::Captured(..)) {
            // This is an explicit argument.
            // Make sure that all arguments so far are explicit.
            assert_eq!(
                self.num_explicit_args,
                self.arguments.len(),
                "captured arguments must be added last"
            );
            self.num_explicit_args += 1;
        }
        self.arguments.push(arg);
        index
    }

    pub fn by_name(&self, name: Symbol) -> Option<(usize, &FormatArgument)> {
        let i = *self.names.get(&name)?;
        Some((i, &self.arguments[i]))
    }

    pub fn by_index(&self, i: usize) -> Option<&FormatArgument> {
        (i < self.num_explicit_args).then(|| &self.arguments[i])
    }

    pub fn unnamed_args(&self) -> &[FormatArgument] {
        &self.arguments[..self.num_unnamed_args]
    }

    pub fn named_args(&self) -> &[FormatArgument] {
        &self.arguments[self.num_unnamed_args..self.num_explicit_args]
    }

    pub fn explicit_args(&self) -> &[FormatArgument] {
        &self.arguments[..self.num_explicit_args]
    }

    pub fn all_args(&self) -> &[FormatArgument] {
        &self.arguments[..]
    }

    pub fn all_args_mut(&mut self) -> &mut Vec<FormatArgument> {
        &mut self.arguments
    }
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=STRUCT | NAME=FormatArgument | COMPLEXITY=2 | LINES=6

```rust
#[derive(Clone, Encodable, Decodable, Debug, Walkable)]
pub struct FormatArgument {
    pub kind: FormatArgumentKind,
    pub expr: Box<Expr>,
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=10

```rust
#[derive(Clone, Encodable, Decodable, Debug, Walkable)]
pub enum FormatArgumentKind {
    /// `format_args(…, arg)`
    Normal,
    /// `format_args(…, arg = 1)`
    Named(Ident),
    /// `format_args("… {arg} …")`
    Captured(Ident),
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=ident | COMPLEXITY=7 | LINES=10

```rust
impl FormatArgumentKind {
    pub fn ident(&self) -> Option<Ident> {
        match self {
            &Self::Normal => None,
            &Self::Named(id) => Some(id),
            &Self::Captured(id) => Some(id),
        }
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=STRUCT | NAME=FormatPlaceholder | COMPLEXITY=11 | LINES=14

```rust
#[derive(Clone, Encodable, Decodable, Debug, PartialEq, Eq, Walkable)]
pub struct FormatPlaceholder {
    /// Index into [`FormatArgs::arguments`].
    pub argument: FormatArgPosition,
    /// The span inside the format string for the full `{…}` placeholder.
    pub span: Option<Span>,
    /// `{}`, `{:?}`, or `{:x}`, etc.
    #[visitable(ignore)]
    pub format_trait: FormatTrait,
    /// `{}` or `{:.5}` or `{:-^20}`, etc.
    #[visitable(ignore)]
    pub format_options: FormatOptions,
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=STRUCT | NAME=FormatArgPosition | COMPLEXITY=4 | LINES=13

```rust
#[derive(Clone, Encodable, Decodable, Debug, PartialEq, Eq, Walkable)]
pub struct FormatArgPosition {
    /// Which argument this position refers to (Ok),
    /// or would've referred to if it existed (Err).
    #[visitable(ignore)]
    pub index: Result<usize, usize>,
    /// What kind of position this is. See [`FormatArgPositionKind`].
    #[visitable(ignore)]
    pub kind: FormatArgPositionKind,
    /// The span of the name or number.
    pub span: Option<Span>,
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=10 | LINES=10

```rust
#[derive(Copy, Clone, Encodable, Decodable, Debug, PartialEq, Eq)]
pub enum FormatArgPositionKind {
    /// `{}` or `{:.*}`
    Implicit,
    /// `{1}` or `{:1$}` or `{:.1$}`
    Number,
    /// `{a}` or `{:a$}` or `{:.a$}`
    Named,
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=11 | LINES=22

```rust
#[derive(Copy, Clone, Encodable, Decodable, Debug, PartialEq, Eq, Hash)]
pub enum FormatTrait {
    /// `{}`
    Display,
    /// `{:?}`
    Debug,
    /// `{:e}`
    LowerExp,
    /// `{:E}`
    UpperExp,
    /// `{:o}`
    Octal,
    /// `{:p}`
    Pointer,
    /// `{:b}`
    Binary,
    /// `{:x}`
    LowerHex,
    /// `{:X}`
    UpperHex,
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=STRUCT | NAME=FormatOptions | COMPLEXITY=15 | LINES=20

```rust
#[derive(Clone, Encodable, Decodable, Default, Debug, PartialEq, Eq)]
pub struct FormatOptions {
    /// The width. E.g. `{:5}` or `{:width$}`.
    pub width: Option<FormatCount>,
    /// The precision. E.g. `{:.5}` or `{:.precision$}`.
    pub precision: Option<FormatCount>,
    /// The alignment. E.g. `{:>}` or `{:<}` or `{:^}`.
    pub alignment: Option<FormatAlignment>,
    /// The fill character. E.g. the `.` in `{:.>10}`.
    pub fill: Option<char>,
    /// The `+` or `-` flag.
    pub sign: Option<FormatSign>,
    /// The `#` flag.
    pub alternate: bool,
    /// The `0` flag. E.g. the `0` in `{:02x}`.
    pub zero_pad: bool,
    /// The `x` or `X` flag (for `Debug` only). E.g. the `x` in `{:x?}`.
    pub debug_hex: Option<FormatDebugHex>,
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=8

```rust
#[derive(Copy, Clone, Encodable, Decodable, Debug, PartialEq, Eq)]
pub enum FormatSign {
    /// The `+` flag.
    Plus,
    /// The `-` flag.
    Minus,
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=4 | LINES=8

```rust
#[derive(Copy, Clone, Encodable, Decodable, Debug, PartialEq, Eq)]
pub enum FormatDebugHex {
    /// The `x` flag in `{:x?}`.
    Lower,
    /// The `X` flag in `{:X?}`.
    Upper,
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=5 | LINES=10

```rust
#[derive(Copy, Clone, Encodable, Decodable, Debug, PartialEq, Eq)]
pub enum FormatAlignment {
    /// `{:<}`
    Left,
    /// `{:>}`
    Right,
    /// `{:^}`
    Center,
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=7 | LINES=8

```rust
#[derive(Clone, Encodable, Decodable, Debug, PartialEq, Eq)]
pub enum FormatCount {
    /// `{:5}` or `{:.5}`
    Literal(u16),
    /// `{:.*}`, `{:.5$}`, or `{:a$}`, etc.
    Argument(FormatArgPosition),
}
```

---
*Generated by AST tracing system*
