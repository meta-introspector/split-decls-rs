macro_rules! deps {
    () => {
        RandomState!();
    };
}

macro_rules! HashMap {
    () => {
        deps!();
        # [doc = " Type alias for [`std::collections::HashMap<K, V, foldhash::fast::RandomState>`]."] pub type HashMap < K , V > = std :: collections :: HashMap < K , V , RandomState > ;
    };
}

HashMap!()