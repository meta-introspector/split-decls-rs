// Generated macro for Symbol (struct)
macro_rules! Depcrate_os_unixSymbol {
() => {
// Module: crate::os::unix
// Provides: {"Symbol"}
// Dependencies: {}
# [doc = " Symbol from a library."] # [doc = ""] # [doc = " A major difference compared to the cross-platform `Symbol` is that this does not ensure that the"] # [doc = " `Symbol` does not outlive the `Library` it comes from."] pub struct Symbol < T > { pointer : * mut core :: ffi :: c_void , pd : marker :: PhantomData < T > , }
};
}
