// Generated macro for impl_692 (impl)
macro_rules! Depcrate_read_macho_dyld_cacheimpl_692 {
() => {
// Module: crate::read::macho::dyld_cache
// Provides: {"impl_692"}
// Dependencies: {}
impl < 'data , E , R > Debug for DyldCacheMapping < 'data , E , R > where E : Endian , R : ReadRef < 'data > , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DyldCacheMapping") . field ("address" , & format_args ! ("{:#x}" , self . address ())) . field ("size" , & format_args ! ("{:#x}" , self . size ())) . field ("file_offset" , & format_args ! ("{:#x}" , self . file_offset ())) . field ("max_prot" , & format_args ! ("{:#x}" , self . max_prot ())) . field ("init_prot" , & format_args ! ("{:#x}" , self . init_prot ())) . finish () } }
};
}
