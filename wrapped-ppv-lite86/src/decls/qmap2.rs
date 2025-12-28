macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! qmap2 {
    () => {
        deps!();
        # [inline (always)] fn qmap2 < T , F > (a : T , b : T , f : F) -> T where T : Store < vec128_storage > + Into < vec128_storage > , F : Fn (u64 , u64) -> u64 , { let a : vec128_storage = a . into () ; let b : vec128_storage = b . into () ; let ao = unsafe { a . q } ; let bo = unsafe { b . q } ; let q = vec128_storage { q : [f (ao [0] , bo [0]) , f (ao [1] , bo [1])] , } ; unsafe { T :: unpack (q) } }
    };
}

qmap2!();