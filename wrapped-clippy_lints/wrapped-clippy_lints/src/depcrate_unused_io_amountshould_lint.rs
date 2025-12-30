// Generated macro for should_lint (function)
macro_rules! Depcrate_unused_io_amountshould_lint {
() => {
// Module: crate::unused_io_amount
// Provides: {"should_lint"}
// Dependencies: {}
fn should_lint < 'a > (cx : & LateContext < 'a > , mut inner : & 'a hir :: Expr < 'a >) -> Option < IoOp > { inner = unpack_match (inner) ; inner = unpack_try (cx , inner) ; inner = unpack_call_chain (inner) ; inner = unpack_await (cx , inner) ; check_io_mode (cx , inner) }
};
}
