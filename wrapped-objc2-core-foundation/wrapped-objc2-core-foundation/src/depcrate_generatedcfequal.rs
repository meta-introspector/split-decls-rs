// Generated macro for CFEqual (function)
macro_rules! Depcrate_generatedCFEqual {
() => {
// Module: crate::generated
// Provides: {"CFEqual"}
// Dependencies: {}
# [inline] pub extern "C-unwind" fn CFEqual (cf1 : Option < & CFType > , cf2 : Option < & CFType >) -> bool { extern "C-unwind" { fn CFEqual (cf1 : Option < & CFType > , cf2 : Option < & CFType >) -> Boolean ; } let ret = unsafe { CFEqual (cf1 , cf2) } ; ret != 0 }
};
}
