// Generated macro for Drain (struct)
macro_rules! DepcrateDrain {
() => {
// Module: crate
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " An iterator that yields and removes all entries from the list."] pub struct Drain < 'a , T > { # [doc = " The index of the head of the unvisited portion of the list."] head : Option < NonMaxUsize > , # [doc = " A reference to the entry list."] list : & 'a mut VecList < T > , # [doc = " The number of entries that have not been visited."] remaining : usize , # [doc = " The index of the tail of the unvisited portion of the list."] tail : Option < NonMaxUsize > , }
};
}
