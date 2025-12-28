macro_rules! deps {
    () => {
        HashMap!();
        IntoIter!();
    };
}

macro_rules! IntoValues {
    () => {
        deps!();
        # [doc = " An owning iterator over the values of a `HashMap` in arbitrary order."] # [doc = " The iterator element type is `V`."] # [doc = ""] # [doc = " This `struct` is created by the [`into_values`] method on [`HashMap`]."] # [doc = " See its documentation for more. The map cannot be used after calling that method."] # [doc = ""] # [doc = " [`into_values`]: struct.HashMap.html#method.into_values"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let map: HashMap<_, _> = [(1, \"a\"), (2, \"b\"), (3, \"c\")].into();"] # [doc = ""] # [doc = " let mut values = map.into_values();"] # [doc = " let mut vec = vec![values.next(), values.next(), values.next()];"] # [doc = ""] # [doc = " // The `IntoValues` iterator produces values in arbitrary order, so"] # [doc = " // the values must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [Some(\"a\"), Some(\"b\"), Some(\"c\")]);"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(values.next(), None);"] # [doc = " assert_eq!(values.next(), None);"] # [doc = " ```"] pub struct IntoValues < K , V , A : Allocator = Global > { inner : IntoIter < K , V , A > , }
    };
}

IntoValues!()