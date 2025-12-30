// Generated macro for impl_63 (impl)
macro_rules! Depcrate_generatedimpl_63 {
() => {
// Module: crate::generated
// Provides: {"impl_63"}
// Dependencies: {}
impl NEAppProxyProviderManager { extern_methods ! (# [cfg (feature = "block2")] # [doc = " This function asynchronously reads all of the NEAppProxy configurations associated with the calling app that have previously been saved to disk and returns them as NEAppProxyProviderManager objects."] # [doc = ""] # [doc = " Parameter `completionHandler`: A block that takes an array NEAppProxyProviderManager objects. The array passed to the block may be empty if no NETunnelProvider configurations were successfully read from the disk.  The NSError passed to this block will be nil if the load operation succeeded, non-nil otherwise."] # [unsafe (method (loadAllFromPreferencesWithCompletionHandler :))] # [unsafe (method_family = none)] pub unsafe fn loadAllFromPreferencesWithCompletionHandler (completion_handler : & block2 :: DynBlock < dyn Fn (* mut NSArray < NEAppProxyProviderManager >, * mut NSError) , >,) ;) ; }
};
}
