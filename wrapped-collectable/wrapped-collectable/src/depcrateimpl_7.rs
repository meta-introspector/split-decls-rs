// Generated macro for impl_7 (impl)
macro_rules! Depcrateimpl_7 {
() => {
// Module: crate
// Provides: {"impl_7"}
// Dependencies: {}
impl < A , C : Default + TryExtend < A > > TryFromIterator < A > for C { type Error = < Self as TryExtend < A > > :: Error ; fn try_from_iter < T > (iter : T) -> Result < Self , Self :: Error > where T : IntoIterator < Item = A > , { let mut collection = Self :: default () ; collection . try_extend (iter) ? ; Ok (collection) } }
};
}
