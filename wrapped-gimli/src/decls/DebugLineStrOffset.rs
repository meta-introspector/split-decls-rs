macro_rules! DebugLineStrOffset {
    () => {
        # [doc = " An offset into the `.debug_line_str` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugLineStrOffset < T = usize > (pub T) ;
    };
}

DebugLineStrOffset!();