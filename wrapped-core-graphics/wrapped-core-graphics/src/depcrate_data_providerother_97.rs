// Generated macro for other_97 (other)
macro_rules! Depcrate_data_providerother_97 {
() => {
// Module: crate::data_provider
// Provides: {"other_97"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { fn CGDataProviderCopyData (provider : crate :: sys :: CGDataProviderRef) -> CFDataRef ; fn CGDataProviderCreateWithData (info : * mut c_void , data : * const c_void , size : usize , releaseData : CGDataProviderReleaseDataCallback ,) -> crate :: sys :: CGDataProviderRef ; fn CGDataProviderGetTypeID () -> CFTypeID ; }
};
}
