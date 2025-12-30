// Generated macro for TagSliceExt (trait)
macro_rules! Depcrate_control_tagTagSliceExt {
() => {
// Module: crate::control::tag
// Provides: {"TagSliceExt"}
// Dependencies: {}
# [doc = " Extension trait for slices of tags."] pub (crate) trait TagSliceExt { # [doc = " Fills the control with the given tag."] fn fill_tag (& mut self , tag : Tag) ; # [doc = " Clears out the control."] # [inline] fn fill_empty (& mut self) { self . fill_tag (Tag :: EMPTY) } }
};
}
