macro_rules! deps {
    () => {
        RawVisibility!();
    };
}

macro_rules! ItemVisibilities {
    () => {
        deps!();
        # [derive (Default , Debug , Eq , PartialEq)] struct ItemVisibilities { arena : ThinVec < RawVisibility > , }
    };
}

ItemVisibilities!()