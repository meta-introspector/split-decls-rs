macro_rules! IdMap {
    () => {
        # [doc = " A mapping between an object id and arbitrary data, and produced when calling [`Graph::detach()`]."] pub type IdMap < T > = gix_hashtable :: HashMap < gix_hash :: ObjectId , T > ;
    };
}

IdMap!();