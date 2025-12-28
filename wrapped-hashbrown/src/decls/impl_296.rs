macro_rules! deps {
    () => {
        HashMap!();
        IntoIter!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl < K , V , S , A : Allocator > IntoIterator for HashMap < K , V , S , A > { type Item = (K , V) ; type IntoIter = IntoIter < K , V , A > ; # [doc = " Creates a consuming iterator, that is, one that moves each key-value"] # [doc = " pair out of the map in arbitrary order. The map cannot be used after"] # [doc = " calling this."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let map: HashMap<_, _> = [(\"a\", 1), (\"b\", 2), (\"c\", 3)].into();"] # [doc = ""] # [doc = " // Not possible with .iter()"] # [doc = " let mut vec: Vec<(&str, i32)> = map.into_iter().collect();"] # [doc = " // The `IntoIter` iterator produces items in arbitrary order, so"] # [doc = " // the items must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [(\"a\", 1), (\"b\", 2), (\"c\", 3)]);"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] fn into_iter (self) -> IntoIter < K , V , A > { IntoIter { inner : self . table . into_iter () , } } }
    };
}

impl_296!()