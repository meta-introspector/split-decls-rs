// Generated macro for impl_53 (impl)
macro_rules! Depcrate_civil_dateimpl_53 {
() => {
// Module: crate::civil::date
// Provides: {"impl_53"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for Date { fn arbitrary (g : & mut quickcheck :: Gen) -> Date { let year = Year :: arbitrary (g) ; let month = Month :: arbitrary (g) ; let day = Day :: arbitrary (g) ; Date :: constrain_ranged (year , month , day) } fn shrink (& self) -> alloc :: boxed :: Box < dyn Iterator < Item = Date > > { alloc :: boxed :: Box :: new ((self . year_ranged () , self . month_ranged () , self . day_ranged ()) . shrink () . map (| (year , month , day) | { Date :: constrain_ranged (year , month , day) }) ,) } }
};
}
