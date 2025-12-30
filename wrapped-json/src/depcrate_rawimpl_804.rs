// Generated macro for impl_804 (impl)
macro_rules! Depcrate_rawimpl_804 {
() => {
// Module: crate::raw
// Provides: {"impl_804"}
// Dependencies: {}
impl RawValue { const fn from_borrowed (json : & str) -> & Self { unsafe { mem :: transmute :: < & str , & RawValue > (json) } } fn from_owned (json : Box < str >) -> Box < Self > { unsafe { mem :: transmute :: < Box < str > , Box < RawValue > > (json) } } fn into_owned (raw_value : Box < Self >) -> Box < str > { unsafe { mem :: transmute :: < Box < RawValue > , Box < str > > (raw_value) } } }
};
}
