macro_rules! deps {
    () => {
        Equivalent!();
        HashMap!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < K , Q , V , S , A > Index < & Q > for HashMap < K , V , S , A > where K : Eq + Hash , Q : Hash + Equivalent < K > + ? Sized , S : BuildHasher , A : Allocator , { type Output = V ; # [doc = " Returns a reference to the value corresponding to the supplied key."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the key is not present in the `HashMap`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let map: HashMap<_, _> = [(\"a\", \"One\"), (\"b\", \"Two\")].into();"] # [doc = ""] # [doc = " assert_eq!(map[&\"a\"], \"One\");"] # [doc = " assert_eq!(map[&\"b\"], \"Two\");"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] fn index (& self , key : & Q) -> & V { self . get (key) . expect ("no entry found for key") } }
    };
}

impl_245!();