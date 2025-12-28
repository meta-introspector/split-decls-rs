macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! inner_u32 {
    () => {
        deps!();
        # [doc = " Default implementation of `inner_u32` on top of `fill_uninit`"] # [inline] pub fn inner_u32 () -> Result < u32 , Error > { let mut res = MaybeUninit :: < u32 > :: uninit () ; let dst = unsafe { let p : * mut MaybeUninit < u8 > = res . as_mut_ptr () . cast () ; slice :: from_raw_parts_mut (p , core :: mem :: size_of :: < u32 > ()) } ; crate :: fill_uninit (dst) ? ; Ok (unsafe { res . assume_init () }) }
    };
}

inner_u32!();