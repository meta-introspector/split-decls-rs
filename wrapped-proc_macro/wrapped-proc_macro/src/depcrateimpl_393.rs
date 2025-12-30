// Generated macro for impl_393 (impl)
macro_rules! Depcrateimpl_393 {
() => {
// Module: crate
// Provides: {"impl_393"}
// Dependencies: {}
# [stable (feature = "proc_macro_lib2" , since = "1.29.0")] impl fmt :: Debug for Literal { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Literal") . field ("kind" , & format_args ! ("{:?}" , self . 0 . kind)) . field ("symbol" , & self . 0 . symbol) . field ("suffix" , & format_args ! ("{:?}" , self . 0 . suffix)) . field ("span" , & self . 0 . span) . finish () } }
};
}
