macro_rules! DebugRanges {
    () => {
        # [doc = " The raw contents of the `.debug_ranges` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugRanges < R > { pub (crate) section : R , }
    };
}

DebugRanges!();