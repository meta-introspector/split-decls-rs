macro_rules! deps {
    () => {
        Subscription!();
        Query!();
        Mutation!();
        Schema!();
    };
}

macro_rules! TEST_HARNESS {
    () => {
        deps!();
        static TEST_HARNESS : OnceLock < Schema < Query , Mutation , Subscription > > = OnceLock :: new () ;
    };
}

TEST_HARNESS!();