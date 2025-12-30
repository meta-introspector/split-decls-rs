// Generated macro for impl_44 (impl)
macro_rules! Depcrate_errorimpl_44 {
() => {
// Module: crate::error
// Provides: {"impl_44"}
// Dependencies: {}
impl < T , R , F > From < Format < F > > for Info < T , R , F > where F : fmt :: Display , { fn from (s : Format < F >) -> Self { Info :: Format (s . 0) } }
};
}
