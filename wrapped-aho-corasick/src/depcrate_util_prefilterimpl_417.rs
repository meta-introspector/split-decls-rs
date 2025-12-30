// Generated macro for impl_417 (impl)
macro_rules! Depcrate_util_prefilterimpl_417 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_417"}
// Dependencies: {}
impl < P : PrefilterI + ? Sized > PrefilterI for Arc < P > { # [inline (always)] fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { (* * self) . find_in (haystack , span) } }
};
}
