// Generated macro for try_slice_mut (macro)
macro_rules! Depcrate_ffitry_slice_mut {
() => {
// Module: crate::ffi
// Provides: {"try_slice_mut"}
// Dependencies: {}
macro_rules ! try_slice_mut { ($ ptr : expr , $ count : expr) => { if $ ptr . is_null () { return $ crate :: panic :: NullParameterOrDefault :: value () ; } else { unsafe { slice :: from_raw_parts_mut ($ ptr , $ count) } } } ; }
};
}
