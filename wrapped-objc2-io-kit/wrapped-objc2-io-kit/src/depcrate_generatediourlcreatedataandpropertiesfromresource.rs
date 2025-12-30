// Generated macro for IOURLCreateDataAndPropertiesFromResource (function)
macro_rules! Depcrate_generatedIOURLCreateDataAndPropertiesFromResource {
() => {
// Module: crate::generated
// Provides: {"IOURLCreateDataAndPropertiesFromResource"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - `alloc` might not allow `None`."] # [doc = " - `url` might not allow `None`."] # [doc = " - `resource_data` must be a valid pointer."] # [doc = " - `properties` must be a valid pointer."] # [doc = " - `desired_properties` generic must be of the correct type."] # [doc = " - `desired_properties` might not allow `None`."] # [doc = " - `error_code` must be a valid pointer."] # [inline] pub unsafe extern "C-unwind" fn IOURLCreateDataAndPropertiesFromResource (alloc : Option < & CFAllocator > , url : Option < & CFURL > , resource_data : * mut * const CFData , properties : * mut * const CFDictionary , desired_properties : Option < & CFArray > , error_code : * mut i32 ,) -> bool { extern "C-unwind" { fn IOURLCreateDataAndPropertiesFromResource (alloc : Option < & CFAllocator > , url : Option < & CFURL > , resource_data : * mut * const CFData , properties : * mut * const CFDictionary , desired_properties : Option < & CFArray > , error_code : * mut i32 ,) -> Boolean ; } let ret = unsafe { IOURLCreateDataAndPropertiesFromResource (alloc , url , resource_data , properties , desired_properties , error_code ,) } ; ret != 0 }
};
}
