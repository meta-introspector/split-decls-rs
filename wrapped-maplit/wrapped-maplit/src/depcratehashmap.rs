// Generated macro for hashmap (macro)
macro_rules! Depcratehashmap {
() => {
// Module: crate
// Provides: {"hashmap"}
// Dependencies: {}
# [macro_export (local_inner_macros)] # [doc = " Create a **HashMap** from a list of key-value pairs"] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " #[macro_use] extern crate maplit;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let map = hashmap!{"] # [doc = "     \"a\" => 1,"] # [doc = "     \"b\" => 2,"] # [doc = " };"] # [doc = " assert_eq!(map[\"a\"], 1);"] # [doc = " assert_eq!(map[\"b\"], 2);"] # [doc = " assert_eq!(map.get(\"c\"), None);"] # [doc = " # }"] # [doc = " ```"] macro_rules ! hashmap { (@ single $ ($ x : tt) *) => (()) ; (@ count $ ($ rest : expr) ,*) => (< [()] >:: len (& [$ (hashmap ! (@ single $ rest)) ,*])) ; ($ ($ key : expr => $ value : expr ,) +) => { hashmap ! ($ ($ key => $ value) ,+) } ; ($ ($ key : expr => $ value : expr) ,*) => { { let _cap = hashmap ! (@ count $ ($ key) ,*) ; let mut _map = :: std :: collections :: HashMap :: with_capacity (_cap) ; $ (let _ = _map . insert ($ key , $ value) ;) * _map } } ; }
};
}
