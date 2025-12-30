// Generated macro for impl_518 (impl)
macro_rules! Depcrate_common_rangeimpl_518 {
() => {
// Module: crate::common::range
// Provides: {"impl_518"}
// Dependencies: {}
impl Header for Range { fn name () -> & 'static HeaderName { & :: http :: header :: RANGE } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . next () . and_then (| val | { if val . to_str () . ok () ? . starts_with ("bytes=") { Some (Range (val . clone ())) } else { None } }) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { values . extend (:: std :: iter :: once (self . 0 . clone ())) ; } }
};
}
