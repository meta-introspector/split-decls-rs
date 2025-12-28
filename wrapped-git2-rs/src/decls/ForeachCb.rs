macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! ForeachCb {
    () => {
        deps!();
        pub type ForeachCb < 'a > = dyn FnMut (& Oid) -> bool + 'a ;
    };
}

ForeachCb!()