// Generated macro for IterMut (struct)
macro_rules! Depcrate_mapIterMut {
() => {
// Module: crate::map
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the entries of a `HashMap` in arbitrary order."] # [doc = " The iterator element type is `(&'a K, &'a mut V)`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter_mut`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter_mut`]: struct.HashMap.html#method.iter_mut"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let mut map: HashMap<_, _> = [(1, \"One\".to_owned()), (2, \"Two\".into())].into();"] # [doc = ""] # [doc = " let mut iter = map.iter_mut();"] # [doc = " iter.next().map(|(_, v)| v.push_str(\" Mississippi\"));"] # [doc = " iter.next().map(|(_, v)| v.push_str(\" Mississippi\"));"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = " assert_eq!(iter.next(), None);"] # [doc = ""] # [doc = " assert_eq!(map.get(&1).unwrap(), &\"One Mississippi\".to_owned());"] # [doc = " assert_eq!(map.get(&2).unwrap(), &\"Two Mississippi\".to_owned());"] # [doc = " ```"] pub struct IterMut < 'a , K , V > { inner : RawIter < (K , V) > , marker : PhantomData < (& 'a K , & 'a mut V) > , }
};
}
