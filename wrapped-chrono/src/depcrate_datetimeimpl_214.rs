// Generated macro for impl_214 (impl)
macro_rules! Depcrate_datetimeimpl_214 {
() => {
// Module: crate::datetime
// Provides: {"impl_214"}
// Dependencies: {}
impl < Tz : TimeZone , Tz2 : TimeZone > PartialEq < DateTime < Tz2 > > for DateTime < Tz > { fn eq (& self , other : & DateTime < Tz2 >) -> bool { self . datetime == other . datetime } }
};
}
