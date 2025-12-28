macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! dmap2 {
    () => {
        deps!();
        fn dmap2 < T , F > (a : T , b : T , f : F) -> T where T : Store < vec128_storage > + Into < vec128_storage > , F : Fn (u32 , u32) -> u32 , { let a : vec128_storage = a . into () ; let b : vec128_storage = b . into () ; let ao = unsafe { a . d } ; let bo = unsafe { b . d } ; let d = vec128_storage { d : [f (ao [0] , bo [0]) , f (ao [1] , bo [1]) , f (ao [2] , bo [2]) , f (ao [3] , bo [3]) ,] , } ; unsafe { T :: unpack (d) } }
    };
}

dmap2!();