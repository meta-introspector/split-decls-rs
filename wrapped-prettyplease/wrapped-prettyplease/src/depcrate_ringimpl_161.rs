// Generated macro for impl_161 (impl)
macro_rules! Depcrate_ringimpl_161 {
() => {
// Module: crate::ring
// Provides: {"impl_161"}
// Dependencies: {}
impl < T > RingBuffer < T > { pub fn new () -> Self { RingBuffer { data : VecDeque :: new () , offset : 0 , } } pub fn is_empty (& self) -> bool { self . data . is_empty () } pub fn len (& self) -> usize { self . data . len () } pub fn push (& mut self , value : T) -> usize { let index = self . offset + self . data . len () ; self . data . push_back (value) ; index } pub fn clear (& mut self) { self . data . clear () ; } pub fn index_range (& self) -> Range < usize > { self . offset .. self . offset + self . data . len () } pub fn first (& self) -> & T { & self . data [0] } pub fn first_mut (& mut self) -> & mut T { & mut self . data [0] } pub fn pop_first (& mut self) -> T { self . offset += 1 ; self . data . pop_front () . unwrap () } pub fn last (& self) -> & T { self . data . back () . unwrap () } pub fn last_mut (& mut self) -> & mut T { self . data . back_mut () . unwrap () } pub fn second_last (& self) -> & T { & self . data [self . data . len () - 2] } pub fn pop_last (& mut self) { self . data . pop_back () . unwrap () ; } }
};
}
