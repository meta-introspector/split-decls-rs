// Generated macro for CFCopyDescription (function)
macro_rules! Depcrate_generatedCFCopyDescription {
() => {
// Module: crate::generated
// Provides: {"CFCopyDescription"}
// Dependencies: {}
# [inline] pub extern "C-unwind" fn CFCopyDescription (cf : Option < & CFType >) -> Option < CFRetained < CFString > > { extern "C-unwind" { fn CFCopyDescription (cf : Option < & CFType >) -> Option < NonNull < CFString > > ; } let ret = unsafe { CFCopyDescription (cf) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
