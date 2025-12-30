// Generated macro for CFCopyTypeIDDescription (function)
macro_rules! Depcrate_generatedCFCopyTypeIDDescription {
() => {
// Module: crate::generated
// Provides: {"CFCopyTypeIDDescription"}
// Dependencies: {}
# [inline] pub extern "C-unwind" fn CFCopyTypeIDDescription (type_id : CFTypeID ,) -> Option < CFRetained < CFString > > { extern "C-unwind" { fn CFCopyTypeIDDescription (type_id : CFTypeID) -> Option < NonNull < CFString > > ; } let ret = unsafe { CFCopyTypeIDDescription (type_id) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
