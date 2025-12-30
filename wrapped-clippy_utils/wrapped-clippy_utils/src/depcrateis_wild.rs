// Generated macro for is_wild (function)
macro_rules! Depcrateis_wild {
() => {
// Module: crate
// Provides: {"is_wild"}
// Dependencies: {}
# [doc = " Checks if given pattern is a wildcard (`_`)"] pub fn is_wild (pat : & Pat < '_ >) -> bool { matches ! (pat . kind , PatKind :: Wild) }
};
}
