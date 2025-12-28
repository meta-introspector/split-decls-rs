macro_rules! deps {
    () => {
        DebugRanges!();
        DebugRngLists!();
    };
}

macro_rules! RangeLists {
    () => {
        deps!();
        # [doc = " The DWARF data found in `.debug_ranges` and `.debug_rnglists` sections."] # [derive (Debug , Default , Clone , Copy)] pub struct RangeLists < R > { debug_ranges : DebugRanges < R > , debug_rnglists : DebugRngLists < R > , }
    };
}

RangeLists!();