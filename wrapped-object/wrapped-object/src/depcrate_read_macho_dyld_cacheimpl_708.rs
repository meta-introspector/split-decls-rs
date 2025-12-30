// Generated macro for impl_708 (impl)
macro_rules! Depcrate_read_macho_dyld_cacheimpl_708 {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"impl_708"}
// Dependencies: {}
impl Debug for DyldRelocation { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DyldRelocation") . field ("offset" , & format_args ! ("{:#x}" , self . offset)) . field ("value" , & format_args ! ("{:#x}" , self . value)) . field ("auth" , & self . auth) . finish () } }
};
}
