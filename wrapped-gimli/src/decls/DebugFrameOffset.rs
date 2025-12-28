macro_rules! DebugFrameOffset {
    () => {
        # [doc = " An offset into the `.debug_frame` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct DebugFrameOffset < T = usize > (pub T) ;
    };
}

DebugFrameOffset!();