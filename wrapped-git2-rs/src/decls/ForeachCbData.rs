macro_rules! deps {
    () => {
        ForeachCb!();
    };
}

macro_rules! ForeachCbData {
    () => {
        deps!();
        struct ForeachCbData < 'a > { pub callback : & 'a mut ForeachCb < 'a > , }
    };
}

ForeachCbData!();