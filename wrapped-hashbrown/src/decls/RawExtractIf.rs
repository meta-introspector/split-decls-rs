macro_rules! deps {
    () => {
        RawIter!();
        RawTable!();
    };
}

macro_rules! RawExtractIf {
    () => {
        deps!();
        pub (crate) struct RawExtractIf < 'a , T , A : Allocator > { pub iter : RawIter < T > , pub table : & 'a mut RawTable < T , A > , }
    };
}

RawExtractIf!();