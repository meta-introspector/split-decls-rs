macro_rules! DebugArangesOffset {
    () => {
        # [doc = " An offset into the `.debug_aranges` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugArangesOffset < T = usize > (pub T) ;
    };
}

DebugArangesOffset!();