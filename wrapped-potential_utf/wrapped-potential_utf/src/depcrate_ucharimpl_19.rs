// Generated macro for impl_19 (impl)
macro_rules! Depcrate_ucharimpl_19 {
() => {
// Module: crate::uchar
// Provides: {"impl_19"}
// Dependencies: {}
impl TryFrom < PotentialCodePoint > for char { type Error = core :: char :: CharTryFromError ; # [inline] fn try_from (value : PotentialCodePoint) -> Result < char , Self :: Error > { value . try_to_char () } }
};
}
