// Generated macro for impl_301 (impl)
macro_rules! Depcrate_errorimpl_301 {
() => {
// Module: crate::error
// Provides: {"impl_301"}
// Dependencies: {}
impl core :: fmt :: Debug for FilePathError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { # [cfg (feature = "std")] { f . debug_struct ("FilePathError") . field ("path" , & self . path) . finish () } # [cfg (not (feature = "std"))] { write ! (f , "<BUG: SHOULD NOT EXIST>") } } }
};
}
