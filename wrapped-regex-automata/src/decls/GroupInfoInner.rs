macro_rules! deps {
    () => {
        SmallIndex!();
        GroupInfo!();
        CaptureNameMap!();
    };
}

macro_rules! GroupInfoInner {
    () => {
        deps!();
        # [doc = " The inner guts of `GroupInfo`. This type only exists so that it can"] # [doc = " be wrapped in an `Arc` to make `GroupInfo` reference counted."] # [derive (Debug , Default)] struct GroupInfoInner { slot_ranges : Vec < (SmallIndex , SmallIndex) > , name_to_index : Vec < CaptureNameMap > , index_to_name : Vec < Vec < Option < Arc < str > > > > , memory_extra : usize , }
    };
}

GroupInfoInner!();