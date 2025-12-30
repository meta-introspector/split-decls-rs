// Generated macro for impl_237 (impl)
macro_rules! Depcrateimpl_237 {
() => {
// Module: crate
// Provides: {"impl_237"}
// Dependencies: {}
impl < T , S : Strategy < T > > Display for ArcSwapAny < T , S > where T : Display + RefCnt , { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . load () . fmt (formatter) } }
};
}
