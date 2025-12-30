// Generated macro for impl_48 (impl)
macro_rules! Depcrateimpl_48 {
() => {
// Module: crate
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'q > Index < & 'q str > for Ini { type Output = Properties ; fn index < 'a > (& 'a self , index : & 'q str) -> & 'a Properties { match self . section (Some (index)) { Some (p) => p , None => panic ! ("Section `{}` does not exist" , index) , } } }
};
}
