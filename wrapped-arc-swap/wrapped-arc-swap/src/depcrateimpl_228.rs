// Generated macro for impl_228 (impl)
macro_rules! Depcrateimpl_228 {
() => {
// Module: crate
// Provides: {"impl_228"}
// Dependencies: {}
impl < T : RefCnt , S : Strategy < T > > From < T > for Guard < T , S > { fn from (inner : T) -> Self { Self :: from_inner (inner) } }
};
}
