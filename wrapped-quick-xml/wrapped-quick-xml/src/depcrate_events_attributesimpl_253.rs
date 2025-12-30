// Generated macro for impl_253 (impl)
macro_rules! Depcrate_events_attributesimpl_253 {
() => {
// Module: crate::events::attributes
// Provides: {"impl_253"}
// Dependencies: {}
impl < 'a > From < Attr < & 'a [u8] > > for Attribute < 'a > { # [inline] fn from (attr : Attr < & 'a [u8] >) -> Self { Self { key : attr . key () , value : Cow :: Borrowed (attr . value ()) , } } }
};
}
