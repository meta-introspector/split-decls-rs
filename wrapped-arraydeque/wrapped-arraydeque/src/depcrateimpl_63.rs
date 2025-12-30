// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < T , const CAP : usize , B : Behavior > IndexMut < usize > for ArrayDeque < T , CAP , B > { # [inline] fn index_mut (& mut self , index : usize) -> & mut T { let len = self . len () ; self . get_mut (index) . or_else (| | { panic ! ("index out of bounds: the len is {} but the index is {}" , len , index) }) . unwrap () } }
};
}
