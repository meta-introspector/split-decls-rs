macro_rules! deps {
    () => {
        FlatMap!();
        AnyValueId!();
        AnyValue!();
    };
}

macro_rules! Extensions {
    () => {
        deps!();
        # [derive (Default , Clone , Debug)] pub (crate) struct Extensions { extensions : FlatMap < AnyValueId , AnyValue > , }
    };
}

Extensions!()