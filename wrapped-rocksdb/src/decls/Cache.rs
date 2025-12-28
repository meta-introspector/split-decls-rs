macro_rules! deps {
    () => {
        CacheWrapper!();
    };
}

macro_rules! Cache {
    () => {
        deps!();
        # [derive (Clone)] pub struct Cache (pub (crate) Arc < CacheWrapper >) ;
    };
}

Cache!()