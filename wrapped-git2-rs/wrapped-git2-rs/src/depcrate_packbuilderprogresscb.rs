// Generated macro for ProgressCb (type)
macro_rules! Depcrate_packbuilderProgressCb {
() => {
// Module: crate::packbuilder
// Provides: {"ProgressCb"}
// Dependencies: {}
pub type ProgressCb < 'a > = dyn FnMut (PackBuilderStage , u32 , u32) -> bool + 'a ;
};
}
