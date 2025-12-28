macro_rules! DebugAranges {
    () => {
        # [doc = " The `DebugAranges` struct represents the DWARF address range information"] # [doc = " found in the `.debug_aranges` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugAranges < R > { section : R , }
    };
}

DebugAranges!();