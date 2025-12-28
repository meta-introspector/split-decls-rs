macro_rules! DebugInfoOffset {
    () => {
        # [doc = " An offset into the `.debug_info` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Ord , PartialOrd , Hash)] pub struct DebugInfoOffset < T = usize > (pub T) ;
    };
}

DebugInfoOffset!()