macro_rules! DebugStrOffset {
    () => {
        # [doc = " An offset into the `.debug_str` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugStrOffset < T = usize > (pub T) ;
    };
}

DebugStrOffset!();