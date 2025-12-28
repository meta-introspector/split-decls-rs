macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [doc = " Access [`IndexMap`] values at indexed positions."] # [doc = ""] # [doc = " Mutable indexing allows changing / updating indexed values"] # [doc = " that are already present."] # [doc = ""] # [doc = " You can **not** insert new values with index syntax -- use [`.insert()`][IndexMap::insert]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::IndexMap;"] # [doc = ""] # [doc = " let mut map = IndexMap::new();"] # [doc = " for word in \"Lorem ipsum dolor sit amet\".split_whitespace() {"] # [doc = "     map.insert(word.to_lowercase(), word.to_string());"] # [doc = " }"] # [doc = " let lorem = &mut map[0];"] # [doc = " assert_eq!(lorem, \"Lorem\");"] # [doc = " lorem.retain(char::is_lowercase);"] # [doc = " assert_eq!(map[\"lorem\"], \"orem\");"] # [doc = " ```"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use indexmap::IndexMap;"] # [doc = ""] # [doc = " let mut map = IndexMap::new();"] # [doc = " map.insert(\"foo\", 1);"] # [doc = " map[10] = 1; // panics!"] # [doc = " ```"] impl < K , V , S > IndexMut < usize > for IndexMap < K , V , S > { # [doc = " Returns a mutable reference to the value at the supplied `index`."] # [doc = ""] # [doc = " ***Panics*** if `index` is out of bounds."] fn index_mut (& mut self , index : usize) -> & mut V { let len : usize = self . len () ; if let Some ((_ , value)) = self . get_index_mut (index) { value } else { panic ! ("index out of bounds: the len is {len} but the index is {index}") ; } } }
    };
}

impl_59!()