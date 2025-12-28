macro_rules! deps {
    () => {
        CacheControl!();
    };
}

macro_rules! CacheControlCalculate {
    () => {
        deps!();
        pub struct CacheControlCalculate < 'a > { pub cache_control : & 'a mut CacheControl , }
    };
}

CacheControlCalculate!()