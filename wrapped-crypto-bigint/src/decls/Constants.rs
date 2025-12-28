macro_rules! Constants {
    () => {
        # [doc = " Trait for associating constant values with a type."] pub trait Constants : ConstZero + ConstOne { # [doc = " Maximum value this integer can express."] const MAX : Self ; }
    };
}

Constants!();