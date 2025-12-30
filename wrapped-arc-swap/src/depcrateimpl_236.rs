// Generated macro for impl_236 (impl)
macro_rules! Depcrateimpl_236 {
() => {
// Module: crate
// Provides: {"impl_236"}
// Dependencies: {}
impl < T , S : Strategy < T > > Debug for ArcSwapAny < T , S > where T : Debug + RefCnt , { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { formatter . debug_tuple ("ArcSwapAny") . field (& self . load ()) . finish () } }
};
}
