// Generated macro for impl_35 (impl)
macro_rules! Depcrate_dateimpl_35 {
() => {
// Module: crate::date
// Provides: {"impl_35"}
// Dependencies: {}
impl < C , A > Ord for Date < A > where C : Calendar , C :: DateInner : Ord , A : AsCalendar < Calendar = C > , { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . inner . cmp (& other . inner) } }
};
}
