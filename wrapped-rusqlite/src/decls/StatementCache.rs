macro_rules! deps {
    () => {
        RawStatement!();
    };
}

macro_rules! StatementCache {
    () => {
        deps!();
        # [doc = " Prepared statements LRU cache."] # [derive (Debug)] pub struct StatementCache (RefCell < LruCache < Arc < str > , RawStatement > >) ;
    };
}

StatementCache!()