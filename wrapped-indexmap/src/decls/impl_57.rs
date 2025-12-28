macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [doc = " Access [`IndexMap`] values corresponding to a key."] # [doc = ""] # [doc = " Mutable indexing allows changing / updating values of key-value"] # [doc = " pairs that are already present."] # [doc = ""] # [doc = " You can **not** insert new pairs with index syntax, use `.insert()`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::IndexMap;"] # [doc = ""] # [doc = " let mut map = IndexMap::new();"] # [doc = " for word in \"Lorem ipsum dolor sit amet\".split_whitespace() {"] # [doc = "     map.insert(word.to_lowercase(), word.to_string());"] # [doc = " }"] # [doc = " let lorem = &mut map[\"lorem\"];"] # [doc = " assert_eq!(lorem, \"Lorem\");"] # [doc = " lorem.retain(char::is_lowercase);"] # [doc = " assert_eq!(map[\"lorem\"], \"orem\");"] # [doc = " ```"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use indexmap::IndexMap;"] # [doc = ""] # [doc = " let mut map = IndexMap::new();"] # [doc = " map.insert(\"foo\", 1);"] # [doc = " map[\"bar\"] = 1; // panics!"] # [doc = " ```"] impl < K , V , Q : ? Sized , S > IndexMut < & Q > for IndexMap < K , V , S > where Q : Hash + Equivalent < K > , S : BuildHasher , { # [doc = " Returns a mutable reference to the value corresponding to the supplied `key`."] # [doc = ""] # [doc = " ***Panics*** if `key` is not present in the map."] fn index_mut (& mut self , key : & Q) -> & mut V { self . get_mut (key) . expect ("no entry found for key") } }
    };
}

impl_57!();