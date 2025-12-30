// Generated macro for ValuesMut (struct)
macro_rules! Depcrate_mapValuesMut {
() => {
// Module: crate::map
// Provides: {"ValuesMut"}
// Dependencies: {}
# [doc = " A mutable iterator over the values of a `HashMap` in arbitrary order."] # [doc = " The iterator element type is `&'a mut V`."] # [doc = ""] # [doc = " This `struct` is created by the [`values_mut`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`values_mut`]: struct.HashMap.html#method.values_mut"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let mut map: HashMap<_, _> = [(1, \"One\".to_owned()), (2, \"Two\".into())].into();"] # [doc = ""] # [doc = " let mut values = map.values_mut();"] # [doc = " values.next().map(|v| v.push_str(\" Mississippi\"));"] # [doc = " values.next().map(|v| v.push_str(\" Mississippi\"));"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(values.next(), None);"] # [doc = " assert_eq!(values.next(), None);"] # [doc = ""] # [doc = " assert_eq!(map.get(&1).unwrap(), &\"One Mississippi\".to_owned());"] # [doc = " assert_eq!(map.get(&2).unwrap(), &\"Two Mississippi\".to_owned());"] # [doc = " ```"] pub struct ValuesMut < 'a , K , V > { inner : IterMut < 'a , K , V > , }
};
}
