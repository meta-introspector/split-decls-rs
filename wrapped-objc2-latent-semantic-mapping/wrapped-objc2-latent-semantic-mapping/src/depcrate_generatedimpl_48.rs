// Generated macro for impl_48 (impl)
macro_rules! Depcrate_generatedimpl_48 {
() => {
// Module: crate::generated
// Provides: {"impl_48"}
// Dependencies: {}
impl LSMMap { # [doc = " Writes information about a map and/or text to a stream in text form"] # [doc (alias = "LSMMapWriteToStream")] # [inline] pub unsafe fn write_to_stream (& self , textref : Option < & LSMText > , stream : & CFWriteStream , options : CFOptionFlags ,) -> OSStatus { extern "C-unwind" { fn LSMMapWriteToStream (mapref : & LSMMap , textref : Option < & LSMText > , stream : & CFWriteStream , options : CFOptionFlags ,) -> OSStatus ; } unsafe { LSMMapWriteToStream (self , textref , stream , options) } } }
};
}
