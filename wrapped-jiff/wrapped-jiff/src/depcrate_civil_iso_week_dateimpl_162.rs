// Generated macro for impl_162 (impl)
macro_rules! Depcrate_civil_iso_week_dateimpl_162 {
() => {
// Module: crate::civil::iso_week_date
// Provides: {"impl_162"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for ISOWeekDate { fn arbitrary (g : & mut quickcheck :: Gen) -> ISOWeekDate { let year = ISOYear :: arbitrary (g) ; let week = ISOWeek :: arbitrary (g) ; let weekday = Weekday :: arbitrary (g) ; ISOWeekDate :: new_ranged_constrain (year , week , weekday) } fn shrink (& self) -> alloc :: boxed :: Box < dyn Iterator < Item = ISOWeekDate > > { alloc :: boxed :: Box :: new ((self . year_ranged () , self . week_ranged () , self . weekday ()) . shrink () . map (| (year , week , weekday) | { ISOWeekDate :: new_ranged_constrain (year , week , weekday) }) ,) } }
};
}
