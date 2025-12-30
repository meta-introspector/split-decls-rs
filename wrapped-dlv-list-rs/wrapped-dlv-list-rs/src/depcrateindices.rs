// Generated macro for Indices (struct)
macro_rules! DepcrateIndices {
() => {
// Module: crate
// Provides: {"Indices"}
// Dependencies: {}
# [doc = " An iterator that yields all indices in the list."] pub struct Indices < 'a , T > { # [doc = " A reference to the actual storage for the entry list."] entries : & 'a Vec < Entry < T > > , # [doc = " The index of the head of the unvisited portion of the list."] head : Option < NonMaxUsize > , # [doc = " The number of entries that have not been visited."] remaining : usize , # [doc = " The index of the tail of the unvisited portion of the list."] tail : Option < NonMaxUsize > , }
};
}
