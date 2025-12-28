macro_rules! deps {
    () => {
        HashMap!();
        RustcVacantEntry!();
        RustcOccupiedEntry!();
        RustcEntry!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl < K , V , S , A > HashMap < K , V , S , A > where K : Eq + Hash , S : BuildHasher , A : Allocator , { # [doc = " Gets the given key's corresponding entry in the map for in-place manipulation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use hashbrown::HashMap;"] # [doc = ""] # [doc = " let mut letters = HashMap::new();"] # [doc = ""] # [doc = " for ch in \"a short treatise on fungi\".chars() {"] # [doc = "     let counter = letters.rustc_entry(ch).or_insert(0);"] # [doc = "     *counter += 1;"] # [doc = " }"] # [doc = ""] # [doc = " assert_eq!(letters[&'s'], 2);"] # [doc = " assert_eq!(letters[&'t'], 3);"] # [doc = " assert_eq!(letters[&'u'], 1);"] # [doc = " assert_eq!(letters.get(&'y'), None);"] # [doc = " ```"] # [cfg_attr (feature = "inline-more" , inline)] pub fn rustc_entry (& mut self , key : K) -> RustcEntry < '_ , K , V , A > { let hash = make_hash (& self . hash_builder , & key) ; if let Some (elem) = self . table . find (hash , | q | q . 0 . eq (& key)) { RustcEntry :: Occupied (RustcOccupiedEntry { elem , table : & mut self . table , }) } else { self . reserve (1) ; RustcEntry :: Vacant (RustcVacantEntry { hash , key , table : & mut self . table , }) } } }
    };
}

impl_364!()