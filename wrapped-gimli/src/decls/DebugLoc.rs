macro_rules! DebugLoc {
    () => {
        # [doc = " The raw contents of the `.debug_loc` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugLoc < R > { pub (crate) section : R , }
    };
}

DebugLoc!();