macro_rules! DebugMacinfoOffset {
    () => {
        # [doc = " An offset into the `.debug_macinfo` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct DebugMacinfoOffset < T = usize > (pub T) ;
    };
}

DebugMacinfoOffset!()