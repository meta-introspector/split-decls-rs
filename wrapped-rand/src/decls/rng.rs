macro_rules! deps {
    () => {
        ThreadRng!();
    };
}

macro_rules! rng {
    () => {
        deps!();
        # [doc = " Access a fast, pre-initialized generator"] # [doc = ""] # [doc = " This is a handle to the local [`ThreadRng`]."] # [doc = ""] # [doc = " See also [`crate::rngs`] for alternatives."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use rand::prelude::*;"] # [doc = ""] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let mut numbers = [1, 2, 3, 4, 5];"] # [doc = " numbers.shuffle(&mut rand::rng());"] # [doc = " println!(\"Numbers: {numbers:?}\");"] # [doc = ""] # [doc = " // Using a local binding avoids an initialization-check on each usage:"] # [doc = " let mut rng = rand::rng();"] # [doc = ""] # [doc = " println!(\"True or false: {}\", rng.random::<bool>());"] # [doc = " println!(\"A simulated die roll: {}\", rng.random_range(1..=6));"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " # Security"] # [doc = ""] # [doc = " Refer to [`ThreadRng#Security`]."] pub fn rng () -> ThreadRng { let rng = THREAD_RNG_KEY . with (| t | t . clone ()) ; ThreadRng { rng } }
    };
}

rng!()