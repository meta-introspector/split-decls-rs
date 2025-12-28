macro_rules! deps {
    () => {
        BaseId!();
        FnvIndexSet!();
        CommonInformationEntry!();
        FrameDescriptionEntry!();
    };
}

macro_rules! FrameTable {
    () => {
        deps!();
        # [doc = " A table of frame description entries."] # [derive (Debug , Default)] pub struct FrameTable { # [doc = " Base id for CIEs."] base_id : BaseId , # [doc = " The common information entries."] cies : FnvIndexSet < CommonInformationEntry > , # [doc = " The frame description entries."] fdes : Vec < (CieId , FrameDescriptionEntry) > , }
    };
}

FrameTable!()