// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl core :: iter :: DoubleEndedIterator for ArrayIntoIter { fn next_back (& mut self) -> Option < Self :: Item > { let index = self . range . next_back () ? ; Some (self . array . get (index)) } fn nth_back (& mut self , n : usize) -> Option < Self :: Item > { self . range . nth_back (n) . map (| index | self . array . get (index)) } }
};
}
