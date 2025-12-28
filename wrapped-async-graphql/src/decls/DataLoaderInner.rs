macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! DataLoaderInner {
    () => {
        deps!();
        struct DataLoaderInner < T > { requests : Mutex < FnvHashMap < TypeId , Box < dyn Any + Sync + Send > > > , loader : T , }
    };
}

DataLoaderInner!();