macro_rules! deps {
    () => {
        UpdateTest!();
        RunnableKind!();
    };
}

macro_rules! macro_372 {
    () => {
        deps!();
        impl_empty_upmap_from_ra_fixture ! (RunnableKind , UpdateTest) ;
    };
}

macro_372!();