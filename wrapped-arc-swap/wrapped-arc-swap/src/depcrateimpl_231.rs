// Generated macro for impl_231 (impl)
macro_rules! Depcrateimpl_231 {
() => {
// Module: crate
// Provides: {"impl_231"}
// Dependencies: {}
impl < T : Display + RefCnt , S : Strategy < T > > Display for Guard < T , S > { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . deref () . fmt (formatter) } }
};
}
