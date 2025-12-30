// Generated macro for impl_603 (impl)
macro_rules! Depcrate_displayimpl_603 {
() => {
// Module: crate::display
// Provides: {"impl_603"}
// Dependencies: {}
impl < T > fmt :: Display for HirDisplayWrapper < '_ , T > where T : HirDisplay , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . write_to (f) { Ok (()) => Ok (()) , Err (HirDisplayError :: FmtError) => Err (fmt :: Error) , Err (HirDisplayError :: DisplaySourceCodeError (_)) => { panic ! ("HirDisplay::hir_fmt failed with DisplaySourceCodeError when calling Display::fmt!") } } } }
};
}
