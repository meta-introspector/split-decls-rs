// Generated macro for label_and_mode (function)
macro_rules! Depcrate_unitlabel_and_mode {
() => {
// Module: crate::unit
// Provides: {"label_and_mode"}
// Dependencies: {}
# [doc = " Returns a unit that is a static `label` along with information on where to display a fraction and throughput."] pub fn label_and_mode (label : & 'static str , mode : display :: Mode) -> Unit { Unit { kind : Kind :: Label (label) , mode : Some (mode) , } }
};
}
