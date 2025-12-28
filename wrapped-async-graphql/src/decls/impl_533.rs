macro_rules! deps {
    () => {
        CacheStorage!();
        Extension!();
        ApolloPersistedQueries!();
        ApolloPersistedQueriesExtension!();
        ExtensionFactory!();
    };
}

macro_rules! impl_533 {
    () => {
        deps!();
        impl < T : CacheStorage > ExtensionFactory for ApolloPersistedQueries < T > { fn create (& self) -> Arc < dyn Extension > { Arc :: new (ApolloPersistedQueriesExtension { storage : self . 0 . clone () , }) } }
    };
}

impl_533!()