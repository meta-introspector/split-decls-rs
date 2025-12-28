macro_rules! DebugStr {
    () => {
        # [doc = " The `DebugStr` struct represents the DWARF strings"] # [doc = " found in the `.debug_str` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugStr < R > { debug_str_section : R , }
    };
}

DebugStr!();