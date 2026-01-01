# AST Trace: ../rust/compiler/rustc_builtin_macros/src/deriving/mod.rs

Generated 11 AST blocks from source file

## Block 1
**Metadata**: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
//! The compiler code necessary to implement the `#[derive]` extensions.

use rustc_ast as ast;
use rustc_ast::{GenericArg, MetaItem};
```

## Block 2
**Metadata**: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_expand::base::{Annotatable, ExpandResult, ExtCtxt, MultiItemModifier};
```

## Block 3
**Metadata**: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use rustc_span::{Span, Symbol, sym};
```

## Block 4
**Metadata**: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1

```rust
use thin_vec::{ThinVec, thin_vec};
```

## Block 5
**Metadata**: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
macro path_local($x:ident) {
    generic::ty::Path::new_local(sym::$x)
}
```

## Block 6
**Metadata**: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=4

```rust
macro pathvec_std($($rest:ident)::+) {{
    vec![ $( sym::$rest ),+ ]
}}
```

## Block 7
**Metadata**: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4

```rust
macro path_std($($x:tt)*) {
    generic::ty::Path::new( pathvec_std!( $($x)* ) )
}
```

## Block 8
**Metadata**: AST_ID=8 | TYPE=FUNCTION | NAME=expand | COMPLEXITY=21 | LINES=68

```rust
pub(crate) mod bounds;
pub(crate) mod clone;
pub(crate) mod coerce_pointee;
pub(crate) mod debug;
pub(crate) mod default;
pub(crate) mod from;
pub(crate) mod hash;

#[path = "cmp/eq.rs"]
pub(crate) mod eq;
#[path = "cmp/ord.rs"]
pub(crate) mod ord;
#[path = "cmp/partial_eq.rs"]
pub(crate) mod partial_eq;
#[path = "cmp/partial_ord.rs"]
pub(crate) mod partial_ord;

pub(crate) mod generic;

pub(crate) type BuiltinDeriveFn =
    fn(&ExtCtxt<'_>, Span, &MetaItem, &Annotatable, &mut dyn FnMut(Annotatable), bool);

pub(crate) struct BuiltinDerive(pub(crate) BuiltinDeriveFn);

impl MultiItemModifier for BuiltinDerive {
    fn expand(
        &self,
        ecx: &mut ExtCtxt<'_>,
        span: Span,
        meta_item: &MetaItem,
        item: Annotatable,
        is_derive_const: bool,
    ) -> ExpandResult<Vec<Annotatable>, Annotatable> {
        // FIXME: Built-in derives often forget to give spans contexts,
        // so we are doing it here in a centralized way.
        let span = ecx.with_def_site_ctxt(span);
        let mut items = Vec::new();
        match item {
            Annotatable::Stmt(stmt) => {
                if let ast::StmtKind::Item(item) = stmt.kind {
                    (self.0)(
                        ecx,
                        span,
                        meta_item,
                        &Annotatable::Item(item),
                        &mut |a| {
                            // Cannot use 'ecx.stmt_item' here, because we need to pass 'ecx'
                            // to the function
                            items.push(Annotatable::Stmt(Box::new(ast::Stmt {
                                id: ast::DUMMY_NODE_ID,
                                kind: ast::StmtKind::Item(a.expect_item()),
                                span,
                            })));
                        },
                        is_derive_const,
                    );
                } else {
                    unreachable!("should have already errored on non-item statement")
                }
            }
            _ => {
                (self.0)(ecx, span, meta_item, &item, &mut |a| items.push(a), is_derive_const);
            }
        }
        ExpandResult::Ready(items)
    }
}
```

## Block 9
**Metadata**: AST_ID=9 | TYPE=FUNCTION | NAME=call_intrinsic | COMPLEXITY=2 | LINES=12

```rust
/// Constructs an expression that calls an intrinsic
fn call_intrinsic(
    cx: &ExtCtxt<'_>,
    span: Span,
    intrinsic: Symbol,
    args: ThinVec<Box<ast::Expr>>,
) -> Box<ast::Expr> {
    let span = cx.with_def_site_ctxt(span);
    let path = cx.std_path(&[sym::intrinsics, intrinsic]);
    cx.expr_call_global(span, path, args)
}
```

## Block 10
**Metadata**: AST_ID=10 | TYPE=FUNCTION | NAME=call_unreachable | COMPLEXITY=4 | LINES=15

```rust
/// Constructs an expression that calls the `unreachable` intrinsic.
fn call_unreachable(cx: &ExtCtxt<'_>, span: Span) -> Box<ast::Expr> {
    let span = cx.with_def_site_ctxt(span);
    let path = cx.std_path(&[sym::intrinsics, sym::unreachable]);
    let call = cx.expr_call_global(span, path, ThinVec::new());

    cx.expr_block(Box::new(ast::Block {
        stmts: thin_vec![cx.stmt_expr(call)],
        id: ast::DUMMY_NODE_ID,
        rules: ast::BlockCheckMode::Unsafe(ast::CompilerGenerated),
        span,
        tokens: None,
    }))
}
```

## Block 11
**Metadata**: AST_ID=11 | TYPE=FUNCTION | NAME=assert_ty_bounds | COMPLEXITY=2 | LINES=13

```rust
fn assert_ty_bounds(
    cx: &ExtCtxt<'_>,
    stmts: &mut ThinVec<ast::Stmt>,
    ty: Box<ast::Ty>,
    span: Span,
    assert_path: &[Symbol],
) {
    // Generate statement `let _: assert_path<ty>;`.
    let span = cx.with_def_site_ctxt(span);
    let assert_path = cx.path_all(span, true, cx.std_path(assert_path), vec![GenericArg::Type(ty)]);
    stmts.push(cx.stmt_let_type_only(span, cx.ty_path(assert_path)));
}
```

---
*Generated by AST tracing system*
