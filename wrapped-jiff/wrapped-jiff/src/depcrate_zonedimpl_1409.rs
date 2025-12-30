// Generated macro for impl_1409 (impl)
macro_rules! Depcrate_zonedimpl_1409 {
() => {
// Module: crate::zoned
// Provides: {"impl_1409"}
// Dependencies: {}
# [cfg (test)] impl quickcheck :: Arbitrary for Zoned { fn arbitrary (g : & mut quickcheck :: Gen) -> Zoned { let timestamp = Timestamp :: arbitrary (g) ; let tz = TimeZone :: UTC ; Zoned :: new (timestamp , tz) } fn shrink (& self) -> alloc :: boxed :: Box < dyn Iterator < Item = Self > > { let timestamp = self . timestamp () ; alloc :: boxed :: Box :: new (timestamp . shrink () . map (| timestamp | Zoned :: new (timestamp , TimeZone :: UTC)) ,) } }
};
}
