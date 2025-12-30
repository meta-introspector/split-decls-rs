// Generated macro for is_adjusted (function)
macro_rules! Depcrateis_adjusted {
() => {
// Module: crate
// Provides: {"is_adjusted"}
// Dependencies: {}
# [doc = " Returns `true` if the given `Expr` has been coerced before."] # [doc = ""] # [doc = " Examples of coercions can be found in the Nomicon at"] # [doc = " <https://doc.rust-lang.org/nomicon/coercions.html>."] # [doc = ""] # [doc = " See `rustc_middle::ty::adjustment::Adjustment` and `rustc_hir_analysis::check::coercion` for"] # [doc = " more information on adjustments and coercions."] pub fn is_adjusted (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { cx . typeck_results () . adjustments () . get (e . hir_id) . is_some () }
};
}
