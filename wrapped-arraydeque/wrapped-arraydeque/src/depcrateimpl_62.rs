// Generated macro for impl_62 (impl)
macro_rules! Depcrateimpl_62 {
() => {
// Module: crate
// Provides: {"impl_62"}
// Dependencies: {}
impl < T , const CAP : usize , B : Behavior > Index < usize > for ArrayDeque < T , CAP , B > { type Output = T ; # [inline] fn index (& self , index : usize) -> & T { let len = self . len () ; self . get (index) . or_else (| | { panic ! ("index out of bounds: the len is {} but the index is {}" , len , index) }) . unwrap () } }
};
}
