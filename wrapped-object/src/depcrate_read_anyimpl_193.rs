// Generated macro for impl_193 (impl)
macro_rules! Depcrate_read_anyimpl_193 {
() => {
// Module: crate::read::any
// Provides: {"impl_193"}
// Dependencies: {}
impl < 'data , 'file , R : ReadRef < 'data > > fmt :: Debug for Section < 'data , 'file , R > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("Section") ; match self . segment_name () { Ok (Some (ref name)) => { s . field ("segment" , name) ; } Ok (None) => { } Err (_) => { s . field ("segment" , & "<invalid>") ; } } s . field ("name" , & self . name () . unwrap_or ("<invalid>")) . field ("address" , & self . address ()) . field ("size" , & self . size ()) . field ("align" , & self . align ()) . field ("kind" , & self . kind ()) . field ("flags" , & self . flags ()) . finish () } }
};
}
