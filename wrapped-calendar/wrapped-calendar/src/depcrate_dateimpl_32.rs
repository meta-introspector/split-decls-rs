// Generated macro for impl_32 (impl)
macro_rules! Depcrate_dateimpl_32 {
() => {
// Module: crate::date
// Provides: {"impl_32"}
// Dependencies: {}
impl < C , A , B > PartialEq < Date < B > > for Date < A > where C : Calendar , A : AsCalendar < Calendar = C > , B : AsCalendar < Calendar = C > , { fn eq (& self , other : & Date < B >) -> bool { self . inner . eq (& other . inner) } }
};
}
