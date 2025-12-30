// Generated macro for IOCFSerialize (function)
macro_rules! Depcrate_generatedIOCFSerialize {
() => {
// Module: crate::generated
// Provides: {"IOCFSerialize"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - `object` should be of the correct type."] # [doc = " - `object` might not allow `None`."] # [inline] pub unsafe extern "C-unwind" fn IOCFSerialize (object : Option < & CFType > , options : CFOptionFlags ,) -> Option < CFRetained < CFData > > { extern "C-unwind" { fn IOCFSerialize (object : Option < & CFType > , options : CFOptionFlags ,) -> Option < NonNull < CFData > > ; } let ret = unsafe { IOCFSerialize (object , options) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) }
};
}
