// Generated macro for bounded_range (function)
macro_rules! Depcrate_foreign_core_opsbounded_range {
() => {
// Module: crate::foreign::core::ops
// Provides: {"bounded_range"}
// Dependencies: {}
pub (crate) fn bounded_range < CB , I , R > (bounds : (I , I) , cb : CB) -> R where CB : Fn ((I , I)) -> R , I : PartialOrd , R : RangeBounds < I > , { let (mut start , mut end) = bounds ; if start > end { mem :: swap (& mut start , & mut end) ; } cb ((start , end)) }
};
}
