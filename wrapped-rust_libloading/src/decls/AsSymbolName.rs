macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! AsSymbolName {
    () => {
        deps!();
        # [doc = " This trait is implemented for types [`Library`](crate::Library) implementations can use to look"] # [doc = " up symbols."] # [doc = ""] # [doc = " It is currently sealed and cannot be implemented or its methods called by users of this crate."] # [expect (private_bounds)] pub trait AsSymbolName : Sealed { }
    };
}

AsSymbolName!();