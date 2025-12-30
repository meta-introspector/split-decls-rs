// Generated macro for impl_102 (impl)
macro_rules! Depcrate_queryimpl_102 {
() => {
// Module: crate::query
// Provides: {"impl_102"}
// Dependencies: {}
impl < T : AsRef < str > > ToQuery for Raw < T > { type Error = Infallible ; fn to_query (& self) -> Result < Cow < '_ , str > , Self :: Error > { Ok (self . 0 . as_ref () . into ()) } }
};
}
