// Generated macro for impl_33 (impl)
macro_rules! Depcrateimpl_33 {
() => {
// Module: crate
// Provides: {"impl_33"}
// Dependencies: {}
impl fmt :: Debug for Mmap { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("Mmap") . field ("ptr" , & self . as_ptr ()) . field ("len" , & self . len ()) . finish () } }
};
}
