// Generated macro for indexset_with_default (macro)
macro_rules! Depcrate_macrosindexset_with_default {
() => {
// Module: crate::macros
// Provides: {"indexset_with_default"}
// Dependencies: {}
# [doc = " Create an [`IndexSet`][crate::IndexSet] from a list of values"] # [doc = " and a [`BuildHasherDefault`][core::hash::BuildHasherDefault]-wrapped custom hasher."] # [doc = ""] # [doc = " ## Example"] # [doc = ""] # [doc = " ```"] # [doc = " use indexmap::indexset_with_default;"] # [doc = " use fnv::FnvHasher;"] # [doc = ""] # [doc = " let set = indexset_with_default!{"] # [doc = "     FnvHasher;"] # [doc = "     \"a\","] # [doc = "     \"b\","] # [doc = " };"] # [doc = " assert!(set.contains(\"a\"));"] # [doc = " assert!(set.contains(\"b\"));"] # [doc = " assert!(!set.contains(\"c\"));"] # [doc = ""] # [doc = " // \"a\" is the first value"] # [doc = " assert_eq!(set.iter().next(), Some(&\"a\"));"] # [doc = " ```"] # [macro_export] macro_rules ! indexset_with_default { ($ H : ty ; $ ($ value : expr ,) +) => { $ crate :: indexset_with_default ! ($ H ; $ ($ value) ,+) } ; ($ H : ty ; $ ($ value : expr) ,*) => { { let builder = :: core :: hash :: BuildHasherDefault ::<$ H >:: default () ; const CAP : usize = < [()] >:: len (& [$ ({ stringify ! ($ value) ; }) ,*]) ; # [allow (unused_mut)] let mut set = $ crate :: IndexSet :: with_capacity_and_hasher (CAP , builder) ; $ (set . insert ($ value) ;) * set } } ; }
};
}
