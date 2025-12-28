macro_rules! deps {
    () => {
        Input!();
        Split!();
    };
}

macro_rules! impl_352 {
    () => {
        deps!();
        impl < 'r , 'h > Split < 'r , 'h > { # [doc = " Returns the current `Input` associated with this iterator."] # [doc = ""] # [doc = " The `start` position on the given `Input` may change during iteration,"] # [doc = " but all other values are guaranteed to remain invariant."] # [inline] pub fn input < 's > (& 's self) -> & 's Input < 'h > { self . finder . input () } }
    };
}

impl_352!();