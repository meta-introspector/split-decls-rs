macro_rules! WeakRefCount {
    () => {
        # [repr (transparent)] # [derive (Default)] pub struct WeakRefCount (AtomicIsize) ;
    };
}

WeakRefCount!()