// In expand_utils_lib/src/ast_fragment_impl.rs

use rustc_ast::ast;
use rustc_ast::NodeId; // Needed for add_placeholders, which is now stubbed out
use rustc_ast::mut_visit::MutVisitor; // For mut_visit_with
use rustc_ast::visit::{Visitor, AssocCtxt}; // For visit_with
use rustc_ast_pretty::pprust; // For to_string (part of AstFragmentKind's generated code implicitly)
use smallvec::SmallVec;

use crate::{AstFragment, AstFragmentKind, shim_walk_stmt, add_placeholders_to_fragment}; // Import from our lib.rs
use crate::InvocationCollectorNode; // Will be moved here soon

// The 'impl AstFragment' block gives the AstFragment enum abilities, like converting itself into specific AST nodes.
impl AstFragment {
    // This method tries to extract an optional expression (OptExpr) from the AstFragment.
    // OptExpr can be either Some(expression) or None.
    pub fn make_opt_expr(self) -> Option<Box<ast::Expr>> {
        panic!("AstFragment::make_opt_expr is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a method receiver expression (MethodReceiverExpr) from the AstFragment.
    // A method receiver is the 'self' part of a method call, like `my_object.method()`.
    pub fn make_method_receiver_expr(self) -> Option<Box<ast::Expr>> {
        panic!("AstFragment::make_method_receiver_expr is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a general expression (Expr) from the AstFragment.
    pub fn make_expr(self) -> Option<Box<ast::Expr>> {
        panic!("AstFragment::make_expr is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a pattern (Pat) from the AstFragment.
    // Patterns are used in 'match' statements or 'let' bindings (e.g., `let Some(x) = ...`).
    pub fn make_pat(self) -> Option<Box<ast::Pat>> {
        panic!("AstFragment::make_pat is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a type (Ty) from the AstFragment.
    pub fn make_ty(self) -> Option<Box<ast::Ty>> {
        panic!("AstFragment::make_ty is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of statements (Stmts) from the AstFragment.
    pub fn make_stmts(self) -> Option<SmallVec<ast::Stmt, 1>> {
        panic!("AstFragment::make_stmts is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of items (Items) from the AstFragment.
    // Items are top-level declarations like functions, structs, enums, etc.
    pub fn make_items(self) -> Option<SmallVec<Box<ast::Item>, 1>> {
        panic!("AstFragment::make_items is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of trait items (TraitItems) from the AstFragment.
    // Trait items are methods or associated types/constants declared within a trait.
    pub fn make_trait_items(self) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        panic!("AstFragment::make_trait_items is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of impl items (ImplItems) from the AstFragment.
    // Impl items are methods or associated types/constants implemented for a specific type.
    pub fn make_impl_items(self) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        panic!("AstFragment::make_impl_items is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of trait impl items (TraitImplItems) from the AstFragment.
    // These are items within an `impl Trait for Type` block.
    pub fn make_trait_impl_items(self) -> Option<SmallVec<Box<ast::AssocItem>, 1>> {
        panic!("AstFragment::make_trait_impl_items is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of foreign items (ForeignItems) from the AstFragment.
    // Foreign items are declarations in `extern "C"` blocks.
    pub fn make_foreign_items(self) -> Option<SmallVec<Box<ast::ForeignItem>, 1>> {
        panic!("AstFragment::make_foreign_items is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of match arms (Arms) from the AstFragment.
    // Match arms are the `pattern => expression` parts of a `match` statement.
    pub fn make_arms(self) -> Option<SmallVec<ast::Arm, 1>> {
        panic!("AstFragment::make_arms is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of expression fields (ExprFields) from the AstFragment.
    // These are fields in struct literal expressions (e.g., `MyStruct { field1: value1, ... }`).
    pub fn make_expr_fields(self) -> Option<SmallVec<ast::ExprField, 1>> {
        panic!("AstFragment::make_expr_fields is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of pattern fields (PatFields) from the AstFragment.
    // These are fields in struct patterns (e.g., `MyStruct { field1, field2: value2 } => ...`).
    pub fn make_pat_fields(self) -> Option<SmallVec<ast::PatField, 1>> {
        panic!("AstFragment::make_pat_fields is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of generic parameters (GenericParams) from the AstFragment.
    // These are type parameters, lifetime parameters, or const generics (e.g., `<T, 'a, const N: usize>`).
    pub fn make_generic_params(self) -> Option<SmallVec<ast::GenericParam, 1>> {
        panic!("AstFragment::make_generic_params is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of function parameters (Params) from the AstFragment.
    pub fn make_params(self) -> Option<SmallVec<ast::Param, 1>> {
        panic!("AstFragment::make_params is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of field definitions (FieldDefs) from the AstFragment.
    // These are fields in struct or enum variant definitions.
    pub fn make_field_defs(self) -> Option<SmallVec<ast::FieldDef, 1>> {
        panic!("AstFragment::make_field_defs is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of enum variants (Variants) from the AstFragment.
    pub fn make_variants(self) -> Option<SmallVec<ast::Variant, 1>> {
        panic!("AstFragment::make_variants is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a sequence of where predicates (WherePredicates) from the AstFragment.
    // These are the clauses in a `where` statement that specify trait bounds (e.g., `where T: Debug`).
    pub fn make_where_predicates(self) -> Option<SmallVec<ast::WherePredicate, 1>> {
        panic!("AstFragment::make_where_predicates is not yet implemented in expand_utils_lib")
    }

    // This method tries to extract a crate (Crate) from the AstFragment.
    // A Crate is the top-level compilation unit in Rust.
    pub fn make_crate(self) -> Option<ast::Crate> {
        panic!("AstFragment::make_crate is not yet implemented in expand_utils_lib")
    }

    pub fn make_ast<T: InvocationCollectorNode>(self) -> T::OutputTy {
        panic!("AstFragment::make_ast is not yet implemented in expand_utils_lib")
    }

    pub fn mut_visit_with<__V: MutVisitor + ?Sized>(&mut self, __visitor: &mut __V) {
        panic!("AstFragment::mut_visit_with is not yet implemented in expand_utils_lib")
    }

    pub fn visit_with<'a, V: Visitor<'a>>(&'a self, visitor: &mut V) -> V::Result {
        panic!("AstFragment::visit_with is not yet implemented in expand_utils_lib")
    }

    pub fn to_string(&self) -> String {
        panic!("AstFragment::to_string is not yet implemented in expand_utils_lib")
    }
}
