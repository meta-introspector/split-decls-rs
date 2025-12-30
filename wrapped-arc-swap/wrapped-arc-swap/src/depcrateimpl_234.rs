// Generated macro for impl_234 (impl)
macro_rules! Depcrateimpl_234 {
() => {
// Module: crate
// Provides: {"impl_234"}
// Dependencies: {}
impl < T : RefCnt , S : Default + Strategy < T > > From < T > for ArcSwapAny < T , S > { fn from (val : T) -> Self { Self :: with_strategy (val , S :: default ()) } }
};
}
