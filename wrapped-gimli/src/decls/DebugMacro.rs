macro_rules! DebugMacro {
    () => {
        # [doc = " The raw contents of the `.debug_macro` section."] # [derive (Debug , Default , Clone , Copy)] pub struct DebugMacro < R > { pub (crate) section : R , }
    };
}

DebugMacro!();