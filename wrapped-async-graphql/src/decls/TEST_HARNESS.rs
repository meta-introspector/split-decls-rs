macro_rules! deps {
    () => {
        Schema!();
        Query!();
        Mutation!();
        Subscription!();
    };
}

macro_rules! TEST_HARNESS {
    () => {
        deps!();
        static TEST_HARNESS : OnceLock < Schema < Query , Mutation , Subscription > > = OnceLock :: new () ;
    };
}

TEST_HARNESS!()