// Generated macro for impl_296 (impl)
macro_rules! Depcrate_errorimpl_296 {
() => {
// Module: crate::error
// Provides: {"impl_296"}
// Dependencies: {}
impl core :: fmt :: Debug for IOError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { # [cfg (feature = "std")] { f . debug_struct ("IOError") . field ("err" , & self . err) . finish () } # [cfg (not (feature = "std"))] { write ! (f , "<BUG: SHOULD NOT EXIST>") } } }
};
}
