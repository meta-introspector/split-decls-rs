// Generated macro for impl_1046 (impl)
macro_rules! Depcrate_collections_vec_dequeimpl_1046 {
() => {
// Module: crate::collections::vec_deque
// Provides: {"impl_1046"}
// Dependencies: {}
impl < T : Clone , A : Allocator > VecDeque < T , A > { # [doc = " Modifies the deque in-place so that `len()` is equal to new_len,"] # [doc = " either by removing excess elements from the back or by appending clones of `value`"] # [doc = " to the back."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::collections::VecDeque;"] # [doc = ""] # [doc = " let mut buf = VecDeque::new();"] # [doc = " buf.push_back(5);"] # [doc = " buf.push_back(10);"] # [doc = " buf.push_back(15);"] # [doc = " assert_eq!(buf, [5, 10, 15]);"] # [doc = ""] # [doc = " buf.resize(2, 0);"] # [doc = " assert_eq!(buf, [5, 10]);"] # [doc = ""] # [doc = " buf.resize(5, 20);"] # [doc = " assert_eq!(buf, [5, 10, 20, 20, 20]);"] # [doc = " ```"] # [stable (feature = "deque_extras" , since = "1.16.0")] # [track_caller] pub fn resize (& mut self , new_len : usize , value : T) { if new_len > self . len () { let extra = new_len - self . len () ; self . extend (repeat_n (value , extra)) } else { self . truncate (new_len) ; } } }
};
}
