# AST Trace: ../rust/library/compiler-builtins/crates/libm-macros/src/parse.rs

Generated 20 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=5

```rust
use std::collections::BTreeMap;

use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Parser};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3

```rust
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::{self, Comma};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use syn::{Arm, Attribute, Expr, ExprMatch, Ident, LitBool, Meta, Token, bracketed};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=STRUCT | NAME=Invocation | COMPLEXITY=2 | LINES=6

```rust
/// The input to our macro; just a list of `field: value` items.
#[derive(Debug)]
pub struct Invocation {
    fields: Punctuated<Mapping, Comma>,
}
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=FUNCTION | NAME=parse | COMPLEXITY=6 | LINES=8

```rust
impl Parse for Invocation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fields: input.parse_terminated(Mapping::parse, Token![,])?,
        })
    }
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=STRUCT | NAME=Mapping | COMPLEXITY=2 | LINES=8

```rust
/// A `key: expression` mapping with nothing else. Basically a simplified `syn::Field`.
#[derive(Debug)]
struct Mapping {
    name: Ident,
    _sep: Token![:],
    expr: Expr,
}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=FUNCTION | NAME=parse | COMPLEXITY=6 | LINES=10

```rust
impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _sep: input.parse()?,
            expr: input.parse()?,
        })
    }
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=STRUCT | NAME=StructuredInput | COMPLEXITY=8 | LINES=26

```rust
/// The input provided to our proc macro, after parsing into the form we expect.
#[derive(Debug)]
pub struct StructuredInput {
    /// Macro to invoke once per function
    pub callback: Ident,
    /// Whether or not to provide `CFn` `CArgs` `RustFn` etc. This is really only needed
    /// once for crate to set up the main trait.
    pub emit_types: Vec<Ident>,
    /// Skip these functions
    pub skip: Vec<Ident>,
    /// If true, omit f16 and f128 functions that aren't present in other libraries.
    pub skip_f16_f128: bool,
    /// Invoke only for these functions
    pub only: Option<Vec<Ident>>,
    /// Attributes that get applied to specific functions
    pub attributes: Option<Vec<AttributeMap>>,
    /// Extra expressions to pass to all invocations of the macro
    pub extra: Option<Expr>,
    /// Per-function extra expressions to pass to the macro
    pub fn_extra: Option<BTreeMap<Ident, Expr>>,
    // For diagnostics
    pub emit_types_span: Option<Span>,
    pub only_span: Option<Span>,
    pub fn_extra_span: Option<Span>,
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=from_fields | COMPLEXITY=41 | LINES=76

```rust
impl StructuredInput {
    pub fn from_fields(input: Invocation) -> syn::Result<Self> {
        let mut map: Vec<_> = input.fields.into_iter().collect();
        let cb_expr = expect_field(&mut map, "callback")?;
        let emit_types_expr = expect_field(&mut map, "emit_types").ok();
        let skip_expr = expect_field(&mut map, "skip").ok();
        let skip_f16_f128 = expect_field(&mut map, "skip_f16_f128").ok();
        let only_expr = expect_field(&mut map, "only").ok();
        let attr_expr = expect_field(&mut map, "attributes").ok();
        let extra = expect_field(&mut map, "extra").ok();
        let fn_extra = expect_field(&mut map, "fn_extra").ok();

        if !map.is_empty() {
            Err(syn::Error::new(
                map.first().unwrap().name.span(),
                format!("unexpected fields {map:?}"),
            ))?;
        }

        let emit_types_span = emit_types_expr.as_ref().map(|expr| expr.span());
        let emit_types = match emit_types_expr {
            Some(expr) => Parser::parse2(parse_ident_or_array, expr.into_token_stream())?,
            None => Vec::new(),
        };

        let skip = match skip_expr {
            Some(expr) => Parser::parse2(parse_ident_array, expr.into_token_stream())?,
            None => Vec::new(),
        };

        let skip_f16_f128 = match skip_f16_f128 {
            Some(expr) => expect_litbool(expr)?.value,
            None => false,
        };

        let only_span = only_expr.as_ref().map(|expr| expr.span());
        let only = match only_expr {
            Some(expr) => Some(Parser::parse2(parse_ident_array, expr.into_token_stream())?),
            None => None,
        };

        let attributes = match attr_expr {
            Some(expr) => {
                let mut attributes = Vec::new();
                let attr_exprs = Parser::parse2(parse_expr_array, expr.into_token_stream())?;

                for attr in attr_exprs {
                    attributes.push(syn::parse2(attr.into_token_stream())?);
                }
                Some(attributes)
            }
            None => None,
        };

        let fn_extra_span = fn_extra.as_ref().map(|expr| expr.span());
        let fn_extra = match fn_extra {
            Some(expr) => Some(extract_fn_extra_field(expr)?),
            None => None,
        };

        Ok(Self {
            callback: expect_ident(cb_expr)?,
            emit_types,
            skip,
            skip_f16_f128,
            only,
            only_span,
            attributes,
            extra,
            fn_extra,
            fn_extra_span,
            emit_types_span,
        })
    }
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=extract_fn_extra_field | COMPLEXITY=34 | LINES=58

```rust
fn extract_fn_extra_field(expr: Expr) -> syn::Result<BTreeMap<Ident, Expr>> {
    let Expr::Match(mexpr) = expr else {
        let e = syn::Error::new(expr.span(), "`fn_extra` expects a match expression");
        return Err(e);
    };

    let ExprMatch {
        attrs,
        match_token: _,
        expr,
        brace_token: _,
        arms,
    } = mexpr;

    expect_empty_attrs(&attrs)?;

    let match_on = expect_ident(*expr)?;
    if match_on != "MACRO_FN_NAME" {
        let e = syn::Error::new(match_on.span(), "only allowed to match on `MACRO_FN_NAME`");
        return Err(e);
    }

    let mut res = BTreeMap::new();

    for arm in arms {
        let Arm {
            attrs,
            pat,
            guard,
            fat_arrow_token: _,
            body,
            comma: _,
        } = arm;

        expect_empty_attrs(&attrs)?;

        let keys = match pat {
            syn::Pat::Wild(w) => vec![Ident::new("_", w.span())],
            _ => Parser::parse2(parse_ident_pat, pat.into_token_stream())?,
        };

        if let Some(guard) = guard {
            let e = syn::Error::new(guard.0.span(), "no guards allowed in this position");
            return Err(e);
        }

        for key in keys {
            let inserted = res.insert(key.clone(), *body.clone());
            if inserted.is_some() {
                let e = syn::Error::new(key.span(), format!("key `{key}` specified twice"));
                return Err(e);
            }
        }
    }

    Ok(res)
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=expect_empty_attrs | COMPLEXITY=5 | LINES=12

```rust
fn expect_empty_attrs(attrs: &[Attribute]) -> syn::Result<()> {
    if attrs.is_empty() {
        return Ok(());
    }

    let e = syn::Error::new(
        attrs.first().unwrap().span(),
        "no attributes allowed in this position",
    );
    Err(e)
}
```

## Block 12
**Metadata**: AST_ID=12 | TYPE=FUNCTION | NAME=expect_field | COMPLEXITY=6 | LINES=12

```rust
/// Extract a named field from a map, raising an error if it doesn't exist.
fn expect_field(v: &mut Vec<Mapping>, name: &str) -> syn::Result<Expr> {
    let pos = v.iter().position(|v| v.name == name).ok_or_else(|| {
        syn::Error::new(
            Span::call_site(),
            format!("missing expected field `{name}`"),
        )
    })?;

    Ok(v.remove(pos).expr)
}
```

## Block 13
**Metadata**: AST_ID=13 | TYPE=FUNCTION | NAME=expect_ident | COMPLEXITY=2 | LINES=5

```rust
/// Coerce an expression into a simple identifier.
fn expect_ident(expr: Expr) -> syn::Result<Ident> {
    syn::parse2(expr.into_token_stream())
}
```

## Block 14
**Metadata**: AST_ID=14 | TYPE=FUNCTION | NAME=expect_litbool | COMPLEXITY=2 | LINES=5

```rust
/// Coerce an expression into a simple keyword.
fn expect_litbool(expr: Expr) -> syn::Result<LitBool> {
    syn::parse2(expr.into_token_stream())
}
```

## Block 15
**Metadata**: AST_ID=15 | TYPE=FUNCTION | NAME=parse_ident_or_array | COMPLEXITY=5 | LINES=9

```rust
/// Parse either a single identifier (`foo`) or an array of identifiers (`[foo, bar, baz]`).
fn parse_ident_or_array(input: ParseStream) -> syn::Result<Vec<Ident>> {
    if !input.peek(token::Bracket) {
        return Ok(vec![input.parse()?]);
    }

    parse_ident_array(input)
}
```

## Block 16
**Metadata**: AST_ID=16 | TYPE=FUNCTION | NAME=parse_expr_array | COMPLEXITY=2 | LINES=8

```rust
/// Parse an array of expressions.
fn parse_expr_array(input: ParseStream) -> syn::Result<Vec<Expr>> {
    let content;
    let _ = bracketed!(content in input);
    let fields = content.parse_terminated(Expr::parse, Token![,])?;
    Ok(fields.into_iter().collect())
}
```

## Block 17
**Metadata**: AST_ID=17 | TYPE=FUNCTION | NAME=parse_ident_array | COMPLEXITY=2 | LINES=8

```rust
/// Parse an array of idents, e.g. `[foo, bar, baz]`.
fn parse_ident_array(input: ParseStream) -> syn::Result<Vec<Ident>> {
    let content;
    let _ = bracketed!(content in input);
    let fields = content.parse_terminated(Ident::parse, Token![,])?;
    Ok(fields.into_iter().collect())
}
```

## Block 18
**Metadata**: AST_ID=18 | TYPE=FUNCTION | NAME=parse_ident_pat | COMPLEXITY=5 | LINES=10

```rust
/// Parse an pattern of idents, specifically `(foo | bar | baz)`.
fn parse_ident_pat(input: ParseStream) -> syn::Result<Vec<Ident>> {
    if !input.peek2(Token![|]) {
        return Ok(vec![input.parse()?]);
    }

    let fields = Punctuated::<Ident, Token![|]>::parse_separated_nonempty(input)?;
    Ok(fields.into_iter().collect())
}
```

## Block 19
**Metadata**: AST_ID=19 | TYPE=STRUCT | NAME=AttributeMap | COMPLEXITY=2 | LINES=15

```rust
/// A mapping of attributes to identifiers (just a simplified `Expr`).
///
/// Expressed as:
///
/// ```ignore
/// #[meta1]
/// #[meta2]
/// [foo, bar, baz]
/// ```
#[derive(Debug)]
pub struct AttributeMap {
    pub meta: Vec<Meta>,
    pub names: Vec<Ident>,
}
```

## Block 20
**Metadata**: AST_ID=20 | TYPE=FUNCTION | NAME=parse | COMPLEXITY=6 | LINES=11

```rust
impl Parse for AttributeMap {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        Ok(Self {
            meta: attrs.into_iter().map(|a| a.meta).collect(),
            names: parse_ident_array(input)?,
        })
    }
}
```

---
*Generated by AST tracing system*
