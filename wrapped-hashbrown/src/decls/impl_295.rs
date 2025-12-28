macro_rules! deps {
    () => {
        IterMut!();
        HashMap!();
        Iter!();
        IntoIter!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl < 'a , K , V , S , A : Allocator > IntoIterator for & 'a mut HashMap < K , V , S , A > { type Item = (& 'a K , & 'a mut V) ; type IntoIter = IterMut < 'a , K , V > ; # [doc = " Creates an iterator over the entries of a `HashMap` in arbitrary order"] # [doc = " with mutable references to the values. The iterator element type is"] # [doc = " `(&'a K, &'a mut V)`."] # [doc = ""] # [doc = " Return the same `IterMut` struct as by the [`iter_mut`] method on"] # [doc = " [`HashMap`]."] # [doc = ""] # [doc = " [`iter_mut`]: struct.HashMap.html#method.iter_mut"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = " let mut map: HashMap<_, _> = [(\"a\", 1), (\"b\", 2), (\"c\", 3)].into();"] # [doc = ""] # [doc = " for (key, value) in &mut map {"] # [doc = "     println!(\"Key: {}, Value: {}\", key, value);"] # [doc = "     *value *= 2;"] # [doc = " }"] # [doc = ""] # [doc = " let mut vec = map.iter().collect::<Vec<_>>();"] # [doc = " // The `Iter` iterator produces items in arbitrary order, so the"] # [doc = " // items must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [(&\"a\", &2), (&\"b\", &4), (&\"c\", &6)]);"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] fn into_iter (self) -> IterMut < 'a , K , V > { self . iter_mut () } }
    };
}

impl_295!()