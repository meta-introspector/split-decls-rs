// Generated macro for impl_249 (impl)
macro_rules! Depcrate_civil_weekdayimpl_249 {
() => {
// Module: crate::civil::weekday
// Provides: {"impl_249"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for Weekday { fn arbitrary (g : & mut quickcheck :: Gen) -> Weekday { let offset = t :: WeekdayZero :: arbitrary (g) ; Weekday :: from_monday_zero_offset_ranged (offset) } fn shrink (& self) -> alloc :: boxed :: Box < dyn Iterator < Item = Weekday > > { alloc :: boxed :: Box :: new (self . to_monday_zero_offset_ranged () . shrink () . map (Weekday :: from_monday_zero_offset_ranged) ,) } }
};
}
