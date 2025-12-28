macro_rules! deps {
    () => {
        RandomSource!();
        DefaultRandomSource!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl RandomSource for DefaultRandomSource { cfg_if :: cfg_if ! { if # [cfg (all (target_arch = "arm" , target_os = "none"))] { fn gen_hasher_seed (& self) -> usize { let stack = self as * const _ as usize ; let previous = self . counter . load (Ordering :: Relaxed) ; let new = previous . wrapping_add (stack) ; self . counter . store (new , Ordering :: Relaxed) ; new } } else { fn gen_hasher_seed (& self) -> usize { let stack = self as * const _ as usize ; self . counter . fetch_add (stack , Ordering :: Relaxed) } } } }
    };
}

impl_78!();