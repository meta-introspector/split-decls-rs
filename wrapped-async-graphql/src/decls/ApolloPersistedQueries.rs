macro_rules! ApolloPersistedQueries {
    () => {
        # [doc = " Apollo persisted queries extension."] # [doc = ""] # [doc = " [Reference](https://www.apollographql.com/docs/react/api/link/persisted-queries/)"] # [cfg_attr (docsrs , doc (cfg (feature = "apollo_persisted_queries")))] pub struct ApolloPersistedQueries < T > (T) ;
    };
}

ApolloPersistedQueries!()