macro_rules! DebugInfo {
    () => {
        # [doc = " The `DebugInfo` struct represents the DWARF debugging information found in"] # [doc = " the `.debug_info` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugInfo < R > { debug_info_section : R , }
    };
}

DebugInfo!();