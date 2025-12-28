macro_rules! DebugMacinfo {
    () => {
        # [doc = " The raw contents of the `.debug_macinfo` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugMacinfo < R > { pub (crate) section : R , }
    };
}

DebugMacinfo!();