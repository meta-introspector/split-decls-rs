// Generated macro for IOCFUnserialize (function)
macro_rules! Depcrate_generatedIOCFUnserialize {
() => {
// Module: crate::generated
// Provides: {"IOCFUnserialize"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - `buffer` must be a valid pointer."] # [doc = " - `allocator` might not allow `None`."] # [doc = " - `error_string` must be a valid pointer."] # [inline] pub unsafe extern "C-unwind" fn IOCFUnserialize (buffer : * const c_char , allocator : Option < & CFAllocator > , options : CFOptionFlags , error_string : * mut * const CFString ,) -> Option < CFRetained < CFType > > { extern "C-unwind" { fn IOCFUnserialize (buffer : * const c_char , allocator : Option < & CFAllocator > , options : CFOptionFlags , error_string : * mut * const CFString ,) -> Option < NonNull < CFType > > ; } let ret = unsafe { IOCFUnserialize (buffer , allocator , options , error_string) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
