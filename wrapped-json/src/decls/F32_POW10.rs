macro_rules! F32_POW10 {
    () => {
        # [doc = " Precalculated values of radix**i for i in range [0, arr.len()-1]."] # [doc = " Each value can be **exactly** represented as that type."] const F32_POW10 : [f32 ; 11] = [1.0 , 10.0 , 100.0 , 1000.0 , 10000.0 , 100000.0 , 1000000.0 , 10000000.0 , 100000000.0 , 1000000000.0 , 10000000000.0 ,] ;
    };
}

F32_POW10!();