macro_rules! deps {
    () => {
        OsError!();
        ThreadRng!();
    };
}

macro_rules! impl_252 {
    () => {
        deps!();
        impl ThreadRng { # [doc = " Immediately reseed the generator"] # [doc = ""] # [doc = " This discards any remaining random data in the cache."] pub fn reseed (& mut self) -> Result < () , OsError > { let rng = unsafe { & mut * self . rng . get () } ; rng . reseed () } }
    };
}

impl_252!();