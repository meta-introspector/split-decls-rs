// Generated macro for new (function)
macro_rules! Depcratenew {
() => {
// Module: crate
// Provides: {"new"}
// Dependencies: {}
pub fn new (properties : & CFDictionary < CFString , CFType >) -> IOSurface { unsafe { TCFType :: wrap_under_create_rule (IOSurfaceCreate (properties . as_concrete_TypeRef ())) } }
};
}
