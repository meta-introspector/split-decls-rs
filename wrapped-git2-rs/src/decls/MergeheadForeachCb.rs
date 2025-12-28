macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! MergeheadForeachCb {
    () => {
        deps!();
        type MergeheadForeachCb < 'a > = dyn FnMut (& Oid) -> bool + 'a ;
    };
}

MergeheadForeachCb!()