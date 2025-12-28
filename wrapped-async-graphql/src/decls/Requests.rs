macro_rules! deps {
    () => {
        CacheStorage!();
        ResSender!();
        Loader!();
    };
}

macro_rules! Requests {
    () => {
        deps!();
        struct Requests < K : Send + Sync + Hash + Eq + Clone + 'static , T : Loader < K > > { keys : HashSet < K > , pending : Vec < (HashSet < K > , ResSender < K , T >) > , cache_storage : Box < dyn CacheStorage < Key = K , Value = T :: Value > > , disable_cache : bool , }
    };
}

Requests!()