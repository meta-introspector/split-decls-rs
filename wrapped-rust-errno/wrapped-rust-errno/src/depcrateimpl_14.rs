// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl fmt :: Display for Errno { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { sys :: with_description (* self , | desc | match desc { Ok (desc) => fmt . write_str (desc) , Err (fm_err) => write ! (fmt , "OS error {} ({} returned error {})" , self . 0 , sys :: STRERROR_NAME , fm_err . 0) , }) } }
};
}
