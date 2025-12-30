// Generated macro for impl_244 (impl)
macro_rules! Depcrate_tagimpl_244 {
() => {
// Module: crate::tag
// Provides: {"impl_244"}
// Dependencies: {}
# [doc = " Types which are [`FixedTag`] always have a known [`Tag`] type."] impl < T : FixedTag + ? Sized > Tagged for T { fn tag (& self) -> Tag { T :: TAG } }
};
}
