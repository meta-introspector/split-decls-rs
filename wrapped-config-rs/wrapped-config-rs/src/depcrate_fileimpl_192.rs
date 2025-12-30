// Generated macro for impl_192 (impl)
macro_rules! Depcrate_fileimpl_192 {
() => {
// Module: crate::file
// Provides: {"impl_192"}
// Dependencies: {}
impl < F > File < FileSourceFile , F > where F : FileStoredFormat + 'static , { pub fn new (name : & str , format : F) -> Self { Self { format : Some (format) , required : true , source : FileSourceFile :: new (name . into ()) , } } }
};
}
