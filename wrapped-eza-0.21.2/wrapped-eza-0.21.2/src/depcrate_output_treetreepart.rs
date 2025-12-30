// Generated macro for TreePart (enum)
macro_rules! Depcrate_output_treeTreePart {
() => {
// Module: crate::output::tree
// Provides: {"TreePart"}
// Dependencies: {}
# [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum TreePart { # [doc = " Rightmost column, *not* the last in the directory."] Edge , # [doc = " Not the rightmost column, and the directory has not finished yet."] Line , # [doc = " Rightmost column, and the last in the directory."] Corner , # [doc = " Not the rightmost column, and the directory *has* finished."] Blank , }
};
}
