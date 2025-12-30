// Generated macro for trim_tick (function)
macro_rules! Depcrate_readertrim_tick {
() => {
// Module: crate::reader
// Provides: {"trim_tick"}
// Dependencies: {}
fn trim_tick (name : & str) -> & str { if name . as_bytes () . iter () . rev () . nth (1) == Some (& b'`') { & name [.. name . len () - 2] } else { name } }
};
}
