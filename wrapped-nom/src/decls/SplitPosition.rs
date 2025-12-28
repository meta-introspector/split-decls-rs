macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! SplitPosition {
    () => {
        deps!();
        # [doc = " Parser wrapper for `split_at_position`"] pub struct SplitPosition < F , E > { predicate : F , error : PhantomData < E > , }
    };
}

SplitPosition!();