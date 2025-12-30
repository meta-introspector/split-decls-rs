// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl < S : Into < String > > IndexMut < Option < S > > for Ini { fn index_mut (& mut self , index : Option < S >) -> & mut Properties { match self . section_mut (index) { Some (p) => p , None => panic ! ("Section does not exist") , } } }
};
}
