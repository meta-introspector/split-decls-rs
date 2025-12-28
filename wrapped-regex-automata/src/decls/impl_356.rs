macro_rules! deps {
    () => {
        SplitN!();
        Input!();
    };
}

macro_rules! impl_356 {
    () => {
        deps!();
        impl < 'r , 'h > SplitN < 'r , 'h > { # [doc = " Returns the current `Input` associated with this iterator."] # [doc = ""] # [doc = " The `start` position on the given `Input` may change during iteration,"] # [doc = " but all other values are guaranteed to remain invariant."] # [inline] pub fn input < 's > (& 's self) -> & 's Input < 'h > { self . splits . input () } }
    };
}

impl_356!();