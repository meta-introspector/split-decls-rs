// Generated macro for macro_46 (macro)
macro_rules! Depcrate_generatedmacro_46 {
() => {
// Module: crate::generated
// Provides: {"macro_46"}
// Dependencies: {}
extern_protocol ! (# [doc = " [Apple's documentation](https://developer.apple.com/documentation/systemextensions/ossystemextensionsworkspaceobserver?language=objc)"] pub unsafe trait OSSystemExtensionsWorkspaceObserver : NSObjectProtocol { # [doc = " This delegate method will be called when a system extension has been validated and allowed by the user to run."] # [optional] # [unsafe (method (systemExtensionWillBecomeEnabled :))] # [unsafe (method_family = none)] unsafe fn systemExtensionWillBecomeEnabled (& self , system_extension_info : & OSSystemExtensionInfo ,) ; # [doc = " This delegate method will be called when the user disables an already enabled system extension, or when the system extension is first installed and is in the disabled state."] # [optional] # [unsafe (method (systemExtensionWillBecomeDisabled :))] # [unsafe (method_family = none)] unsafe fn systemExtensionWillBecomeDisabled (& self , system_extension_info : & OSSystemExtensionInfo ,) ; # [doc = " This delegate method will be called when a system extension is deactivated and is about to get uninstalled. The extension may still be running until the system is rebooted."] # [optional] # [unsafe (method (systemExtensionWillBecomeInactive :))] # [unsafe (method_family = none)] unsafe fn systemExtensionWillBecomeInactive (& self , system_extension_info : & OSSystemExtensionInfo ,) ; }) ;
};
}
