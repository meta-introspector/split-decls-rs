// Generated macro for impl_34 (impl)
macro_rules! Depcrate_dateimpl_34 {
() => {
// Module: crate::date
// Provides: {"impl_34"}
// Dependencies: {}
impl < C , A , B > PartialOrd < Date < B > > for Date < A > where C : Calendar , A : AsCalendar < Calendar = C > , B : AsCalendar < Calendar = C > , { fn partial_cmp (& self , other : & Date < B >) -> Option < core :: cmp :: Ordering > { self . inner . partial_cmp (& other . inner) } }
};
}
