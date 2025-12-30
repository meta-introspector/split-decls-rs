// Generated macro for key_input_stream (function)
macro_rules! Depcrate_inputkey_input_stream {
() => {
// Module: crate::input
// Provides: {"key_input_stream"}
// Dependencies: {}
# [doc = " Return a stream of key input Events"] # [doc = ""] # [doc = " Requires the `input-async` feature."] # [cfg (feature = "input-async-crossterm")] pub fn key_input_stream () -> impl futures_core :: stream :: Stream < Item = Key > { use futures_lite :: StreamExt ; crossterm :: event :: EventStream :: new () . filter_map (| r | r . ok ()) . filter_map (| e | match e { crossterm :: event :: Event :: Key (key) => Some (key) , _ => None , }) }
};
}
