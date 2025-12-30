// Generated macro for peel_place_mappers (function)
macro_rules! Depcrate_autorefspeel_place_mappers {
() => {
// Module: crate::autorefs
// Provides: {"peel_place_mappers"}
// Dependencies: {}
# [doc = " Peels expressions from `expr` that can map a place."] fn peel_place_mappers < 'tcx > (mut expr : & 'tcx Expr < 'tcx >) -> & 'tcx Expr < 'tcx > { loop { match expr . kind { ExprKind :: Index (base , _idx , _) => expr = & base , ExprKind :: Field (e , _) => expr = & e , _ => break expr , } } }
};
}
