macro_rules! deps {
    () => {
        RunnableKind!();
        UpdateTest!();
    };
}

macro_rules! macro_372 {
    () => {
        deps!();
        impl_empty_upmap_from_ra_fixture ! (RunnableKind , UpdateTest) ;
    };
}

macro_372!()