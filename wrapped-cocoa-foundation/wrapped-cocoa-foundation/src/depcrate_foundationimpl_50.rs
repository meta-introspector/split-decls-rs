// Generated macro for impl_50 (impl)
macro_rules! Depcrate_foundationimpl_50 {
() => {
// Module: crate::foundation
// Provides: {"impl_50"}
// Dependencies: {}
impl Iterator for NSFastIterator { type Item = id ; fn next (& mut self) -> Option < id > { if self . idx >= self . len { self . len = unsafe { msg_send ! [self . object , countByEnumeratingWithState :& mut self . state objects : self . buffer . as_mut_ptr () count : NS_FAST_ENUM_BUF_SIZE] } ; self . idx = 0 ; } let new_mut = unsafe { * self . state . mutations_ptr } ; if let Some (old_mut) = self . mut_val { assert ! (old_mut == new_mut , "The collection was mutated while being enumerated") ; } if self . idx < self . len { let object = unsafe { * self . state . items_ptr . add (self . idx) } ; self . mut_val = Some (new_mut) ; self . idx += 1 ; Some (object) } else { None } } }
};
}
