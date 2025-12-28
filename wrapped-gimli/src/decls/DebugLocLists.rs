macro_rules! DebugLocLists {
    () => {
        # [doc = " The `DebugLocLists` struct represents the DWARF data"] # [doc = " found in the `.debug_loclists` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugLocLists < R > { section : R , }
    };
}

DebugLocLists!();