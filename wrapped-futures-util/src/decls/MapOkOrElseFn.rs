macro_rules! deps {
    () => {
        MapOkFn!();
        MergeResultFn!();
        MapErrFn!();
        ChainFn!();
    };
}

macro_rules! MapOkOrElseFn {
    () => {
        deps!();
        pub (crate) type MapOkOrElseFn < F , G > = ChainFn < MapOkFn < F > , ChainFn < MapErrFn < G > , MergeResultFn > > ;
    };
}

MapOkOrElseFn!()