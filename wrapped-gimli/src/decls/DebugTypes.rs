macro_rules! DebugTypes {
    () => {
        # [doc = " The `DebugTypes` struct represents the DWARF type information"] # [doc = " found in the `.debug_types` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugTypes < R > { debug_types_section : R , }
    };
}

DebugTypes!();