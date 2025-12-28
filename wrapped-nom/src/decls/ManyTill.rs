macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! ManyTill {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [doc = " Parser implementation for the [many_till] combinator"] pub struct ManyTill < F , G , E > { f : F , g : G , e : PhantomData < E > , }
    };
}

ManyTill!()