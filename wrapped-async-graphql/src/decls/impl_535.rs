macro_rules! deps {
    () => {
        ServerError!();
        ExtensionContext!();
        Extension!();
        PersistedQuery!();
        CacheStorage!();
        NextPrepareRequest!();
        ServerResult!();
        ApolloPersistedQueriesExtension!();
        Request!();
    };
}

macro_rules! impl_535 {
    () => {
        deps!();
        # [async_trait :: async_trait] impl < T : CacheStorage > Extension for ApolloPersistedQueriesExtension < T > { async fn prepare_request (& self , ctx : & ExtensionContext < '_ > , mut request : Request , next : NextPrepareRequest < '_ > ,) -> ServerResult < Request > { let res = if let Some (value) = request . extensions . remove ("persistedQuery") { let persisted_query : PersistedQuery = from_value (value) . map_err (| _ | { ServerError :: new ("Invalid \"PersistedQuery\" extension configuration." , None) }) ? ; if persisted_query . version != 1 { return Err (ServerError :: new (format ! ("Only the \"PersistedQuery\" extension of version \"1\" is supported, and the current version is \"{}\"." , persisted_query . version) , None ,)) ; } if request . query . is_empty () { if let Some (doc) = self . storage . get (persisted_query . sha256_hash) . await { Ok (Request { parsed_query : Some (doc) , .. request }) } else { Err (ServerError :: new ("PersistedQueryNotFound" , None)) } } else { let sha256_hash = format ! ("{:x}" , Sha256 :: digest (request . query . as_bytes ())) ; if persisted_query . sha256_hash != sha256_hash { Err (ServerError :: new ("provided sha does not match query" , None)) } else { let doc = async_graphql_parser :: parse_query (& request . query) ? ; self . storage . set (sha256_hash , doc . clone ()) . await ; Ok (Request { query : String :: new () , parsed_query : Some (doc) , .. request }) } } } else { Ok (request) } ; next . run (ctx , res ?) . await } }
    };
}

impl_535!()