# AST Trace: ../rust/library/stdarch/crates/stdarch-gen-arm/src/matching.rs

Generated 17 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use proc_macro2::TokenStream;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use std::fmt;

use crate::context::{self, LocalContext};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use crate::typekinds::{BaseType, BaseTypeKind, TypeKind};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=MatchSizeValues | COMPLEXITY=2 | LINES=9

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchSizeValues<T> {
    pub default: T,
    pub byte: Option<T>,
    pub halfword: Option<T>,
    pub doubleword: Option<T>,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=get | COMPLEXITY=20 | LINES=19

```rust
impl<T> MatchSizeValues<T> {
    pub fn get(&mut self, ty: &TypeKind, ctx: &LocalContext) -> context::Result<&T> {
        let base_ty = if let Some(w) = ty.wildcard() {
            ctx.provide_type_wildcard(w)?
        } else {
            ty.clone()
        };

        if let BaseType::Sized(_, bitsize) = base_ty.base_type().unwrap() {
            match (bitsize, &self.byte, &self.halfword, &self.doubleword) {
                (64, _, _, Some(v)) | (16, _, Some(v), _) | (8, Some(v), _, _) => Ok(v),
                _ => Ok(&self.default),
            }
        } else {
            Err(format!("cannot match bitsize to unsized type {ty:?}!"))
        }
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=MatchKindValues | COMPLEXITY=2 | LINES=8

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchKindValues<T> {
    pub default: T,
    pub float: Option<T>,
    pub unsigned: Option<T>,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=get | COMPLEXITY=12 | LINES=19

```rust
impl<T> MatchKindValues<T> {
    pub fn get(&mut self, ty: &TypeKind, ctx: &LocalContext) -> context::Result<&T> {
        let base_ty = if let Some(w) = ty.wildcard() {
            ctx.provide_type_wildcard(w)?
        } else {
            ty.clone()
        };

        match (
            base_ty.base_type().unwrap().kind(),
            &self.float,
            &self.unsigned,
        ) {
            (BaseTypeKind::Float, Some(v), _) | (BaseTypeKind::UInt, _, Some(v)) => Ok(v),
            _ => Ok(&self.default),
        }
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum SizeMatchable<T> {
    Matched(T),
    Unmatched {
        match_size: Option<TypeKind>,
        #[serde(flatten)]
        values: MatchSizeValues<Box<T>>,
    },
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=perform_match | COMPLEXITY=12 | LINES=17

```rust
impl<T: Clone> SizeMatchable<T> {
    pub fn perform_match(&mut self, ctx: &LocalContext) -> context::Result {
        match self {
            Self::Unmatched {
                match_size: None,
                values: MatchSizeValues { default, .. },
            } => *self = Self::Matched(*default.to_owned()),
            Self::Unmatched {
                match_size: Some(ty),
                values,
            } => *self = Self::Matched(*values.get(ty, ctx)?.to_owned()),
            _ => {}
        }
        Ok(())
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=as_ref | COMPLEXITY=15 | LINES=10

```rust
impl<T: fmt::Debug> AsRef<T> for SizeMatchable<T> {
    fn as_ref(&self) -> &T {
        if let SizeMatchable::Matched(v) = self {
            v
        } else {
            panic!("no match for {self:?} was performed");
        }
    }
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=as_mut | COMPLEXITY=15 | LINES=10

```rust
impl<T: fmt::Debug> AsMut<T> for SizeMatchable<T> {
    fn as_mut(&mut self) -> &mut T {
        if let SizeMatchable::Matched(v) = self {
            v
        } else {
            panic!("no match for {self:?} was performed");
        }
    }
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=to_tokens | COMPLEXITY=5 | LINES=6

```rust
impl<T: fmt::Debug + ToTokens> ToTokens for SizeMatchable<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.as_ref().to_tokens(tokens)
    }
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=11

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields)]
pub enum KindMatchable<T> {
    Matched(T),
    Unmatched {
        match_kind: Option<TypeKind>,
        #[serde(flatten)]
        values: MatchKindValues<Box<T>>,
    },
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=perform_match | COMPLEXITY=12 | LINES=17

```rust
impl<T: Clone> KindMatchable<T> {
    pub fn perform_match(&mut self, ctx: &LocalContext) -> context::Result {
        match self {
            Self::Unmatched {
                match_kind: None,
                values: MatchKindValues { default, .. },
            } => *self = Self::Matched(*default.to_owned()),
            Self::Unmatched {
                match_kind: Some(ty),
                values,
            } => *self = Self::Matched(*values.get(ty, ctx)?.to_owned()),
            _ => {}
        }
        Ok(())
    }
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=as_ref | COMPLEXITY=15 | LINES=10

```rust
impl<T: fmt::Debug> AsRef<T> for KindMatchable<T> {
    fn as_ref(&self) -> &T {
        if let KindMatchable::Matched(v) = self {
            v
        } else {
            panic!("no match for {self:?} was performed");
        }
    }
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=as_mut | COMPLEXITY=15 | LINES=10

```rust
impl<T: fmt::Debug> AsMut<T> for KindMatchable<T> {
    fn as_mut(&mut self) -> &mut T {
        if let KindMatchable::Matched(v) = self {
            v
        } else {
            panic!("no match for {self:?} was performed");
        }
    }
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=to_tokens | COMPLEXITY=5 | LINES=6

```rust
impl<T: fmt::Debug + ToTokens> ToTokens for KindMatchable<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.as_ref().to_tokens(tokens)
    }
}
```

---
*Generated by AST tracing system*
