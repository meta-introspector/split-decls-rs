// Generated macro for IOURLCreatePropertyFromResource (function)
macro_rules! Depcrate_generatedIOURLCreatePropertyFromResource {
() => {
// Module: crate::generated
// Provides: {"IOURLCreatePropertyFromResource"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - `alloc` might not allow `None`."] # [doc = " - `url` might not allow `None`."] # [doc = " - `property` might not allow `None`."] # [doc = " - `error_code` must be a valid pointer."] # [inline] pub unsafe extern "C-unwind" fn IOURLCreatePropertyFromResource (alloc : Option < & CFAllocator > , url : Option < & CFURL > , property : Option < & CFString > , error_code : * mut i32 ,) -> Option < CFRetained < CFType > > { extern "C-unwind" { fn IOURLCreatePropertyFromResource (alloc : Option < & CFAllocator > , url : Option < & CFURL > , property : Option < & CFString > , error_code : * mut i32 ,) -> Option < NonNull < CFType > > ; } let ret = unsafe { IOURLCreatePropertyFromResource (alloc , url , property , error_code) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
