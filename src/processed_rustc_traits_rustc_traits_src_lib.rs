// SRC: ../rust/compiler/rustc_traits/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=provide | COMPLEXITY=3 | LINES=29 */
// Queries that are independent from the main solver code.

// tidy-alphabetical-start
#[recursion_limit = "256"]
// tidy-alphabetical-end


use crate::rustc_complete::query::Providers;
pub use crate::rustc_trait_selection::traits::query::type_op::ascribe_user_type::type_op_ascribe_user_type_with_span;
pub use type_op::type_op_prove_predicate_with_cause;

pub fn provide(p: &mut Providers) {
    dropck_outlives::provide(p);
    evaluate_obligation::provide(p);
    implied_outlives_bounds::provide(p);
    normalize_projection_ty::provide(p);
    normalize_erasing_regions::provide(p);
    type_op::provide(p);
    p.codegen_select_candidate = codegen::codegen_select_candidate;
    p.coroutine_hidden_types = coroutine_witnesses::coroutine_hidden_types;
}