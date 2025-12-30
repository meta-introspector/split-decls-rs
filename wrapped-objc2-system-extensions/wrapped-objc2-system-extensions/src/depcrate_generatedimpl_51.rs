// Generated macro for impl_51 (impl)
macro_rules! Depcrate_generatedimpl_51 {
() => {
// Module: crate::generated
// Provides: {"impl_51"}
// Dependencies: {}
impl OSSystemExtensionsWorkspace { extern_methods ! (# [unsafe (method (sharedWorkspace))] # [unsafe (method_family = none)] pub unsafe fn sharedWorkspace () -> Retained < OSSystemExtensionsWorkspace >; # [doc = " Start observing changes to System Extension(s) which are enabled or ready to be enabled."] # [unsafe (method (addObserver : error : _))] # [unsafe (method_family = none)] pub unsafe fn addObserver_error (& self , observer : & ProtocolObject < dyn OSSystemExtensionsWorkspaceObserver >,) -> Result < () , Retained < NSError >>; # [doc = " Stop observing changes to System Extension(s)."] # [unsafe (method (removeObserver :))] # [unsafe (method_family = none)] pub unsafe fn removeObserver (& self , observer : & ProtocolObject < dyn OSSystemExtensionsWorkspaceObserver >,) ; # [doc = " Get information about system extension(s) in an app with a bundle identifier"] # [doc = ""] # [doc = ""] # [doc = " Parameter `bundleID`: BundleIdentifier of the application containing the system extension(s)"] # [doc = ""] # [doc = " Parameter `out_error`: Error parameter to be populated with relevant error information"] # [doc = ""] # [doc = ""] # [doc = " Returns: A set of system extension property objects on success, nil otherwise."] # [unsafe (method (systemExtensionsForApplicationWithBundleID : error : _))] # [unsafe (method_family = none)] pub unsafe fn systemExtensionsForApplicationWithBundleID_error (& self , bundle_id : & NSString ,) -> Result < Retained < NSSet < OSSystemExtensionProperties >>, Retained < NSError >>;) ; }
};
}
