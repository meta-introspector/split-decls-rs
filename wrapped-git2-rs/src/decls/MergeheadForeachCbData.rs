macro_rules! deps {
    () => {
        MergeheadForeachCb!();
    };
}

macro_rules! MergeheadForeachCbData {
    () => {
        deps!();
        struct MergeheadForeachCbData < 'a > { callback : & 'a mut MergeheadForeachCb < 'a > , }
    };
}

MergeheadForeachCbData!()