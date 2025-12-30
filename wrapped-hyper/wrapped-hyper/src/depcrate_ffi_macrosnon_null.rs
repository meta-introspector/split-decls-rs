// Generated macro for non_null (macro)
macro_rules! Depcrate_ffi_macrosnon_null {
() => {
// Module: crate::ffi::macros
// Provides: {"non_null"}
// Dependencies: {}
macro_rules ! non_null { ($ ptr : ident , $ eval : expr , $ err : expr) => { { debug_assert ! (!$ ptr . is_null () , "{:?} must not be null" , stringify ! ($ ptr)) ; if $ ptr . is_null () { return $ err ; } unsafe { $ eval } } } ; (&*$ ptr : ident ?= $ err : expr) => { { non_null ! ($ ptr , &*$ ptr , $ err) } } ; (& mut *$ ptr : ident ?= $ err : expr) => { { non_null ! ($ ptr , & mut *$ ptr , $ err) } } ; (Box :: from_raw ($ ptr : ident) ?= $ err : expr) => { { non_null ! ($ ptr , Box :: from_raw ($ ptr) , $ err) } } ; (Arc :: from_raw ($ ptr : ident) ?= $ err : expr) => { { non_null ! ($ ptr , Arc :: from_raw ($ ptr) , $ err) } } ; }
};
}
