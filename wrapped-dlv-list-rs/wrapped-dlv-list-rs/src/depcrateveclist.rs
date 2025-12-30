// Generated macro for VecList (struct)
macro_rules! DepcrateVecList {
() => {
// Module: crate
// Provides: {"VecList"}
// Dependencies: {}
# [doc = " A semi-doubly linked list implemented with a vector."] # [doc = ""] # [doc = " This provides many of the benefits of an actual linked list with a few tradeoffs. First, due to the use of an"] # [doc = " underlying vector, an individual insert operation may be O(n) due to allocating more space for the vector. However,"] # [doc = " it is amortized O(1) and it avoids the frequent allocations that traditional linked lists suffer from."] # [doc = ""] # [doc = " Another tradeoff is that extending a traditional linked list with another list is O(1) but a vector based"] # [doc = " implementation is O(n). Splicing has a similar disadvantage."] # [doc = ""] # [doc = " Lastly, the vector based implementation is likely to have better cache locality in general."] pub struct VecList < T > { # [doc = " The backing storage for the list. This includes both used and unused indices."] entries : Vec < Entry < T > > , # [doc = " The current generation of the list. This is used to avoid the ABA problem."] generation : u64 , # [doc = " The index of the head of the list."] head : Option < NonMaxUsize > , # [doc = " The length of the list since we cannot rely on the length of [`VecList::entries`] because it includes unused"] # [doc = " indices."] length : usize , # [doc = " The index of the tail of the list."] tail : Option < NonMaxUsize > , # [doc = " The index of the head of the vacant indices."] vacant_head : Option < NonMaxUsize > , }
};
}
