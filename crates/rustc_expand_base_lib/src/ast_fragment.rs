use crate::{MacResult, DummyResult};
use rustc_ast::{self as ast, NodeId, PatKind, StmtKind, ExprKind, Expr, Ty};
use rustc_errors::ErrorGuaranteed;
use rustc_span::Span;
use smallvec::SmallVec;
use rustc_ast::mut_visit::MutVisitor;
use rustc_ast::visit::{self, Visitor, try_visit, VisitorResult};
use rustc_ast_pretty::pprust;
use crate::prelude::Annotatable; // Added
use crate::resolver_traits::OpaqueDeriveResolution;

// ADDED: AstFragment and AstFragmentKind definitions from expand_utils_lib
/// A fragment of AST that can be produced by a single macro expansion.
/// Can also serve as an input and intermediate result for macro expansion operations.
#[derive(Debug)]
pub enum AstFragment {
    OptExpr(Option<Box<ast::Expr>>),
    MethodReceiverExpr(Box<ast::Expr>),
    Expr(Box<ast::Expr>),
    Pat(Box<ast::Pat>),
    Ty(Box<ast::Ty>),
    Stmts(SmallVec<ast::Stmt, 1>),
    Items(SmallVec<Box<ast::Item>, 1>),
    TraitItems(SmallVec<Box<ast::AssocItem>, 1>),
    ImplItems(SmallVec<Box<ast::AssocItem>, 1>),
    TraitImplItems(SmallVec<Box<ast::AssocItem>, 1>), // Re-added
    ForeignItems(SmallVec<Box<ast::ForeignItem>, 1>),
    Arms(SmallVec<ast::Arm, 1>),
    ExprFields(SmallVec<ast::ExprField, 1>),
    PatFields(SmallVec<ast::PatField, 1>),
    GenericParams(SmallVec<ast::GenericParam, 1>),
    Params(SmallVec<ast::Param, 1>),
    FieldDefs(SmallVec<ast::FieldDef, 1>),
    Variants(SmallVec<ast::Variant, 1>),
    WherePredicates(SmallVec<ast::WherePredicate, 1>),
    Crate(ast::Crate),
}

/// "Discriminant" of an AST fragment.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AstFragmentKind {
    OptExpr,
    MethodReceiverExpr,
    Expr, Pat, Ty, Stmts, Items, TraitItems, ImplItems, TraitImplItems, ForeignItems, Arms,
    ExprFields, PatFields, GenericParams, Params, FieldDefs, Variants, WherePredicates, Crate,
}

impl std::fmt::Display for AstFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // Use Debug implementation for Display
    }
}

impl AstFragmentKind {
    pub fn name(self) -> &'static str {
        match self {
            AstFragmentKind::OptExpr => "optional expression",
            AstFragmentKind::MethodReceiverExpr => "method receiver expression",
            AstFragmentKind::Expr => "expression",
            AstFragmentKind::Pat => "pattern",
            AstFragmentKind::Ty => "type",
            AstFragmentKind::Stmts => "statement",
            AstFragmentKind::Items => "item",
            AstFragmentKind::TraitItems => "trait item",
            AstFragmentKind::ImplItems => "impl item",
            AstFragmentKind::TraitImplItems => "impl item",
            AstFragmentKind::ForeignItems => "foreign item",
            AstFragmentKind::Arms => "match arm",
            AstFragmentKind::ExprFields => "field expression",
            AstFragmentKind::PatFields => "field pattern",
            AstFragmentKind::GenericParams => "generic parameter",
            AstFragmentKind::Params => "function parameter",
            AstFragmentKind::FieldDefs => "field",
            AstFragmentKind::Variants => "variant",
            AstFragmentKind::WherePredicates => "where predicate",
            AstFragmentKind::Crate => "crate",
        }
    }

    pub fn make_from<DRT: OpaqueDeriveResolution + 'static>(self, result: Box<dyn MacResult<DRT> + '_>) -> Option<AstFragment> {
        match self {
	        AstFragmentKind::Expr => result.make_expr().map(AstFragment::Expr),
            AstFragmentKind::Pat => result.make_pat().map(AstFragment::Pat),
            AstFragmentKind::Ty => result.make_ty().map(AstFragment::Ty),
            AstFragmentKind::Stmts => result.make_stmts().map(AstFragment::Stmts),
            AstFragmentKind::Items => result.make_items().map(AstFragment::Items),
            AstFragmentKind::TraitItems => result.make_trait_items().map(AstFragment::TraitItems),
            AstFragmentKind::ImplItems => result.make_impl_items().map(AstFragment::ImplItems),
            AstFragmentKind::TraitImplItems => result.make_trait_impl_items().map(AstFragment::TraitImplItems), // Re-added
            // Moved to end: AstFragmentKind::TraitImplItems => result.make_trait_impl_items().map(AstFragment::TraitImplItems),
            AstFragmentKind::ForeignItems => result.make_foreign_items().map(AstFragment::ForeignItems),
            AstFragmentKind::Arms => result.make_arms().map(AstFragment::Arms),
            AstFragmentKind::ExprFields => result.make_expr_fields().map(AstFragment::ExprFields),
            AstFragmentKind::PatFields => result.make_pat_fields().map(AstFragment::PatFields),
            AstFragmentKind::GenericParams => result.make_generic_params().map(AstFragment::GenericParams),
            AstFragmentKind::Params => result.make_params().map(AstFragment::Params),
            AstFragmentKind::FieldDefs => result.make_field_defs().map(AstFragment::FieldDefs), // Corrected
            AstFragmentKind::Variants => result.make_variants().map(AstFragment::Variants),
            AstFragmentKind::WherePredicates => result.make_where_predicates().map(AstFragment::WherePredicates),
            AstFragmentKind::Crate => result.make_crate().map(AstFragment::Crate),
            // Explains: For OptExpr, we use the general `make_expr()` from `MacResult`
            // and then wrap it as an `OptExpr` variant of `AstFragment`.
            AstFragmentKind::OptExpr => result.make_expr().map(|e| AstFragment::OptExpr(Some(e))),
            // Explains: For MethodReceiverExpr, we also use the general `make_expr()`
            // and wrap it accordingly.
            AstFragmentKind::MethodReceiverExpr => result.make_expr().map(AstFragment::MethodReceiverExpr),
            // Moved TraitImplItems to the end to see if it changes the error.

        }
    }

    pub fn expect_from_annotatables(self, items: impl Iterator<Item = Annotatable>) -> AstFragment {
        unimplemented!()
    }

    // Explains: This method creates a "dummy" or placeholder `AstFragment` for this kind.
    // This is used as a fallback when a macro expansion fails, to allow compilation to continue
    // and report more errors later.
    #[allow(unused_variables)]
    pub fn dummy<DRT: OpaqueDeriveResolution + 'static>(self, span: Span, guar: ErrorGuaranteed) -> AstFragment {
        // We use `self.make_from` (the current `AstFragmentKind`) with a `DummyResult`
        // which provides "empty" or "default" results for each `make_*` method.
        // `.unwrap()` is used because `DummyResult` is guaranteed to always produce a `Some` value.
        self.make_from(DummyResult::any::<DRT>(span, guar)).unwrap()
    }
}


impl AstFragment {
    fn add_placeholders(&mut self, placeholders: &[NodeId]) {
        if placeholders.is_empty() {
            return;
        }
        match self {
            // For each fragment definition, dispatch to the arm generator.
            // This will generate the appropriate match arm for `add_placeholders`
            // if the fragment is of 'many' type.
            _ => panic!("add_placeholders: Fragment not yet implemented or dispatched correctly"),
            // Handles the hardcoded OptExpr and MethodReceiverExpr
            AstFragment::OptExpr(_) | AstFragment::MethodReceiverExpr(_) => {}
        }
    }

    pub fn make_opt_expr(self) -> Option<Box<ast::Expr>> {
        match self {
            AstFragment::OptExpr(expr) => expr,
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }

    pub fn make_method_receiver_expr(self) -> Option<Box<ast::Expr>> {
        match self {
            AstFragment::MethodReceiverExpr(expr) => Some(expr),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }

    pub fn make_expr(self) -> Option<Box<ast::Expr>> {
        match self {
            AstFragment::Expr(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_pat(self) -> Option<Box<ast::Pat>> {
        match self {
            AstFragment::Pat(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_ty(self) -> Option<Box<ast::Ty>> {
        match self {
            AstFragment::Ty(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_stmts(self) -> Option<SmallVec<ast::Stmt, 1>> {
        match self {
            AstFragment::Stmts(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_items(self) -> Option<SmallVec<Box<ast::Item>, 1>> {
        match self {
            AstFragment::Items(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_trait_items(self) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        match self {
            AstFragment::TraitItems(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_impl_items(self) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        match self {
            AstFragment::ImplItems(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_foreign_items(self) -> Option<SmallVec<Box<ast::ForeignItem>, 1>> {
        match self {
            AstFragment::ForeignItems(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_arms(self) -> Option<SmallVec<ast::Arm, 1>> {
        match self {
            AstFragment::Arms(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_expr_fields(self) -> Option<SmallVec<ast::ExprField, 1>> {
        match self {
            AstFragment::ExprFields(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_pat_fields(self) -> Option<SmallVec<ast::PatField, 1>> {
        match self {
            AstFragment::PatFields(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_generic_params(self) -> Option<SmallVec<ast::GenericParam, 1>> {
        match self {
            AstFragment::GenericParams(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_params(self) -> Option<SmallVec<ast::Param, 1>> {
        match self {
            AstFragment::Params(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_field_defs(self) -> Option<SmallVec<ast::FieldDef, 1>> {
        match self {
            AstFragment::FieldDefs(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_variants(self) -> Option<SmallVec<ast::Variant, 1>> {
        match self {
            AstFragment::Variants(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_where_predicates(self) -> Option<SmallVec<ast::WherePredicate, 1>> {
        match self {
            AstFragment::WherePredicates(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }
    pub fn make_crate(self) -> Option<ast::Crate> {
        match self {
            AstFragment::Crate(ast) => Some(ast),
            _ => panic!("AstFragment::make_* called on the wrong kind of fragment"),
        }
    }

    // Now that InvocationCollectorNode is in this module, this can be uncommented
    /*pub fn make_ast<T: InvocationCollectorNode>(self) -> T::OutputTy {
        T::fragment_to_output(self)
    }*/


    pub fn mut_visit_with(&mut self, vis: &mut impl MutVisitor) {
        match self {
            AstFragment::OptExpr(opt_expr) => {
                if let Some(expr) = opt_expr.take() {
                    *opt_expr = vis.filter_map_expr(expr)
                }
            }
            AstFragment::MethodReceiverExpr(expr) => vis.visit_method_receiver_expr(expr),
            _ => panic!("mut_visit_with: Fragment not yet implemented or dispatched correctly"),
        }
    }

                pub fn visit_with<'a, V: Visitor<'a, Result = ()>>(&'a self, visitor: &mut V) -> V::Result {
                    match self {
                        AstFragment::OptExpr(Some(expr)) => try_visit!(visitor.visit_expr(expr)),
                        AstFragment::OptExpr(None) => V::Result::output(),
                        AstFragment::MethodReceiverExpr(expr) => try_visit!(visitor.visit_method_receiver_expr(expr)),
                        _ => panic!("visit_with: Fragment not yet implemented or dispatched correctly"),
                    }
                    V::Result::output()
                }
    pub fn to_string(&self) -> String {
        match self {
            AstFragment::OptExpr(Some(expr)) => pprust::expr_to_string(expr),
            AstFragment::OptExpr(None) => unreachable!(),
            AstFragment::MethodReceiverExpr(expr) => pprust::expr_to_string(expr),
            _ => panic!("to_string: Fragment not yet implemented or dispatched correctly"),
        }
    }
}