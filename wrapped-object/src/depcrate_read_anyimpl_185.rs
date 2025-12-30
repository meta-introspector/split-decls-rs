// Generated macro for impl_185 (impl)
macro_rules! Depcrate_read_anyimpl_185 {
() => {
// Module: crate::read::any
// Provides: {"impl_185"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > fmt :: Debug for Segment < 'data , 'file , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("Segment") ; match self . name () { Ok (Some (ref name)) => { s . field ("name" , name) ; } Ok (None) => { } Err (_) => { s . field ("name" , & "<invalid>") ; } } s . field ("address" , & self . address ()) . field ("size" , & self . size ()) . finish () } }
};
}
