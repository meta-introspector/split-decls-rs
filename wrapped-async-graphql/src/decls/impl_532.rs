macro_rules! deps {
    () => {
        CacheStorage!();
        ApolloPersistedQueries!();
    };
}

macro_rules! impl_532 {
    () => {
        deps!();
        impl < T : CacheStorage > ApolloPersistedQueries < T > { # [doc = " Creates an apollo persisted queries extension."] pub fn new (cache_storage : T) -> ApolloPersistedQueries < T > { Self (cache_storage) } }
    };
}

impl_532!()