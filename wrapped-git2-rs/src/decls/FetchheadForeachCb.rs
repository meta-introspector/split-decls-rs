macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! FetchheadForeachCb {
    () => {
        deps!();
        type FetchheadForeachCb < 'a > = dyn FnMut (& str , & [u8] , & Oid , bool) -> bool + 'a ;
    };
}

FetchheadForeachCb!()