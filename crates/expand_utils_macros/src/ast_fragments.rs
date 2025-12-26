use rustc_expand_base_lib::prelude::*;
use rustc_ast::{self as ast, NodeId};
use smallvec::SmallVec;
use rustc_ast_pretty::pprust;
use rustc_ast::visit::{self, Visitor, try_visit};
use rustc_ast::mut_visit::{self, MutVisitor};

#[macro_export]
macro_rules! ast_fragments {
    (
        // The `$macro_body` captures all fragment definitions, which look like:
        //    Kind(Type) { "name"; one/many ...; fn make_kind; }
        $($macro_body:tt)*
    ) => {
        // Internal helper macro to dispatch based on whether a fragment is 'one' or 'many'.
        // This macro takes the full fragment definition as input.

        macro_rules! __dispatch_fragment_to_arm_generator {
            ( @mut_visit_with ($Kind:ident($AstTy:ty) {
                $kind_name:expr;
                one
                    fn $mut_visit_ast:ident;
                    fn $visit_ast:ident;
                    fn $ast_to_string:path;
                fn $make_ast:ident;
            }) ) => {
                // Pass only the directly relevant arguments to the simplified helper macro
                __generate_mut_visit_with_one_arm! { $Kind, $AstTy, $kind_name, $mut_visit_ast, $visit_ast, $ast_to_string, $make_ast }
            };
            ( @mut_visit_with ($Kind:ident($AstTy:ty) {
                $kind_name:expr;
                many
                    fn $flat_map_ast_elt:ident;
                    fn $visit_ast_elt:ident($args:tt); // Capture $args as it was originally
                    fn $ast_to_string_elt:path;
                fn $make_ast:ident;
            }) ) => {
                // Pass only the directly relevant arguments to the simplified helper macro
                __generate_mut_visit_with_many_arm! { $Kind, $AstTy, $kind_name, $flat_map_ast_elt, $visit_ast_elt, $args, $ast_to_string_elt, $make_ast }
            };
            ( @mut_visit_with ($_:tt)) => {}; // Catch-all for fragments not handled here (OptExpr, MethodReceiverExpr)
        
        
            ( @add_placeholders ($Kind:ident($AstTy:ty) {
                $kind_name:expr;
                one // add_placeholders doesn't apply to one fragments
                    fn $mut_visit_ast:ident; fn $visit_ast:ident; fn $ast_to_string:path;
                fn $make_ast:ident;
            }) ) => {}; // Do nothing for one fragments in add_placeholders
            ( @add_placeholders ($Kind:ident($AstTy:ty) {
                $kind_name:expr;
                many
                    fn $flat_map_ast_elt:ident;
                    fn $visit_ast_elt:ident($args:tt);
                    fn $ast_to_string_elt:path;
                fn $make_ast:ident;
            }) ) => {
                __generate_add_placeholders_arm! { $Kind, $AstTy, $kind_name, $flat_map_ast_elt, $visit_ast_elt, $args, $ast_to_string_elt, $make_ast }
            };
            ( @add_placeholders ($_:tt) ) => {}; // Catch-all for fragments not handled here
        
        
            ( @visit_with ($Kind:ident($AstTy:ty) {
                $kind_name:expr;
                one
                    fn $mut_visit_ast:ident;
                    fn $visit_ast:ident;
                    fn $ast_to_string:path;
                fn $make_ast:ident;
            }) ) => {
                __generate_visit_with_one_arm! { $Kind, $AstTy, $kind_name, $mut_visit_ast, $visit_ast, $ast_to_string, $make_ast }
            };
            ( @visit_with ($Kind:ident($AstTy:ty) {
                $kind_name:expr;
                many
                    fn $flat_map_ast_elt:ident;
                    fn $visit_ast_elt:ident($args:tt);
                    fn $ast_to_string_elt:path;
                fn $make_ast:ident;
            }) ) => {
                __generate_visit_with_many_arm! { $Kind, $AstTy, $kind_name, $flat_map_ast_elt, $visit_ast_elt, $args, $ast_to_string_elt, $make_ast }
            };
            ( @visit_with ($_:tt) ) => {}; // Catch-all for fragments not handled here
        
        
            ( @to_string ($Kind:ident($AstTy:ty) {
                $kind_name:expr;
                one
                    fn $mut_visit_ast:ident;
                    fn $visit_ast:ident;
                    fn $ast_to_string:path;
                fn $make_ast:ident;
            }) ) => {
                __generate_to_string_one_arm! { $Kind, $AstTy, $kind_name, $mut_visit_ast, $visit_ast, $ast_to_string, $make_ast }
            };
            ( @to_string ($Kind:ident($AstTy:ty) {
                $kind_name:expr;
                many
                    fn $flat_map_ast_elt:ident;
                    fn $visit_ast_elt:ident($args:tt);
                    fn $ast_to_string_elt:path;
                fn $make_ast:ident;
            }) ) => {
                __generate_to_string_many_arm! { $Kind, $AstTy, $kind_name, $flat_map_ast_elt, $visit_ast_elt, $args, $ast_to_string_elt, $make_ast }
            };
            ( @to_string ($_:tt) ) => {}; // Catch-all for fragments not handled here
        }
        // Now, the actual `impl AstFragment` block, which will use the helper macros
        // for generating the match arms within its methods.


        // Dummy `elems_to_string` to satisfy the macro. This will be replaced or properly defined here
        fn elems_to_string<T, F>(_elems: &[T], _f: F) -> String
        where
            F: Fn(&T) -> String,
        {
            // This is a placeholder, as the actual implementation needs context from ast_fragments.rs
            // For now, return an empty string or a placeholder message.
            String::from("")
        }
    };
}

// The actual invocation of the ast_fragments! macro.
// This defines the various AST fragment kinds and their associated visitor/mutator methods.
ast_fragments! {
    Expr(Box<ast::Expr>) {
        "expression";
        one fn visit_expr; fn visit_expr; fn pprust::expr_to_string;
        fn make_expr;
    }
    Pat(Box<ast::Pat>) {
        "pattern";
        one fn visit_pat; fn visit_pat; fn pprust::pat_to_string;
        fn make_pat;
    }
    Ty(Box<ast::Ty>) {
        "type";
        one fn visit_ty; fn visit_ty; fn pprust::ty_to_string;
        fn make_ty;
    }
    Stmts(SmallVec<ast::Stmt, 1>) {
        "statement";
        many fn flat_map_stmt; fn visit_stmt(); fn pprust::stmt_to_string;
        fn make_stmts;
    }
    Items(SmallVec<Box<ast::Item>, 1>) {
        "item";
        many fn flat_map_item; fn visit_item(); fn pprust::item_to_string;
        fn make_items;
    }
    TraitItems(SmallVec<Box<ast::AssocItem>, 1>) {
        "trait item";
        many fn flat_map_assoc_item; fn visit_assoc_item(AssocCtxt::Trait);
            fn pprust::assoc_item_to_string;
        fn make_trait_items;
    }
    ImplItems(SmallVec<Box<ast::AssocItem>, 1>) {
        "impl item";
        many fn flat_map_assoc_item; fn visit_assoc_item(AssocCtxt::Impl { of_trait: false });
            fn pprust::assoc_item_to_string;
        fn make_impl_items;
    }
    TraitImplItems(SmallVec<Box<ast::AssocItem>, 1>) {
        "impl item";
        many fn flat_map_assoc_item; fn visit_assoc_item(AssocCtxt::Impl { of_trait: true });
            fn pprust::assoc_item_to_string;
        fn make_trait_impl_items;
    }
    ForeignItems(SmallVec<Box<ast::ForeignItem>, 1>) {
        "foreign item";
        many fn flat_map_foreign_item; fn visit_foreign_item(); fn pprust::foreign_item_to_string;
        fn make_foreign_items;
    }
    Arms(SmallVec<ast::Arm, 1>) {
        "match arm";
        many fn flat_map_arm; fn visit_arm(); fn unreachable_to_string;
        fn make_arms;
    }
    ExprFields(SmallVec<ast::ExprField, 1>) {
        "field expression";
        many fn flat_map_expr_field; fn visit_expr_field(); fn unreachable_to_string;
        fn make_expr_fields;
    }
    PatFields(SmallVec<ast::PatField, 1>) {
        "field pattern";
        many fn flat_map_pat_field; fn visit_pat_field(); fn unreachable_to_string;
        fn make_pat_fields;
    }
    GenericParams(SmallVec<ast::GenericParam, 1>) {
        "generic parameter";
        many fn flat_map_generic_param; fn visit_generic_param(); fn unreachable_to_string;
        fn make_generic_params;
    }
    Params(SmallVec<ast::Param, 1>) {
        "function parameter";
        many fn flat_map_param; fn visit_param(); fn unreachable_to_string;
        fn make_params;
    }
    FieldDefs(SmallVec<ast::FieldDef, 1>) {
        "field";
        many fn flat_map_field_def; fn visit_field_def(); fn unreachable_to_string;
        fn make_field_defs;
    }
    Variants(SmallVec<ast::Variant, 1>) {
        "variant"; many fn flat_map_variant; fn visit_variant(); fn unreachable_to_string;
        fn make_variants;
    }
    WherePredicates(SmallVec<ast::WherePredicate, 1>) {
        "where predicate";
        many fn flat_map_where_predicate; fn visit_where_predicate(); fn unreachable_to_string;
        fn make_where_predicates;
    }
    Crate(ast::Crate) {
        "crate";
        one fn visit_crate; fn visit_crate; fn unreachable_to_string;
        fn make_crate;
    }
}
