macro_rules! deps {
    () => {
        StoreBulkMut!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < K , V , S > LiteMap < K , V , S > where S : StoreBulkMut < K , V > , { # [doc = " Retains only the elements specified by the predicate."] # [doc = ""] # [doc = " In other words, remove all elements such that `f((&k, &v))` returns `false`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use litemap::LiteMap;"] # [doc = ""] # [doc = " let mut map = LiteMap::new_vec();"] # [doc = " map.insert(1, \"one\");"] # [doc = " map.insert(2, \"two\");"] # [doc = " map.insert(3, \"three\");"] # [doc = ""] # [doc = " // Retain elements with odd keys"] # [doc = " map.retain(|k, _| k % 2 == 1);"] # [doc = ""] # [doc = " assert_eq!(map.get(&1), Some(&\"one\"));"] # [doc = " assert_eq!(map.get(&2), None);"] # [doc = " ```"] # [inline] pub fn retain < F > (& mut self , predicate : F) where F : FnMut (& K , & V) -> bool , { self . values . lm_retain (predicate) } }
    };
}

impl_26!();