macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Format < '_ > { # [doc = " Return true if the `name` is directly associated with `id`, i.e. there are no commits between them."] pub fn is_exact_match (& self) -> bool { self . depth == 0 } # [doc = " Set this instance to print in long mode, that is if `depth` is 0, it will still print the whole"] # [doc = " long form even though it's not quite necessary."] # [doc = ""] # [doc = " Otherwise, it is allowed to shorten itself."] pub fn long (& mut self , long : bool) -> & mut Self { self . long = long ; self } }
    };
}

impl_3!();