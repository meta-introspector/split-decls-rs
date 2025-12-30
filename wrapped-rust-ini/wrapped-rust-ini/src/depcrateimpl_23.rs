// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl < S : AsRef < str > > Index < S > for Properties { type Output = str ; fn index (& self , index : S) -> & str { let s = index . as_ref () ; match self . get (s) { Some (p) => p , None => panic ! ("Key `{}` does not exist" , s) , } } }
};
}
