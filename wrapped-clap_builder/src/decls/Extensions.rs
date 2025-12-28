macro_rules! deps {
    () => {
        AnyValueId!();
        AnyValue!();
        FlatMap!();
    };
}

macro_rules! Extensions {
    () => {
        deps!();
        # [derive (Default , Clone , Debug)] pub (crate) struct Extensions { extensions : FlatMap < AnyValueId , AnyValue > , }
    };
}

Extensions!();