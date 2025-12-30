// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl < 'a , T > Iterator for Iter < 'a , T > { type Item = (u32 , & 'a T) ; fn next (& mut self) -> Option < (u32 , & 'a T) > { let idx = self . state . next (| i | self . slots [i as usize] . next) ? ; let result = self . slots [idx as usize] . value . as_ref () . expect ("corrupt LRU list") ; Some ((idx , result)) } fn size_hint (& self) -> (usize , Option < usize >) { (self . state . len as usize , Some (self . state . len as usize)) } }
};
}
