macro_rules! deps {
    () => {
        Reader!();
        DebugAddr!();
        RawRngListIter!();
        DebugAddrBase!();
    };
}

macro_rules! RngListIter {
    () => {
        deps!();
        # [doc = " An iterator over an address range list."] # [doc = ""] # [doc = " This iterator internally handles processing of base addresses and different"] # [doc = " entry types.  Thus, it only returns range entries that are valid"] # [doc = " and already adjusted for the base address."] # [derive (Debug)] pub struct RngListIter < R : Reader > { raw : RawRngListIter < R > , base_address : u64 , debug_addr : DebugAddr < R > , debug_addr_base : DebugAddrBase < R :: Offset > , }
    };
}

RngListIter!()