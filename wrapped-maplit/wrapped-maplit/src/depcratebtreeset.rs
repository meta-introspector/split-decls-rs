// Generated macro for btreeset (macro)
macro_rules! Depcratebtreeset {
() => {
// Module: crate
// Provides: {"btreeset"}
// Dependencies: {}
# [macro_export (local_inner_macros)] # [doc = " Create a **BTreeSet** from a list of elements."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " #[macro_use] extern crate maplit;"] # [doc = " # fn main() {"] # [doc = ""] # [doc = " let set = btreeset!{\"a\", \"b\"};"] # [doc = " assert!(set.contains(\"a\"));"] # [doc = " assert!(set.contains(\"b\"));"] # [doc = " assert!(!set.contains(\"c\"));"] # [doc = " # }"] # [doc = " ```"] macro_rules ! btreeset { ($ ($ key : expr ,) +) => (btreeset ! ($ ($ key) ,+)) ; ($ ($ key : expr) ,*) => { { let mut _set = :: std :: collections :: BTreeSet :: new () ; $ (_set . insert ($ key) ;) * _set } } ; }
};
}
