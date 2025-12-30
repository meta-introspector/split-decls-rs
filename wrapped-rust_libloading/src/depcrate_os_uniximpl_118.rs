// Generated macro for impl_118 (impl)
macro_rules! Depcrate_os_uniximpl_118 {
() => {
// Module: crate::os::unix
// Provides: {"impl_118"}
// Dependencies: {}
impl < T > Symbol < Option < T > > { # [doc = " Lift Option out of the symbol."] pub fn lift_option (self) -> Option < Symbol < T > > { if self . pointer . is_null () { None } else { Some (Symbol { pointer : self . pointer , pd : marker :: PhantomData , }) } } }
};
}
