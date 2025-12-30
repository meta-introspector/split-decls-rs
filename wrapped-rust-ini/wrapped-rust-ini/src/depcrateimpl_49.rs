// Generated macro for impl_49 (impl)
macro_rules! Depcrateimpl_49 {
() => {
// Module: crate
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'q > IndexMut < & 'q str > for Ini { fn index_mut < 'a > (& 'a mut self , index : & 'q str) -> & 'a mut Properties { match self . section_mut (Some (index)) { Some (p) => p , None => panic ! ("Section `{}` does not exist" , index) , } } }
};
}
