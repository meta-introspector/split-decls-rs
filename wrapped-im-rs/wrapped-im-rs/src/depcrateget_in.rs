// Generated macro for get_in (macro)
macro_rules! Depcrateget_in {
() => {
// Module: crate
// Provides: {"get_in"}
// Dependencies: {}
# [doc = " Get a value inside multiple levels of data structures."] # [doc = ""] # [doc = " This macro takes a [`Vector`][Vector], [`OrdMap`][OrdMap] or [`HashMap`][HashMap],"] # [doc = " along with a key or a series of keys, and returns the value at the location inside"] # [doc = " the data structure described by the key sequence, or `None` if any of the keys didn't"] # [doc = " exist."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate im;"] # [doc = " # use std::sync::Arc;"] # [doc = " # fn main() {"] # [doc = " let vec_inside_vec = vector![vector![1, 2, 3], vector![4, 5, 6]];"] # [doc = ""] # [doc = " assert_eq!(Some(&6), get_in![vec_inside_vec, 1 => 2]);"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " [Vector]: ../vector/enum.Vector.html"] # [doc = " [HashMap]: ../hashmap/struct.HashMap.html"] # [doc = " [OrdMap]: ../ordmap/struct.OrdMap.html"] # [macro_export] macro_rules ! get_in { ($ target : expr , $ path : expr => $ ($ tail : tt) => *) => { { $ target . get ($ path) . and_then (| v | get_in ! (v , $ ($ tail) => *)) } } ; ($ target : expr , $ path : expr) => { $ target . get ($ path) } ; }
};
}
