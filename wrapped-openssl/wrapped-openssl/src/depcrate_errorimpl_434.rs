// Generated macro for impl_434 (impl)
macro_rules! Depcrate_errorimpl_434 {
() => {
// Module: crate::error
// Provides: {"impl_434"}
// Dependencies: {}
impl fmt :: Debug for Error { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut builder = fmt . debug_struct ("Error") ; builder . field ("code" , & self . code ()) ; if let Some (library) = self . library () { builder . field ("library" , & library) ; } if let Some (function) = self . function () { builder . field ("function" , & function) ; } if let Some (reason) = self . reason () { builder . field ("reason" , & reason) ; } builder . field ("file" , & self . file ()) ; builder . field ("line" , & self . line ()) ; if let Some (data) = self . data () { builder . field ("data" , & data) ; } builder . finish () } }
};
}
