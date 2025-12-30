// Generated macro for update_in (macro)
macro_rules! Depcrateupdate_in {
() => {
// Module: crate
// Provides: {"update_in"}
// Dependencies: {}
# [doc = " Update a value inside multiple levels of data structures."] # [doc = ""] # [doc = " This macro takes a [`Vector`][Vector], [`OrdMap`][OrdMap] or [`HashMap`][HashMap],"] # [doc = " a key or a series of keys, and a value, and returns the data structure with the"] # [doc = " new value at the location described by the keys."] # [doc = ""] # [doc = " If one of the keys in the path doesn't exist, the macro will panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[macro_use] extern crate im;"] # [doc = " # use std::sync::Arc;"] # [doc = " # fn main() {"] # [doc = " let vec_inside_vec = vector![vector![1, 2, 3], vector![4, 5, 6]];"] # [doc = ""] # [doc = " let expected = vector![vector![1, 2, 3], vector![4, 5, 1337]];"] # [doc = ""] # [doc = " assert_eq!(expected, update_in![vec_inside_vec, 1 => 2, 1337]);"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " [Vector]: ../vector/enum.Vector.html"] # [doc = " [HashMap]: ../hashmap/struct.HashMap.html"] # [doc = " [OrdMap]: ../ordmap/struct.OrdMap.html"] # [macro_export] macro_rules ! update_in { ($ target : expr , $ path : expr => $ ($ tail : tt) => *, $ value : expr) => { { let inner = $ target . get ($ path) . expect ("update_in! macro: key not found in target") ; $ target . update ($ path , update_in ! (inner , $ ($ tail) => *, $ value)) } } ; ($ target : expr , $ path : expr , $ value : expr) => { $ target . update ($ path , $ value) } ; }
};
}
