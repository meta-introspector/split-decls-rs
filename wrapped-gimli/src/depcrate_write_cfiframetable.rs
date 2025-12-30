// Generated macro for FrameTable (struct)
macro_rules! Depcrate_write_cfiFrameTable {
() => {
// Module: crate::write::cfi
// Provides: {"FrameTable"}
// Dependencies: {}
# [doc = " A table of frame description entries."] # [derive (Debug , Default)] pub struct FrameTable { # [doc = " Base id for CIEs."] base_id : BaseId , # [doc = " The common information entries."] cies : FnvIndexSet < CommonInformationEntry > , # [doc = " The frame description entries."] fdes : Vec < (CieId , FrameDescriptionEntry) > , }
};
}
