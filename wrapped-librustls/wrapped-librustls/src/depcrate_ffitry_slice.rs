// Generated macro for try_slice (macro)
macro_rules! Depcrate_ffitry_slice {
() => {
// Module: crate::ffi
// Provides: {"try_slice"}
// Dependencies: {}
macro_rules ! try_slice { ($ ptr : expr , $ count : expr) => { if $ ptr . is_null () { return $ crate :: panic :: NullParameterOrDefault :: value () ; } else { unsafe { slice :: from_raw_parts ($ ptr , $ count) } } } ; }
};
}
