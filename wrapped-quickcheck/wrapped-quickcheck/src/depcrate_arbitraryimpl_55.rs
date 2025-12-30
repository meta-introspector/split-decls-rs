// Generated macro for impl_55 (impl)
macro_rules! Depcrate_arbitraryimpl_55 {
() => {
// Module: crate::arbitrary
// Provides: {"impl_55"}
// Dependencies: {}
impl Arbitrary for OsString { fn arbitrary (g : & mut Gen) -> OsString { OsString :: from (String :: arbitrary (g)) } fn shrink (& self) -> Box < dyn Iterator < Item = OsString > > { let mystring : String = self . clone () . into_string () . unwrap () ; Box :: new (mystring . shrink () . map (OsString :: from)) } }
};
}
