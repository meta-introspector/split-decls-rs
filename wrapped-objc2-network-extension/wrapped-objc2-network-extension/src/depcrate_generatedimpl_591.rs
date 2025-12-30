// Generated macro for impl_591 (impl)
macro_rules! Depcrate_generatedimpl_591 {
() => {
// Module: crate::generated
// Provides: {"impl_591"}
// Dependencies: {}
impl NETransparentProxyManager { extern_methods ! (# [cfg (feature = "block2")] # [doc = " This function asynchronously reads all of the transparent proxy configurations associated with the calling app that have previously been saved to disk and returns them as NETransparentProxyManager objects."] # [doc = ""] # [doc = " Parameter `completionHandler`: A block that takes an array NETransparentProxyManager objects. The array passed to the block may be empty if no transparent proxy configurations were successfully read from the disk.  The NSError passed to this block will be nil if the load operation succeeded, non-nil otherwise."] # [unsafe (method (loadAllFromPreferencesWithCompletionHandler :))] # [unsafe (method_family = none)] pub unsafe fn loadAllFromPreferencesWithCompletionHandler (completion_handler : & block2 :: DynBlock < dyn Fn (* mut NSArray < NETransparentProxyManager >, * mut NSError) , >,) ;) ; }
};
}
