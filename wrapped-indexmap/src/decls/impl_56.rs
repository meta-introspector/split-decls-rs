macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [doc = " Access [`IndexMap`] values corresponding to a key."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::IndexMap;"] # [doc = ""] # [doc = " let mut map = IndexMap::new();"] # [doc = " for word in \"Lorem ipsum dolor sit amet\".split_whitespace() {"] # [doc = "     map.insert(word.to_lowercase(), word.to_uppercase());"] # [doc = " }"] # [doc = " assert_eq!(map[\"lorem\"], \"LOREM\");"] # [doc = " assert_eq!(map[\"ipsum\"], \"IPSUM\");"] # [doc = " ```"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use indexmap::IndexMap;"] # [doc = ""] # [doc = " let mut map = IndexMap::new();"] # [doc = " map.insert(\"foo\", 1);"] # [doc = " println!(\"{:?}\", map[\"bar\"]); // panics!"] # [doc = " ```"] impl < K , V , Q : ? Sized , S > Index < & Q > for IndexMap < K , V , S > where Q : Hash + Equivalent < K > , S : BuildHasher , { type Output = V ; # [doc = " Returns a reference to the value corresponding to the supplied `key`."] # [doc = ""] # [doc = " ***Panics*** if `key` is not present in the map."] fn index (& self , key : & Q) -> & V { self . get (key) . expect ("no entry found for key") } }
    };
}

impl_56!();