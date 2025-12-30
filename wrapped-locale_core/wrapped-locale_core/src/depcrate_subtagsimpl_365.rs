// Generated macro for impl_365 (impl)
macro_rules! Depcrate_subtagsimpl_365 {
() => {
// Module: crate::subtags
// Provides: {"impl_365"}
// Dependencies: {}
impl < const N : usize > TryFrom < tinystr :: TinyAsciiStr < N > > for Subtag { type Error = crate :: parser :: errors :: ParseError ; fn try_from (value : tinystr :: TinyAsciiStr < N >) -> Result < Self , Self :: Error > { Self :: try_from_str (& value) } }
};
}
