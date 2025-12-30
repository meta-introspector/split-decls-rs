// Generated macro for Iter (struct)
macro_rules! DepcrateIter {
() => {
// Module: crate
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Iterator over elements of an [`LruSlab`], from most to least recently used"] pub struct Iter < 'a , T > { slots : & 'a [Slot < T >] , state : IterState , }
};
}
