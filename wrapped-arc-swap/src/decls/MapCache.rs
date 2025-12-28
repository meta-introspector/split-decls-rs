macro_rules! deps {
    () => {
        Cache!();
        Access!();
    };
}

macro_rules! MapCache {
    () => {
        deps!();
        # [doc = " An implementation of a cache with a projection into the accessed value."] # [doc = ""] # [doc = " This is the implementation structure for [`Cache::map`]. It can't be created directly and it"] # [doc = " should be used through the [`Access`] trait."] # [derive (Clone , Debug)] pub struct MapCache < A , T , F > { inner : Cache < A , T > , projection : F , }
    };
}

MapCache!()