macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! MultiSpace0 {
    () => {
        deps!();
        # [doc = " Parser implementation for [multispace0()]"] pub struct MultiSpace0 < E > { e : PhantomData < E > , }
    };
}

MultiSpace0!()