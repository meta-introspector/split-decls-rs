macro_rules! deps {
    () => {
        Store!();
    };
}

macro_rules! omap {
    () => {
        deps!();
        # [inline (always)] fn omap < T , F > (a : T , f : F) -> T where T : Store < vec128_storage > + Into < vec128_storage > , F : Fn (u128) -> u128 , { let a : vec128_storage = a . into () ; let ao = o_of_q (unsafe { a . q }) ; let o = vec128_storage { q : q_of_o (f (ao)) } ; unsafe { T :: unpack (o) } }
    };
}

omap!();