macro_rules! deps {
    () => {
        Lock!();
    };
}

macro_rules! TryLock {
    () => {
        deps!();
        # [doc = " Sentinel representing an acquired lock through which the data can be"] # [doc = " accessed."] pub (crate) struct TryLock < 'a , T > { __ptr : & 'a Lock < T > , }
    };
}

TryLock!()