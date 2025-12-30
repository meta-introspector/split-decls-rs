// Generated macro for impl_295 (impl)
macro_rules! Depcrate_errorimpl_295 {
() => {
// Module: crate::error
// Provides: {"impl_295"}
// Dependencies: {}
impl core :: fmt :: Display for IOError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { # [cfg (feature = "std")] { write ! (f , "{}" , self . err) } # [cfg (not (feature = "std"))] { write ! (f , "<BUG: SHOULD NOT EXIST>") } } }
};
}
