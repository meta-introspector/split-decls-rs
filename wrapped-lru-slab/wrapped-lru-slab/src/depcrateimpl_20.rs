// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a , T > DoubleEndedIterator for IterMut < 'a , T > { fn next_back (& mut self) -> Option < (u32 , & 'a mut T) > { unsafe { let idx = self . state . next_back (| i | * addr_of_mut ! ((* self . slots . add (i as usize)) . prev)) ? ; let result = (* addr_of_mut ! ((* self . slots . add (idx as usize)) . value)) . as_mut () . expect ("corrupt LRU list") ; Some ((idx , result)) } } }
};
}
