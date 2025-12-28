macro_rules! deps {
    () => {
        LazyResult!();
        Function!();
    };
}

macro_rules! LazyFunction {
    () => {
        deps!();
        pub (crate) struct LazyFunction < R : gimli :: Reader > { dw_die_offset : gimli :: UnitOffset < R :: Offset > , lazy : LazyResult < Function < R > > , }
    };
}

LazyFunction!();