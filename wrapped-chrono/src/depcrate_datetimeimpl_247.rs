// Generated macro for impl_247 (impl)
macro_rules! Depcrate_datetimeimpl_247 {
() => {
// Module: crate::datetime
// Provides: {"impl_247"}
// Dependencies: {}
# [cfg (all (feature = "arbitrary" , feature = "std"))] impl < 'a , Tz > arbitrary :: Arbitrary < 'a > for DateTime < Tz > where Tz : TimeZone , < Tz as TimeZone > :: Offset : arbitrary :: Arbitrary < 'a > , { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < DateTime < Tz > > { let datetime = NaiveDateTime :: arbitrary (u) ? ; let offset = < Tz as TimeZone > :: Offset :: arbitrary (u) ? ; Ok (DateTime :: from_naive_utc_and_offset (datetime , offset)) } }
};
}
