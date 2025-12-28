macro_rules! deps {
    () => {
        Parser!();
        ErrorKind!();
    };
}

macro_rules! SplitPosition1 {
    () => {
        deps!();
        # [doc = " Parser wrapper for `split_at_position1`"] pub struct SplitPosition1 < F , E > { e : ErrorKind , predicate : F , error : PhantomData < E > , }
    };
}

SplitPosition1!();