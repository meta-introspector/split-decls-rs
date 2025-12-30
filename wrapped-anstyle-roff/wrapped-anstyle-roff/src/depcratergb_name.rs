// Generated macro for rgb_name (function)
macro_rules! Depcratergb_name {
() => {
// Module: crate
// Provides: {"rgb_name"}
// Dependencies: {}
fn rgb_name (c : & RgbColor) -> String { format ! ("hex_{}" , to_hex (c) . as_str ()) }
};
}
