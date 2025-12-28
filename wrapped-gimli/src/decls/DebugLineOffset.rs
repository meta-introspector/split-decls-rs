macro_rules! DebugLineOffset {
    () => {
        # [doc = " An offset into the `.debug_line` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugLineOffset < T = usize > (pub T) ;
    };
}

DebugLineOffset!();