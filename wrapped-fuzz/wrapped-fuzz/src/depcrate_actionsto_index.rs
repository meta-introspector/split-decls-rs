// Generated macro for to_index (function)
macro_rules! Depcrate_actionsto_index {
() => {
// Module: crate::actions
// Provides: {"to_index"}
// Dependencies: {}
fn to_index (s : & str , idx : u8) -> usize { s . char_indices () . map (| (idx , _) | idx) . chain ([s . len ()]) . cycle () . nth (idx as usize) . unwrap_or_default () }
};
}
