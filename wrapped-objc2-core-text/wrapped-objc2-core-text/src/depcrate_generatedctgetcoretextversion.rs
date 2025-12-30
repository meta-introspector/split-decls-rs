// Generated macro for CTGetCoreTextVersion (function)
macro_rules! Depcrate_generatedCTGetCoreTextVersion {
() => {
// Module: crate::generated
// Provides: {"CTGetCoreTextVersion"}
// Dependencies: {}
# [doc = " Returns the version of the CoreText framework."] # [doc = ""] # [doc = ""] # [doc = " This function returns a number indicating the version of the"] # [doc = " CoreText framework. Note that framework version is not always"] # [doc = " an accurate indicator of feature availability. The recommended"] # [doc = " way to use this function is first to check that the function"] # [doc = " pointer is non-NULL, followed by calling it and comparing its"] # [doc = " result to a defined constant (or constants). For example, to"] # [doc = " determine whether the CoreText API is available:"] # [doc = " if ("] # [doc = " &CTGetCoreTextVersion"] # [doc = " != NULL"] # [doc = " &"] # [doc = " &"] # [doc = " CTGetCoreTextVersion() >= kCTVersionNumber10_5) {"] # [doc = " // CoreText API is available"] # [doc = " }"] # [doc = ""] # [doc = ""] # [doc = " Returns: The version number. This value is for comparison with the"] # [doc = " constants beginning with kCTVersionNumber and will not exceed"] # [doc = " kCTVersionNumber11_0."] # [deprecated = "Use -[NSProcessInfo operatingSystemVersion]"] # [inline] pub extern "C-unwind" fn CTGetCoreTextVersion () -> u32 { extern "C-unwind" { fn CTGetCoreTextVersion () -> u32 ; } unsafe { CTGetCoreTextVersion () } }
};
}
