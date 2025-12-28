macro_rules! DebugLineStr {
    () => {
        # [doc = " The `DebugLineStr` struct represents the DWARF strings"] # [doc = " found in the `.debug_line_str` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugLineStr < R > { section : R , }
    };
}

DebugLineStr!()