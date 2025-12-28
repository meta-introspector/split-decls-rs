macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! ApolloTracingExtension {
    () => {
        deps!();
        struct ApolloTracingExtension { inner : Mutex < Inner > , }
    };
}

ApolloTracingExtension!()