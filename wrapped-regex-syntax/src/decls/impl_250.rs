macro_rules! deps {
    () => {
        Hir!();
        Repetition!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl Repetition { # [doc = " Returns a new repetition with the same `min`, `max` and `greedy`"] # [doc = " values, but with its sub-expression replaced with the one given."] pub fn with (& self , sub : Hir) -> Repetition { Repetition { min : self . min , max : self . max , greedy : self . greedy , sub : Box :: new (sub) , } } }
    };
}

impl_250!();