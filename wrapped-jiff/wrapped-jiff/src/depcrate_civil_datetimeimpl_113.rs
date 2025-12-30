// Generated macro for impl_113 (impl)
macro_rules! Depcrate_civil_datetimeimpl_113 {
() => {
// Module: crate::civil::datetime
// Provides: {"impl_113"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for DateTime { fn arbitrary (g : & mut quickcheck :: Gen) -> DateTime { let date = Date :: arbitrary (g) ; let time = Time :: arbitrary (g) ; DateTime :: from_parts (date , time) } fn shrink (& self) -> alloc :: boxed :: Box < dyn Iterator < Item = DateTime > > { alloc :: boxed :: Box :: new ((self . date () , self . time ()) . shrink () . map (| (date , time) | DateTime :: from_parts (date , time)) ,) } }
};
}
