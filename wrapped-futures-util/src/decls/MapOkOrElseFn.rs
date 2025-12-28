macro_rules! deps {
    () => {
        MapOkFn!();
        MapErrFn!();
        MergeResultFn!();
        ChainFn!();
    };
}

macro_rules! MapOkOrElseFn {
    () => {
        deps!();
        pub (crate) type MapOkOrElseFn < F , G > = ChainFn < MapOkFn < F > , ChainFn < MapErrFn < G > , MergeResultFn > > ;
    };
}

MapOkOrElseFn!();