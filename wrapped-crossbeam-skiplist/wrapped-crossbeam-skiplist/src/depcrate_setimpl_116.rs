// Generated macro for impl_116 (impl)
macro_rules! Depcrate_setimpl_116 {
() => {
// Module: crate::set
// Provides: {"impl_116"}
// Dependencies: {}
impl < T > SkipSet < T > { # [doc = " Returns a new, empty set."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_skiplist::SkipSet;"] # [doc = ""] # [doc = " let set: SkipSet<i32> = SkipSet::new();"] # [doc = " ```"] pub fn new () -> Self { Self { inner : map :: SkipMap :: new () , } } # [doc = " Returns `true` if the set is empty."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_skiplist::SkipSet;"] # [doc = ""] # [doc = " let set = SkipSet::new();"] # [doc = " assert!(set.is_empty());"] # [doc = ""] # [doc = " set.insert(1);"] # [doc = " assert!(!set.is_empty());"] # [doc = " ```"] pub fn is_empty (& self) -> bool { self . inner . is_empty () } # [doc = " Returns the number of entries in the set."] # [doc = ""] # [doc = " If the set is being concurrently modified, consider the returned number just an"] # [doc = " approximation without any guarantees."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_skiplist::SkipSet;"] # [doc = ""] # [doc = " let set = SkipSet::new();"] # [doc = " assert_eq!(set.len(), 0);"] # [doc = ""] # [doc = " set.insert(1);"] # [doc = " assert_eq!(set.len(), 1);"] # [doc = " ```"] pub fn len (& self) -> usize { self . inner . len () } }
};
}
