macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! dmap {
    () => {
        deps!();
        # [inline (always)] fn dmap < T , F > (t : T , f : F) -> T where T : Store < vec128_storage > + Into < vec128_storage > , F : Fn (u32) -> u32 , { let t : vec128_storage = t . into () ; let d = unsafe { t . d } ; let d = vec128_storage { d : [f (d [0]) , f (d [1]) , f (d [2]) , f (d [3])] , } ; unsafe { T :: unpack (d) } }
    };
}

dmap!()