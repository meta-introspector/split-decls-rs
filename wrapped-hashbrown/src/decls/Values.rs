macro_rules! deps {
    () => {
        HashMap!();
        Iter!();
    };
}

macro_rules! Values {
    () => {
        deps!();
        # [doc = " An iterator over the values of a `HashMap` in arbitrary order."] # [doc = " The iterator element type is `&'a V`."] # [doc = ""] # [doc = " This `struct` is created by the [`values`] method on [`HashMap`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`values`]: struct.HashMap.html#method.values"] # [doc = " [`HashMap`]: struct.HashMap.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let map: HashMap<_, _> = [(1, \"a\"), (2, \"b\"), (3, \"c\")].into();"] # [doc = ""] # [doc = " let mut values = map.values();"] # [doc = " let mut vec = vec![values.next(), values.next(), values.next()];"] # [doc = ""] # [doc = " // The `Values` iterator produces values in arbitrary order, so the"] # [doc = " // values must be sorted to test them against a sorted array."] # [doc = " vec.sort_unstable();"] # [doc = " assert_eq!(vec, [Some(&\"a\"), Some(&\"b\"), Some(&\"c\")]);"] # [doc = ""] # [doc = " // It is fused iterator"] # [doc = " assert_eq!(values.next(), None);"] # [doc = " assert_eq!(values.next(), None);"] # [doc = " ```"] pub struct Values < 'a , K , V > { inner : Iter < 'a , K , V > , }
    };
}

Values!()