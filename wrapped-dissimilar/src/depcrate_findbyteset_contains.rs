// Generated macro for byteset_contains (function)
macro_rules! Depcrate_findbyteset_contains {
() => {
// Module: crate::find
// Provides: {"byteset_contains"}
// Dependencies: {}
fn byteset_contains (byteset : u64 , ch : char) -> bool { (byteset >> ((ch as u8 & 0x3f) as usize)) & 1 != 0 }
};
}
