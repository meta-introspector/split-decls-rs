// Generated macro for impl_189 (impl)
macro_rules! Depcrateimpl_189 {
() => {
// Module: crate
// Provides: {"impl_189"}
// Dependencies: {}
impl < 'a > From < Cow < 'a , str > > for Utf8String { # [inline] fn from (val : Cow < 'a , str >) -> Self { Self { s : Ok (val . into_owned ()) , } } }
};
}
