use rustc_ast::{self as ast, NodeId, PatKind, StmtKind, ExprKind, Expr, Ty};
use rustc_errors::ErrorGuaranteed;
use rustc_span::{Span, DUMMY_SP};
use thin_vec::ThinVec;
use smallvec::{SmallVec, smallvec};
use crate::resolver_traits::OpaqueDeriveResolution;

// Use a macro because forwarding to a simple function has type system issues
macro_rules! make_stmts_default {
    ($me:expr, $TypeParam:ty) => {
        <dyn MacResult<$TypeParam>>::make_expr($me).map(|e| {
            smallvec![ast::Stmt {
                id: ast::DUMMY_NODE_ID,
                span: e.span,
                kind: StmtKind::Expr(e),
            }]
        })
    };
}

/// The result of a macro expansion. The return values of the various
/// methods are spliced into the AST at the callsite of the macro.
pub trait MacResult<DRT: OpaqueDeriveResolution + 'static> {
    /// Creates an expression.
    fn make_expr(self: Box<Self>) -> Option<Box<ast::Expr>> {
        None
    }

    /// Creates zero or more items.
    fn make_items(self: Box<Self>) -> Option<SmallVec<Box<ast::Item>, 1>> {
        None
    }

    /// Creates zero or more impl items.
    fn make_impl_items(self: Box<Self>) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        None
    }

    /// Creates zero or more impl items.
    fn make_trait_impl_items(self: Box<Self>) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        self.make_impl_items() // Delegate to make_impl_items
    }

    /// Creates zero or more trait items.
    fn make_trait_items(self: Box<Self>) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        None
    }

    /// Creates zero or more items in an `extern {}` block
    fn make_foreign_items(self: Box<Self>) -> Option<SmallVec<Box<ast::ForeignItem>, 1>> {
        None
    }

    /// Creates a pattern.
    fn make_pat(self: Box<Self>) -> Option<Box<ast::Pat>> {
        None
    }

    fn make_stmts(self: Box<Self>) -> Option<SmallVec<ast::Stmt, 1>> {
        None
    }

    fn make_ty(self: Box<Self>) -> Option<Box<ast::Ty>> {
        None
    }

    fn make_arms(self: Box<Self>) -> Option<SmallVec<ast::Arm, 1>> {
        None
    }

    fn make_expr_fields(self: Box<Self>) -> Option<SmallVec<ast::ExprField, 1>> {
        None
    }

    fn make_pat_fields(self: Box<Self>) -> Option<SmallVec<ast::PatField, 1>> {
        None
    }

    fn make_generic_params(self: Box<Self>) -> Option<SmallVec<ast::GenericParam, 1>> {
        None
    }

    fn make_params(self: Box<Self>) -> Option<SmallVec<ast::Param, 1>> {
        None
    }

    fn make_field_defs(self: Box<Self>) -> Option<SmallVec<ast::FieldDef, 1>> {
        None
    }

    fn make_variants(self: Box<Self>) -> Option<SmallVec<ast::Variant, 1>> {
        None
    }

    fn make_where_predicates(self: Box<Self>) -> Option<SmallVec<ast::WherePredicate, 1>> {
        None
    }

    fn make_crate(self: Box<Self>) -> Option<ast::Crate> {
        // Fn-like macros cannot produce a crate.
        unreachable!()
    }
}

macro_rules! make_MacEager {
    ( $( $fld:ident: $t:ty, )* ) => {
        /// `MacResult` implementation for the common case where you've already
        /// built each form of AST that you might return.
        #[derive(Default)]
        pub struct MacEager {
            $(
                pub $fld: Option<$t>,
            )*
        }

        impl MacEager {
            $(
                pub fn $fld<DRT: OpaqueDeriveResolution + 'static>(v: $t) -> Box<dyn MacResult<DRT>> {
                    Box::new(MacEager {
                        $fld: Some(v),
                        ..Default::default()
                    })
                }
            )*
        }
    }
}

make_MacEager! {
    expr: Box<ast::Expr>,
    pat: Box<ast::Pat>,
    items: SmallVec<Box<ast::Item>, 1>,
    impl_items: SmallVec<Box<ast::AssocItem>, 1>,
    trait_items: SmallVec<Box<ast::AssocItem>, 1>,
    foreign_items: SmallVec<Box<ast::ForeignItem>, 1>,
    stmts: SmallVec<ast::Stmt, 1>,
    ty: Box<ast::Ty>,
    where_predicates: SmallVec<ast::WherePredicate, 1>, // Re-added
}

impl<DRT: OpaqueDeriveResolution + 'static> MacResult<DRT> for MacEager {
    fn make_expr(self: Box<Self>) -> Option<Box<ast::Expr>> {
        self.expr
    }

    fn make_items(self: Box<Self>) -> Option<SmallVec<Box<ast::Item>, 1>> {
        self.items
    }

    fn make_impl_items(self: Box<Self>) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        self.impl_items
    }

    fn make_trait_impl_items(self: Box<Self>) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        self.impl_items
    }

    fn make_trait_items(self: Box<Self>) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        self.trait_items
    }

    fn make_foreign_items(self: Box<Self>) -> Option<SmallVec<Box<ast::ForeignItem>, 1>> {
        self.foreign_items
    }

    fn make_stmts(self: Box<Self>) -> Option<SmallVec<ast::Stmt, 1>> {
        match self.stmts.as_ref().map_or(0, |s| s.len()) {
            0 => make_stmts_default!(self,DRT),
            _ => self.stmts,
        }
    }

    fn make_pat(self: Box<Self>) -> Option<Box<ast::Pat>> {
        if let Some(p) = self.pat {
            return Some(p);
        }
        if let Some(e) = self.expr {
            if matches!(e.kind, ExprKind::Lit(_) | ExprKind::IncludedBytes(_)) {
                return Some(Box::new(ast::Pat {
                    id: ast::DUMMY_NODE_ID,
                    span: e.span,
                    kind: PatKind::Expr(e),
                    tokens: None,
                }));
            }
        }
        None
    }

    fn make_ty(self: Box<Self>) -> Option<Box<ast::Ty>> {
        self.ty
    }
}

/// Fill-in macro expansion result, to allow compilation to continue
/// after hitting errors.
#[derive(Copy, Clone)]
pub struct DummyResult {
    guar: Option<ErrorGuaranteed>,
    span: Span,
}

impl DummyResult {
    /// Creates a default MacResult that can be anything.
    ///
    /// Use this as a return value after hitting any errors and
    /// calling `span_err`.
    pub fn any<DRT: OpaqueDeriveResolution + 'static>(span: Span, guar: ErrorGuaranteed) -> Box<dyn MacResult<DRT> + 'static> {
        Box::new(DummyResult { guar: Some(guar), span })
    }

    /// Same as `any`, but must be a valid fragment, not error.
    pub fn any_valid<DRT: OpaqueDeriveResolution + 'static>(span: Span) -> Box<dyn MacResult<DRT> + 'static> {
        Box::new(DummyResult { guar: None, span })
    }

    /// A plain dummy expression.
    pub fn raw_expr(sp: Span, guar: Option<ErrorGuaranteed>) -> Box<ast::Expr> {
        Box::new(ast::Expr {
            id: ast::DUMMY_NODE_ID,
            kind: if let Some(guar) = guar {
                ExprKind::Err(guar)
            } else {
                ExprKind::Tup(ThinVec::new())
            },
            span: sp,
            attrs: ast::AttrVec::new(),
            tokens: None,
        })
    }
}

impl<DRT: OpaqueDeriveResolution + 'static> MacResult<DRT> for DummyResult {
    fn make_expr(self: Box<DummyResult>) -> Option<Box<ast::Expr>> {
        Some(DummyResult::raw_expr(self.span, self.guar))
    }

    fn make_pat(self: Box<DummyResult>) -> Option<Box<ast::Pat>> {
        Some(Box::new(ast::Pat {
            id: ast::DUMMY_NODE_ID,
            kind: PatKind::Wild,
            span: self.span,
            tokens: None,
        }))
    }

    fn make_items(self: Box<DummyResult>) -> Option<SmallVec<Box<ast::Item>, 1>> {
        Some(SmallVec::new())
    }

    fn make_impl_items(self: Box<DummyResult>) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        Some(SmallVec::new())
    }

    fn make_trait_impl_items(self: Box<DummyResult>) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        Some(SmallVec::new())
    }

    fn make_trait_items(self: Box<DummyResult>) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        Some(SmallVec::new())
    }

    fn make_foreign_items(self: Box<Self>) -> Option<SmallVec<Box<ast::ForeignItem>, 1>> {
        Some(SmallVec::new())
    }

    fn make_stmts(self: Box<DummyResult>) -> Option<SmallVec<ast::Stmt, 1>> {
        Some(smallvec![ast::Stmt {
            id: ast::DUMMY_NODE_ID,
            kind: StmtKind::Expr(DummyResult::raw_expr(self.span, self.guar)),
            span: self.span,
        }])
    }

    fn make_ty(self: Box<DummyResult>) -> Option<Box<ast::Ty>> {
        // FIXME(nnethercote): you might expect `ast::TyKind::Dummy` to be used here, but some
        // values produced here end up being lowered to HIR, which `ast::TyKind::Dummy` does not
        // support, so we use an empty tuple instead.
        Some(Box::new(ast::Ty {
            id: ast::DUMMY_NODE_ID,
            kind: ast::TyKind::Tup(ThinVec::new()),
            span: self.span,
            tokens: None,
        }))
    }

    fn make_arms(self: Box<DummyResult>) -> Option<SmallVec<ast::Arm, 1>> {
        Some(SmallVec::new())
    }

    fn make_expr_fields(self: Box<DummyResult>) -> Option<SmallVec<ast::ExprField, 1>> {
        Some(SmallVec::new())
    }

    fn make_pat_fields(self: Box<DummyResult>) -> Option<SmallVec<ast::PatField, 1>> {
        Some(SmallVec::new())
    }

    fn make_generic_params(self: Box<DummyResult>) -> Option<SmallVec<ast::GenericParam, 1>> {
        Some(SmallVec::new())
    }

    fn make_params(self: Box<DummyResult>) -> Option<SmallVec<ast::Param, 1>> {
        Some(SmallVec::new())
    }

    fn make_field_defs(self: Box<DummyResult>) -> Option<SmallVec<ast::FieldDef, 1>> {
        Some(SmallVec::new())
    }

    fn make_variants(self: Box<DummyResult>) -> Option<SmallVec<ast::Variant, 1>> {
        Some(SmallVec::new())
    }

    fn make_where_predicates(self: Box<DummyResult>) -> Option<SmallVec<ast::WherePredicate, 1>> {
        Some(SmallVec::new())
    }

    fn make_crate(self: Box<DummyResult>) -> Option<ast::Crate> {
        Some(ast::Crate {
            attrs: Default::default(),
            items: Default::default(),
            spans: Default::default(),
            id: ast::DUMMY_NODE_ID,
            is_placeholder: false,
	})
    }
}
