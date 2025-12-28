macro_rules! deps {
    () => {
        DefaultHashBuilder!();
        LinkedHashMap!();
    };
}

macro_rules! LruCache {
    () => {
        deps!();
        pub struct LruCache < K , V , S = DefaultHashBuilder > { map : LinkedHashMap < K , V , S > , max_size : usize , }
    };
}

LruCache!();