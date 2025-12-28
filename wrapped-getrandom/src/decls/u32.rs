macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! u32 {
    () => {
        deps!();
        # [doc = " Get random `u32` from the system's preferred random number source."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # fn main() -> Result<(), getrandom::Error> {"] # [doc = " let rng_seed = getrandom::u32()?;"] # [doc = " # Ok(()) }"] # [doc = " ```"] # [inline] pub fn u32 () -> Result < u32 , Error > { backends :: inner_u32 () }
    };
}

u32!()