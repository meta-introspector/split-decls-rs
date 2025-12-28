macro_rules! deps {
    () => {
        LocListsFormat!();
        Reader!();
        Encoding!();
    };
}

macro_rules! RawLocListIter {
    () => {
        deps!();
        # [doc = " A raw iterator over a location list."] # [doc = ""] # [doc = " This iterator does not perform any processing of the location entries,"] # [doc = " such as handling base addresses."] # [derive (Debug)] pub struct RawLocListIter < R : Reader > { input : R , encoding : Encoding , format : LocListsFormat , }
    };
}

RawLocListIter!();