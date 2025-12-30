// Generated macro for is_attribute (function)
macro_rules! Depcrate_undocumented_unsafe_blocksis_attribute {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"is_attribute"}
// Dependencies: {}
fn is_attribute (text : & str) -> bool { (text . starts_with ("#[") || text . starts_with ("#![")) && text . trim_end () . ends_with (']') }
};
}
