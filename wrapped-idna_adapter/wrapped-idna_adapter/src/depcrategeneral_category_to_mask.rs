// Generated macro for general_category_to_mask (function)
macro_rules! Depcrategeneral_category_to_mask {
() => {
// Module: crate
// Provides: {"general_category_to_mask"}
// Dependencies: {}
# [doc = " Turns a genecal category into a mask for comparing with multiple categories at once."] const fn general_category_to_mask (gc : GeneralCategory) -> u32 { 1 << (gc as u32) }
};
}
