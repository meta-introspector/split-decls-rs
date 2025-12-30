// Generated macro for IOCFUnserializeBinary (function)
macro_rules! Depcrate_generatedIOCFUnserializeBinary {
() => {
// Module: crate::generated
// Provides: {"IOCFUnserializeBinary"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - `buffer` must be a valid pointer."] # [doc = " - `allocator` might not allow `None`."] # [doc = " - `error_string` must be a valid pointer."] # [inline] pub unsafe extern "C-unwind" fn IOCFUnserializeBinary (buffer : * const c_char , buffer_size : usize , allocator : Option < & CFAllocator > , options : CFOptionFlags , error_string : * mut * const CFString ,) -> Option < CFRetained < CFType > > { extern "C-unwind" { fn IOCFUnserializeBinary (buffer : * const c_char , buffer_size : usize , allocator : Option < & CFAllocator > , options : CFOptionFlags , error_string : * mut * const CFString ,) -> Option < NonNull < CFType > > ; } let ret = unsafe { IOCFUnserializeBinary (buffer , buffer_size , allocator , options , error_string) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
