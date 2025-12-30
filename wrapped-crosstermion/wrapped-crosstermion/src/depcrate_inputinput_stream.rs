// Generated macro for input_stream (function)
macro_rules! Depcrate_inputinput_stream {
() => {
// Module: crate::input
// Provides: {"input_stream"}
// Dependencies: {}
# [doc = " Return a stream of input Events"] # [doc = ""] # [doc = " Requires the `input-async` feature."] # [cfg (feature = "input-async-crossterm")] pub fn input_stream () -> impl futures_core :: stream :: Stream < Item = Event > { use futures_lite :: StreamExt ; crossterm :: event :: EventStream :: new () . filter_map (| r | r . ok ()) }
};
}
