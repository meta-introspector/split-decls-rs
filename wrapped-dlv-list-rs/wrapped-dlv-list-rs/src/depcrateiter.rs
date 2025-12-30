// Generated macro for Iter (struct)
macro_rules! DepcrateIter {
() => {
// Module: crate
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator that yields immutable references to entries in the list."] pub struct Iter < 'a , T > { # [doc = " A reference to the actual storage for the entry list."] entries : & 'a Vec < Entry < T > > , # [doc = " The index of the head of the unvisited portion of the list."] head : Option < NonMaxUsize > , # [doc = " The number of entries that have not been visited."] remaining : usize , # [doc = " The index of the tail of the unvisited portion of the list."] tail : Option < NonMaxUsize > , }
};
}
