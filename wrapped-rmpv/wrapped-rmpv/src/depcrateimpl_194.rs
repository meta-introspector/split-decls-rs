// Generated macro for impl_194 (impl)
macro_rules! Depcrateimpl_194 {
() => {
// Module: crate
// Provides: {"impl_194"}
// Dependencies: {}
impl < 'a > From < Utf8StringRef < 'a > > for Utf8String { fn from (val : Utf8StringRef < 'a >) -> Self { match val . s { Ok (s) => Self { s : Ok (s . into ()) } , Err ((buf , err)) => Self { s : Err ((buf . into () , err)) , } , } } }
};
}
