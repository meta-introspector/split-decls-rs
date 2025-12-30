// Generated macro for CodeView (struct)
macro_rules! Depcrate_readCodeView {
() => {
// Module: crate::read
// Provides: {"CodeView"}
// Dependencies: {}
# [doc = " PDB information from the debug directory in a PE file."] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub struct CodeView < 'data > { guid : [u8 ; 16] , path : ByteString < 'data > , age : u32 , }
};
}
