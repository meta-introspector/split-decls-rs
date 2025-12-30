// Generated macro for impl_230 (impl)
macro_rules! Depcrateimpl_230 {
() => {
// Module: crate
// Provides: {"impl_230"}
// Dependencies: {}
impl < T : Debug + RefCnt , S : Strategy < T > > Debug for Guard < T , S > { fn fmt (& self , formatter : & mut Formatter) -> FmtResult { self . deref () . fmt (formatter) } }
};
}
