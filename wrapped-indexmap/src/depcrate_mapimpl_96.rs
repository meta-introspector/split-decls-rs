// Generated macro for impl_96 (impl)
macro_rules! Depcrate_mapimpl_96 {
() => {
// Module: crate::map
// Provides: {"impl_96"}
// Dependencies: {}
# [doc = " Access [`IndexMap`] values at indexed positions."] # [doc = ""] # [doc = " See [`Index<usize> for Keys`][keys] to access a map's keys instead."] # [doc = ""] # [doc = " [keys]: Keys#impl-Index<usize>-for-Keys<'a,+K,+V>"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::IndexMap;"] # [doc = ""] # [doc = " let mut map = IndexMap::new();"] # [doc = " for word in \"Lorem ipsum dolor sit amet\".split_whitespace() {"] # [doc = "     map.insert(word.to_lowercase(), word.to_uppercase());"] # [doc = " }"] # [doc = " assert_eq!(map[0], \"LOREM\");"] # [doc = " assert_eq!(map[1], \"IPSUM\");"] # [doc = " map.reverse();"] # [doc = " assert_eq!(map[0], \"AMET\");"] # [doc = " assert_eq!(map[1], \"SIT\");"] # [doc = " map.sort_keys();"] # [doc = " assert_eq!(map[0], \"AMET\");"] # [doc = " assert_eq!(map[1], \"DOLOR\");"] # [doc = " ```"] # [doc = ""] # [doc = " ```should_panic"] # [doc = " use indexmap::IndexMap;"] # [doc = ""] # [doc = " let mut map = IndexMap::new();"] # [doc = " map.insert(\"foo\", 1);"] # [doc = " println!(\"{:?}\", map[10]); // panics!"] # [doc = " ```"] impl < K , V , S > Index < usize > for IndexMap < K , V , S > { type Output = V ; # [doc = " Returns a reference to the value at the supplied `index`."] # [doc = ""] # [doc = " ***Panics*** if `index` is out of bounds."] fn index (& self , index : usize) -> & V { if let Some ((_ , value)) = self . get_index (index) { value } else { panic ! ("index out of bounds: the len is {len} but the index is {index}" , len = self . len ()) ; } } }
};
}
