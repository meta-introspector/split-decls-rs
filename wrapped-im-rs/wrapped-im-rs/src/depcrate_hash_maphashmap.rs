// Generated macro for hashmap (macro)
macro_rules! Depcrate_hash_maphashmap {
() => {
// Module: crate::hash::map
// Provides: {"hashmap"}
// Dependencies: {}
# [doc = " Construct a hash map from a sequence of key/value pairs."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate im;"] # [doc = " # use im::hashmap::HashMap;"] # [doc = " # fn main() {"] # [doc = " assert_eq!("] # [doc = "   hashmap!{"] # [doc = "     1 => 11,"] # [doc = "     2 => 22,"] # [doc = "     3 => 33"] # [doc = "   },"] # [doc = "   HashMap::from(vec![(1, 11), (2, 22), (3, 33)])"] # [doc = " );"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! hashmap { () => { $ crate :: hashmap :: HashMap :: new () } ; ($ ($ key : expr => $ value : expr) ,*) => { { let mut map = $ crate :: hashmap :: HashMap :: new () ; $ ({ map . insert ($ key , $ value) ; }) *; map } } ; ($ ($ key : expr => $ value : expr ,) *) => { { let mut map = $ crate :: hashmap :: HashMap :: new () ; $ ({ map . insert ($ key , $ value) ; }) *; map } } ; }
};
}
