macro_rules! deps {
    () => {
        IntoIter!();
        HashMap!();
        Iter!();
    };
}

macro_rules! impl_294 {
    () => {
        deps!();
        impl < 'a , K , V , S , A : Allocator > IntoIterator for & 'a HashMap < K , V , S , A > { type Item = (& 'a K , & 'a V) ; type IntoIter = Iter < 'a , K , V > ; # [doc = " Creates an iterator over the entries of a `HashMap` in arbitrary order."] # [doc = " The iterator element type is `(&'a K, &'a V)`."] # [doc = ""] # [doc = " Return the same `Iter` struct as by the [`iter`] method on [`HashMap`]."] # [doc = ""] # [doc = " [`iter`]: struct.HashMap.html#method.iter"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = " let map_one: HashMap<_, _> = [(1, \"a\"), (2, \"b\"), (3, \"c\")].into();"] # [doc = " let mut map_two = HashMap::new();"] # [doc = ""] # [doc = " for (key, value) in &map_one {"] # [doc = "     println!(\"Key: {}, Value: {}\", key, value);"] # [doc = "     map_two.insert(*key, *value);"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(map_one, map_two);"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] fn into_iter (self) -> Iter < 'a , K , V > { self . iter () } }
    };
}

impl_294!()