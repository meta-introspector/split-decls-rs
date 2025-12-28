macro_rules! deps {
    () => {
        HashMap!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl < K , V , S , A > Default for HashMap < K , V , S , A > where S : Default , A : Default + Allocator , { # [doc = " Creates an empty `HashMap<K, V, S, A>`, with the `Default` value for the hasher and allocator."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = " use std::collections::hash_map::RandomState;"] # [doc = ""] # [doc = " // You can specify all types of HashMap, including hasher and allocator."] # [doc = " // Created map is empty and don't allocate memory"] # [doc = " let map: HashMap<u32, String> = Default::default();"] # [doc = " assert_eq!(map.capacity(), 0);"] # [doc = " let map: HashMap<u32, String, RandomState> = HashMap::default();"] # [doc = " assert_eq!(map.capacity(), 0);"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { Self :: with_hasher_in (Default :: default () , Default :: default ()) } }
    };
}

impl_244!();