// Generated macro for impl_300 (impl)
macro_rules! Depcrate_errorimpl_300 {
() => {
// Module: crate::error
// Provides: {"impl_300"}
// Dependencies: {}
impl core :: fmt :: Display for FilePathError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { # [cfg (feature = "std")] { write ! (f , "{}" , self . path . display ()) } # [cfg (not (feature = "std"))] { write ! (f , "<BUG: SHOULD NOT EXIST>") } } }
};
}
