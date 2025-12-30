// Generated macro for impl_26 (impl)
macro_rules! Depcrateimpl_26 {
() => {
// Module: crate
// Provides: {"impl_26"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a > From < PercentEncode < 'a > > for Cow < 'a , str > { fn from (mut iter : PercentEncode < 'a >) -> Self { match iter . next () { None => "" . into () , Some (first) => match iter . next () { None => first . into () , Some (second) => { let mut string = first . to_owned () ; string . push_str (second) ; string . extend (iter) ; string . into () } } , } } }
};
}
