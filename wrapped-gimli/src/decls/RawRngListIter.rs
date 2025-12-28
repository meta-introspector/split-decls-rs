macro_rules! deps {
    () => {
        Reader!();
        RangeListsFormat!();
        Encoding!();
    };
}

macro_rules! RawRngListIter {
    () => {
        deps!();
        # [doc = " A raw iterator over an address range list."] # [doc = ""] # [doc = " This iterator does not perform any processing of the range entries,"] # [doc = " such as handling base addresses."] # [derive (Debug)] pub struct RawRngListIter < R : Reader > { input : R , encoding : Encoding , format : RangeListsFormat , }
    };
}

RawRngListIter!()