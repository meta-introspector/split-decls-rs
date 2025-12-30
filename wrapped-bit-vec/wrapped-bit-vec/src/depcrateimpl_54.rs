// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'a , B : 'a + BitBlock > IterMut < 'a , B > { fn get (& mut self , index : Option < usize >) -> Option < MutBorrowedBit < 'a , B > > { let value = (* self . vec) . borrow () . get (index ?) ? ; Some (MutBorrowedBit { vec : self . vec . clone () , index : index ? , # [cfg (debug_assertions)] old_value : value , new_value : value , }) } }
};
}
