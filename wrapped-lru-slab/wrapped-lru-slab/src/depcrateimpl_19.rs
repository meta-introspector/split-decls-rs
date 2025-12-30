// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a , T > Iterator for IterMut < 'a , T > { type Item = (u32 , & 'a mut T) ; fn next (& mut self) -> Option < (u32 , & 'a mut T) > { unsafe { let idx = self . state . next (| i | * addr_of_mut ! ((* self . slots . add (i as usize)) . next)) ? ; let result = (* addr_of_mut ! ((* self . slots . add (idx as usize)) . value)) . as_mut () . expect ("corrupt LRU list") ; Some ((idx , result)) } } fn size_hint (& self) -> (usize , Option < usize >) { (self . state . len as usize , Some (self . state . len as usize)) } }
};
}
