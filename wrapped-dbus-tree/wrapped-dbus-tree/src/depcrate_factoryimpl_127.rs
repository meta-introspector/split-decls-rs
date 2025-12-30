// Generated macro for impl_127 (impl)
macro_rules! Depcrate_factoryimpl_127 {
() => {
// Module: crate::factory
// Provides: {"impl_127"}
// Dependencies: {}
impl Factory < MTFn < () > , () > { # [doc = " Creates a new factory for single-thread use."] pub fn new_fn < D : DataType > () -> Factory < MTFn < D > , D > { Factory (IfaceCache :: new ()) } # [doc = " Creates a new factory for single-thread use, where callbacks can mutate their environment."] pub fn new_fnmut < D : DataType > () -> Factory < MTFnMut < D > , D > { Factory (IfaceCache :: new ()) } # [doc = " Creates a new factory for multi-thread use."] pub fn new_sync < D : DataType > () -> Factory < MTSync < D > , D > { Factory (IfaceCache :: new ()) } }
};
}
