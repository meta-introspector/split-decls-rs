macro_rules! deps {
    () => {
        DebugLoc!();
        DebugLocLists!();
    };
}

macro_rules! LocationLists {
    () => {
        deps!();
        # [doc = " The DWARF data found in `.debug_loc` and `.debug_loclists` sections."] # [derive (Debug , Default , Clone , Copy)] pub struct LocationLists < R > { debug_loc : DebugLoc < R > , debug_loclists : DebugLocLists < R > , }
    };
}

LocationLists!();