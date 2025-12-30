// Generated macro for dynamic (function)
macro_rules! Depcrate_unitdynamic {
() => {
// Module: crate::unit
// Provides: {"dynamic"}
// Dependencies: {}
# [doc = " Returns a unit that is a dynamic `label`."] pub fn dynamic (label : impl DisplayValue + Send + Sync + 'static) -> Unit { Unit { kind : Kind :: Dynamic (Arc :: new (label)) , mode : None , } }
};
}
