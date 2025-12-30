// Generated macro for impl_131 (impl)
macro_rules! Depcrate_filesimpl_131 {
() => {
// Module: crate::files
// Provides: {"impl_131"}
// Dependencies: {}
impl < FileKind , T > InFileWrapper < FileKind , T > { pub fn new (file_id : FileKind , value : T) -> Self { Self { file_id , value } } pub fn map < F : FnOnce (T) -> U , U > (self , f : F) -> InFileWrapper < FileKind , U > { InFileWrapper :: new (self . file_id , f (self . value)) } }
};
}
