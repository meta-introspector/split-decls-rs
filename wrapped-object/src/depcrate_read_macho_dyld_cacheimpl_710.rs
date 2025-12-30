// Generated macro for impl_710 (impl)
macro_rules! Depcrate_read_macho_dyld_cacheimpl_710 {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"impl_710"}
// Dependencies: {}
impl Debug for DyldRelocationAuth { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Ptrauth") . field ("key" , & self . key) . field ("diversity" , & format_args ! ("{:#x}" , self . diversity)) . field ("addr_div" , & self . addr_div) . finish () } }
};
}
