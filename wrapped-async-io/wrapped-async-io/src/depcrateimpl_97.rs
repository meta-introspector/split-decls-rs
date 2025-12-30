// Generated macro for impl_97 (impl)
macro_rules! Depcrateimpl_97 {
() => {
// Module: crate
// Provides: {"impl_97"}
// Dependencies: {}
# [cfg (unix)] impl < T : AsFd + From < OwnedFd > > TryFrom < OwnedFd > for Async < T > { type Error = io :: Error ; fn try_from (value : OwnedFd) -> Result < Self , Self :: Error > { Async :: new (value . into ()) } }
};
}
