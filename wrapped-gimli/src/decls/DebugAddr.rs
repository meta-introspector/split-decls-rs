macro_rules! DebugAddr {
    () => {
        # [doc = " The raw contents of the `.debug_addr` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugAddr < R > { section : R , }
    };
}

DebugAddr!();