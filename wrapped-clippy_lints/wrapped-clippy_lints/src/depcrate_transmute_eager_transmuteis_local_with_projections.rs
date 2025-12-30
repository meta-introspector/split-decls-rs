// Generated macro for is_local_with_projections (function)
macro_rules! Depcrate_transmute_eager_transmuteis_local_with_projections {
() => {
// Module: crate::transmute::eager_transmute
// Provides: {"is_local_with_projections"}
// Dependencies: {}
# [doc = " Checks if an expression is a path to a local variable (with optional projections), e.g."] # [doc = " `x.field[0].field2` would return true."] fn is_local_with_projections (expr : & Expr < '_ >) -> bool { path_to_local_with_projections (expr) . is_some () }
};
}
