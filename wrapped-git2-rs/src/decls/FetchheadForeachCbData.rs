macro_rules! deps {
    () => {
        FetchheadForeachCb!();
    };
}

macro_rules! FetchheadForeachCbData {
    () => {
        deps!();
        struct FetchheadForeachCbData < 'a > { callback : & 'a mut FetchheadForeachCb < 'a > , }
    };
}

FetchheadForeachCbData!()