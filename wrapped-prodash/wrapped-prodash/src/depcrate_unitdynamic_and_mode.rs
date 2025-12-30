// Generated macro for dynamic_and_mode (function)
macro_rules! Depcrate_unitdynamic_and_mode {
() => {
// Module: crate::unit
// Provides: {"dynamic_and_mode"}
// Dependencies: {}
# [doc = " Returns a unit that is a dynamic `label` along with information on where to display a fraction and throughput."] pub fn dynamic_and_mode (label : impl DisplayValue + Send + Sync + 'static , mode : display :: Mode) -> Unit { Unit { kind : Kind :: Dynamic (Arc :: new (label)) , mode : Some (mode) , } }
};
}
