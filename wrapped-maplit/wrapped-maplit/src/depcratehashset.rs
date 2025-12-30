// Generated macro for hashset (macro)
macro_rules! Depcratehashset {
() => {
// Module: crate
// Provides: {"hashset"}
// Dependencies: {}
# [doc = " Create a **HashSet** from a list of elements."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " #[macro_use] extern crate maplit;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let set = hashset!{\"a\", \"b\"};"] # [doc = " assert!(set.contains(\"a\"));"] # [doc = " assert!(set.contains(\"b\"));"] # [doc = " assert!(!set.contains(\"c\"));"] # [doc = " # }"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! hashset { (@ single $ ($ x : tt) *) => (()) ; (@ count $ ($ rest : expr) ,*) => (< [()] >:: len (& [$ (hashset ! (@ single $ rest)) ,*])) ; ($ ($ key : expr ,) +) => { hashset ! ($ ($ key) ,+) } ; ($ ($ key : expr) ,*) => { { let _cap = hashset ! (@ count $ ($ key) ,*) ; let mut _set = :: std :: collections :: HashSet :: with_capacity (_cap) ; $ (let _ = _set . insert ($ key) ;) * _set } } ; }
};
}
