// Generated macro for impl_136 (impl)
macro_rules! Depcrate_filesimpl_136 {
() => {
// Module: crate::files
// Provides: {"impl_136"}
// Dependencies: {}
impl < FileKind , T > InFileWrapper < FileKind , Option < T > > { pub fn transpose (self) -> Option < InFileWrapper < FileKind , T > > { Some (InFileWrapper :: new (self . file_id , self . value ?)) } }
};
}
