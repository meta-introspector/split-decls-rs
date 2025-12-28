macro_rules! deps {
    () => {
        ExtensionFactory!();
        ApolloPersistedQueries!();
        ApolloPersistedQueriesExtension!();
        Extension!();
        CacheStorage!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        impl < T : CacheStorage > ExtensionFactory for ApolloPersistedQueries < T > { fn create (& self) -> Arc < dyn Extension > { Arc :: new (ApolloPersistedQueriesExtension { storage : self . 0 . clone () , }) } }
    };
}

impl_533!();