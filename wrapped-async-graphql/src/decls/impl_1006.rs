macro_rules! deps {
    () => {
        CacheControl!();
    };
}

macro_rules! impl_1006 {
    () => {
        deps!();
        impl CacheControl { # [must_use] pub (crate) fn merge (self , other : & CacheControl) -> CacheControl { CacheControl { public : self . public && other . public , max_age : match (self . max_age , other . max_age) { (- 1 , _) => - 1 , (_ , - 1) => - 1 , (a , 0) => a , (0 , b) => b , (a , b) => a . min (b) , } , } } }
    };
}

impl_1006!()