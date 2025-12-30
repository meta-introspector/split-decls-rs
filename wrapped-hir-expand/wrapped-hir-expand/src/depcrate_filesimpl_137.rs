// Generated macro for impl_137 (impl)
macro_rules! Depcrate_filesimpl_137 {
() => {
// Module: crate::files
// Provides: {"impl_137"}
// Dependencies: {}
impl < FileKind , L , R > InFileWrapper < FileKind , Either < L , R > > { pub fn transpose (self) -> Either < InFileWrapper < FileKind , L > , InFileWrapper < FileKind , R > > { match self . value { Either :: Left (l) => Either :: Left (InFileWrapper :: new (self . file_id , l)) , Either :: Right (r) => Either :: Right (InFileWrapper :: new (self . file_id , r)) , } } }
};
}
