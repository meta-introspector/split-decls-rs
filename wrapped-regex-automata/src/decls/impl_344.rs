macro_rules! deps {
    () => {
        Regex!();
        FindMatches!();
        Input!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl < 'r , 'h > FindMatches < 'r , 'h > { # [doc = " Returns the `Regex` value that created this iterator."] # [inline] pub fn regex (& self) -> & 'r Regex { self . re } # [doc = " Returns the current `Input` associated with this iterator."] # [doc = ""] # [doc = " The `start` position on the given `Input` may change during iteration,"] # [doc = " but all other values are guaranteed to remain invariant."] # [inline] pub fn input < 's > (& 's self) -> & 's Input < 'h > { self . it . input () } }
    };
}

impl_344!()