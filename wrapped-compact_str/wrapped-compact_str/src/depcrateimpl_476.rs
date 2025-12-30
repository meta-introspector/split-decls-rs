// Generated macro for impl_476 (impl)
macro_rules! Depcrateimpl_476 {
() => {
// Module: crate
// Provides: {"impl_476"}
// Dependencies: {}
impl < 'a > From < Cow < 'a , str > > for CompactString { fn from (cow : Cow < 'a , str >) -> Self { match cow { Cow :: Borrowed (s) => s . into () , Cow :: Owned (s) => s . into () , } } }
};
}
