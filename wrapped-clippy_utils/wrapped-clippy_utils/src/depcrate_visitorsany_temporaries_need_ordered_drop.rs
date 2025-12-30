// Generated macro for any_temporaries_need_ordered_drop (function)
macro_rules! Depcrate_visitorsany_temporaries_need_ordered_drop {
() => {
// Module: crate::visitors
// Provides: {"any_temporaries_need_ordered_drop"}
// Dependencies: {}
pub fn any_temporaries_need_ordered_drop < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < 'tcx >) -> bool { for_each_unconsumed_temporary (cx , e , | ty | { if needs_ordered_drop (cx , ty) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } }) . is_break () }
};
}
