// Generated macro for Symbol (struct)
macro_rules! Depcrate_os_windowsSymbol {
() => {
// Module: crate::os::windows
// Provides: {"Symbol"}
// Dependencies: {}
# [doc = " A symbol from a library."] # [doc = ""] # [doc = " A major difference compared to the cross-platform `Symbol` is that this does not ensure that the"] # [doc = " `Symbol` does not outlive the `Library` that it comes from."] pub struct Symbol < T > { pointer : FARPROC , pd : marker :: PhantomData < T > , }
};
}
