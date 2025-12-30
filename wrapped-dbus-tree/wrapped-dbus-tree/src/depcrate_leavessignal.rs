// Generated macro for Signal (struct)
macro_rules! Depcrate_leavesSignal {
() => {
// Module: crate::leaves
// Provides: {"Signal"}
// Dependencies: {}
# [derive (Debug)] # [doc = " A D-Bus Signal."] pub struct Signal < D : DataType > { name : Member < 'static > , data : D :: Signal , arguments : Vec < Argument > , anns : Annotations , }
};
}
