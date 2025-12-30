// Generated macro for make_size_square (function)
macro_rules! Depcrate_core_geometrymake_size_square {
() => {
// Module: crate::core::geometry
// Provides: {"make_size_square"}
// Dependencies: {}
# [doc = " Make the shape have the same X and Y values."] pub fn make_size_square (sz : Point) -> Point { let l = sz . x . max (sz . y) ; Point :: new (l , l) }
};
}
