macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! LengthCount {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Parser implementation for the [length_count] combinator"] pub struct LengthCount < F , G , E > { length : F , parser : G , e : PhantomData < E > , }
    };
}

LengthCount!();