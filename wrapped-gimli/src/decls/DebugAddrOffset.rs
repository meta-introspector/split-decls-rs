macro_rules! DebugAddrOffset {
    () => {
        # [doc = " An offset into the `.debug_addr` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugAddrOffset < T = usize > (pub T) ;
    };
}

DebugAddrOffset!()