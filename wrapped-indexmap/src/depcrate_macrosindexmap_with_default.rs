// Generated macro for indexmap_with_default (macro)
macro_rules! Depcrate_macrosindexmap_with_default {
() => {
// Module: crate::macros
// Provides: {"indexmap_with_default"}
// Dependencies: {}
# [doc = " Create an [`IndexMap`][crate::IndexMap] from a list of key-value pairs"] # [doc = " and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::indexmap_with_default;"] # [doc = " use fnv::FnvHasher;"] # [doc = ""] # [doc = " let map = indexmap_with_default!{"] # [doc = "     FnvHasher;"] # [doc = "     \"a\" => 1,"] # [doc = "     \"b\" => 2,"] # [doc = " };"] # [doc = " assert_eq!(map[\"a\"], 1);"] # [doc = " assert_eq!(map[\"b\"], 2);"] # [doc = " assert_eq!(map.get(\"c\"), None);"] # [doc = ""] # [doc = " // \"a\" is the first key"] # [doc = " assert_eq!(map.keys().next(), Some(&\"a\"));"] # [doc = " ```"] # [macro_export] macro_rules ! indexmap_with_default { ($ H : ty ; $ ($ key : expr => $ value : expr ,) +) => { $ crate :: indexmap_with_default ! ($ H ; $ ($ key => $ value) ,+) } ; ($ H : ty ; $ ($ key : expr => $ value : expr) ,*) => { { let builder = :: core :: hash :: BuildHasherDefault ::<$ H >:: default () ; const CAP : usize = < [()] >:: len (& [$ ({ stringify ! ($ key) ; }) ,*]) ; # [allow (unused_mut)] let mut map = $ crate :: IndexMap :: with_capacity_and_hasher (CAP , builder) ; $ (map . insert ($ key , $ value) ;) * map } } ; }
};
}
