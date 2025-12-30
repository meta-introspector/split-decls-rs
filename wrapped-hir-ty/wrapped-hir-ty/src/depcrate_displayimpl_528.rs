// Generated macro for impl_528 (impl)
macro_rules! Depcrate_displayimpl_528 {
() => {
// Module: crate::display
// Provides: {"impl_528"}
// Dependencies: {}
impl < 'db , T > fmt :: Display for HirDisplayWrapper < '_ , 'db , T > where T : HirDisplay < 'db > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self . write_to (f) { Ok (()) => Ok (()) , Err (HirDisplayError :: FmtError) => Err (fmt :: Error) , Err (HirDisplayError :: DisplaySourceCodeError (_)) => { panic ! ("HirDisplay::hir_fmt failed with DisplaySourceCodeError when calling Display::fmt!") } } } }
};
}
