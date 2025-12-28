macro_rules! DebugAddrBase {
    () => {
        # [doc = " An offset to a set of entries in the `.debug_addr` section."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct DebugAddrBase < T = usize > (pub T) ;
    };
}

DebugAddrBase!();