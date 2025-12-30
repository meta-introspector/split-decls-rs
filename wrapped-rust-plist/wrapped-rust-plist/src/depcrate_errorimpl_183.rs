// Generated macro for impl_183 (impl)
macro_rules! Depcrate_errorimpl_183 {
() => {
// Module: crate::error
// Provides: {"impl_183"}
// Dependencies: {}
impl ErrorKind { pub fn with_byte_offset (self , offset : u64) -> Error { self . with_position (FilePosition (offset)) } pub fn with_position (self , pos : FilePosition) -> Error { Error { inner : Box :: new (ErrorImpl { kind : self , file_position : Some (pos) , }) , } } pub fn without_position (self) -> Error { Error { inner : Box :: new (ErrorImpl { kind : self , file_position : None , }) , } } }
};
}
