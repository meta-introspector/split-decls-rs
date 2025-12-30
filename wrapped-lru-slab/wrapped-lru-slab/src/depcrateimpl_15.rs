// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'a , T > DoubleEndedIterator for Iter < 'a , T > { fn next_back (& mut self) -> Option < (u32 , & 'a T) > { let idx = self . state . next_back (| i | self . slots [i as usize] . prev) ? ; let result = self . slots [idx as usize] . value . as_ref () . expect ("corrupt LRU list") ; Some ((idx , result)) } }
};
}
