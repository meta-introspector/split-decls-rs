macro_rules! CacheStorage {
    () => {
        # [doc = " Cache storage for persisted queries."] # [async_trait :: async_trait] pub trait CacheStorage : Send + Sync + Clone + 'static { # [doc = " Load the query by `key`."] async fn get (& self , key : String) -> Option < ExecutableDocument > ; # [doc = " Save the query by `key`."] async fn set (& self , key : String , query : ExecutableDocument) ; }
    };
}

CacheStorage!();