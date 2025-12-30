// Generated macro for impl_142 (impl)
macro_rules! Depcrate_os_windowsimpl_142 {
() => {
// Module: crate::os::windows
// Provides: {"impl_142"}
// Dependencies: {}
impl < T > Symbol < Option < T > > { # [doc = " Lift Option out of the symbol."] pub fn lift_option (self) -> Option < Symbol < T > > { if self . pointer . is_none () { None } else { Some (Symbol { pointer : self . pointer , pd : marker :: PhantomData , }) } } }
};
}
