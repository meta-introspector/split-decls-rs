macro_rules! HeaderSlice {
    () => {
        # [derive (Debug , Eq , PartialEq , Hash , PartialOrd)] # [repr (C)] pub (crate) struct HeaderSlice < H , T : ? Sized > { pub (crate) header : H , length : usize , slice : T , }
    };
}

HeaderSlice!();