// Generated macro for impl_45 (impl)
macro_rules! Depcrate_generatedimpl_45 {
() => {
// Module: crate::generated
// Provides: {"impl_45"}
// Dependencies: {}
impl LSMMap { # [doc = " Compiles the map if necessary and then stores it into the given file."] # [doc (alias = "LSMMapWriteToURL")] # [inline] pub unsafe fn write_to_url (& self , file : & CFURL , flags : CFOptionFlags) -> OSStatus { extern "C-unwind" { fn LSMMapWriteToURL (mapref : & LSMMap , file : & CFURL , flags : CFOptionFlags) -> OSStatus ; } unsafe { LSMMapWriteToURL (self , file , flags) } } # [doc = " Loads a map from a given file."] # [doc (alias = "LSMMapCreateFromURL")] # [inline] pub unsafe fn from_url (alloc : Option < & CFAllocator > , file : & CFURL , flags : CFOptionFlags ,) -> Option < CFRetained < LSMMap > > { extern "C-unwind" { fn LSMMapCreateFromURL (alloc : Option < & CFAllocator > , file : & CFURL , flags : CFOptionFlags ,) -> Option < NonNull < LSMMap > > ; } let ret = unsafe { LSMMapCreateFromURL (alloc , file , flags) } ; ret . map (| ret | unsafe { CFRetained :: from_raw (ret) }) } }
};
}
