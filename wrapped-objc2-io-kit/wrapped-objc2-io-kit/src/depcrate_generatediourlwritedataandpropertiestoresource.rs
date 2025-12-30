// Generated macro for IOURLWriteDataAndPropertiesToResource (function)
macro_rules! Depcrate_generatedIOURLWriteDataAndPropertiesToResource {
() => {
// Module: crate::generated
// Provides: {"IOURLWriteDataAndPropertiesToResource"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - `url` might not allow `None`."] # [doc = " - `data_to_write` might not allow `None`."] # [doc = " - `properties_to_write` generic must be of the correct type."] # [doc = " - `properties_to_write` generic must be of the correct type."] # [doc = " - `properties_to_write` might not allow `None`."] # [doc = " - `error_code` must be a valid pointer."] # [inline] pub unsafe extern "C-unwind" fn IOURLWriteDataAndPropertiesToResource (url : Option < & CFURL > , data_to_write : Option < & CFData > , properties_to_write : Option < & CFDictionary > , error_code : * mut i32 ,) -> bool { extern "C-unwind" { fn IOURLWriteDataAndPropertiesToResource (url : Option < & CFURL > , data_to_write : Option < & CFData > , properties_to_write : Option < & CFDictionary > , error_code : * mut i32 ,) -> Boolean ; } let ret = unsafe { IOURLWriteDataAndPropertiesToResource (url , data_to_write , properties_to_write , error_code) } ; ret != 0 }
};
}
