macro_rules! DebugAddrIndex {
    () => {
        # [doc = " An index into a set of addresses in the `.debug_addr` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugAddrIndex < T = usize > (pub T) ;
    };
}

DebugAddrIndex!()