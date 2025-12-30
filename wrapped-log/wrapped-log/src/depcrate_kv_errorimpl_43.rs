// Generated macro for impl_43 (impl)
macro_rules! Depcrate_kv_errorimpl_43 {
() => {
// Module: crate::kv::error
// Provides: {"impl_43"}
// Dependencies: {}
impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use self :: Inner :: * ; match & self . inner { # [cfg (feature = "std")] Boxed (err) => err . fmt (f) , # [cfg (feature = "value-bag")] Value (err) => err . fmt (f) , Msg (msg) => msg . fmt (f) , Fmt => fmt :: Error . fmt (f) , } } }
};
}
