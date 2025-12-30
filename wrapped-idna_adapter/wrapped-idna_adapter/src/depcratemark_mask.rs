// Generated macro for MARK_MASK (const)
macro_rules! DepcrateMARK_MASK {
() => {
// Module: crate
// Provides: {"MARK_MASK"}
// Dependencies: {}
# [doc = " Mask for the disallowed general categories of the first character in a label."] const MARK_MASK : u32 = general_category_to_mask (GeneralCategory :: NonspacingMark) | general_category_to_mask (GeneralCategory :: SpacingMark) | general_category_to_mask (GeneralCategory :: EnclosingMark) ;
};
}
