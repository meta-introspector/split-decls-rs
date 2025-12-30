// Generated macro for impl_12 (impl)
macro_rules! Depcrate_entryimpl_12 {
() => {
// Module: crate::entry
// Provides: {"impl_12"}
// Dependencies: {}
impl std :: fmt :: Debug for Entry < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { f . debug_struct ("Entry") . field ("path_buf" , & self . relative_path ()) . field ("mode" , & self . mode) . field ("id" , & self . id) . field ("remaining" , & self . remaining) . finish () } }
};
}
