// Generated macro for impl_46 (impl)
macro_rules! Depcrateimpl_46 {
() => {
// Module: crate
// Provides: {"impl_46"}
// Dependencies: {}
impl < S : Into < String > > Index < Option < S > > for Ini { type Output = Properties ; fn index (& self , index : Option < S >) -> & Properties { match self . section (index) { Some (p) => p , None => panic ! ("Section does not exist") , } } }
};
}
