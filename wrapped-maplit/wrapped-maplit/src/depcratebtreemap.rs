// Generated macro for btreemap (macro)
macro_rules! Depcratebtreemap {
() => {
// Module: crate
// Provides: {"btreemap"}
// Dependencies: {}
# [macro_export (local_inner_macros)] # [doc = " Create a **BTreeMap** from a list of key-value pairs"] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " #[macro_use] extern crate maplit;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let map = btreemap!{"] # [doc = "     \"a\" => 1,"] # [doc = "     \"b\" => 2,"] # [doc = " };"] # [doc = " assert_eq!(map[\"a\"], 1);"] # [doc = " assert_eq!(map[\"b\"], 2);"] # [doc = " assert_eq!(map.get(\"c\"), None);"] # [doc = " # }"] # [doc = " ```"] macro_rules ! btreemap { ($ ($ key : expr => $ value : expr ,) +) => (btreemap ! ($ ($ key => $ value) ,+)) ; ($ ($ key : expr => $ value : expr) ,*) => { { let mut _map = :: std :: collections :: BTreeMap :: new () ; $ (let _ = _map . insert ($ key , $ value) ;) * _map } } ; }
};
}
