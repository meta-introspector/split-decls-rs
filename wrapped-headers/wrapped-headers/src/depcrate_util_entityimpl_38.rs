// Generated macro for impl_38 (impl)
macro_rules! Depcrate_util_entityimpl_38 {
() => {
// Module: crate::util::entity
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a > From < & 'a EntityTagRange > for HeaderValue { fn from (tag : & 'a EntityTagRange) -> HeaderValue { match * tag { EntityTagRange :: Any => HeaderValue :: from_static ("*") , EntityTagRange :: Tags (ref tags) => tags . into () , } } }
};
}
