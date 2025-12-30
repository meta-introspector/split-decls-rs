// Generated macro for impl_191 (impl)
macro_rules! Depcrate_fileimpl_191 {
() => {
// Module: crate::file
// Provides: {"impl_191"}
// Dependencies: {}
impl < F > File < FileSourceString , F > where F : FileStoredFormat + 'static , { pub fn from_str (s : & str , format : F) -> Self { Self { format : Some (format) , required : true , source : s . into () , } } }
};
}
