macro_rules! DebugMacroOffset {
    () => {
        # [doc = " An offset into the `.debug_macro` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct DebugMacroOffset < T = usize > (pub T) ;
    };
}

DebugMacroOffset!();