macro_rules! ApolloPersistedQueriesExtension {
    () => {
        struct ApolloPersistedQueriesExtension < T > { storage : T , }
    };
}

ApolloPersistedQueriesExtension!();