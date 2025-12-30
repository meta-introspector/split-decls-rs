// Generated macro for IntoIter (struct)
macro_rules! DepcrateIntoIter {
() => {
// Module: crate
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An iterator that moves all entries out of the entry list."] # [derive (Clone)] pub struct IntoIter < T > { # [doc = " The index of the head of the unvisited portion of the list."] head : Option < NonMaxUsize > , # [doc = " The entry list from which entries are yielded."] list : VecList < T > , # [doc = " The number of entries that have not been visited."] remaining : usize , # [doc = " The index of the tail of the unvisited portion of the list."] tail : Option < NonMaxUsize > , }
};
}
