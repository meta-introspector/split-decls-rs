// Generated macro for CodegenErrors (enum)
macro_rules! DepcrateCodegenErrors {
() => {
// Module: crate
// Provides: {"CodegenErrors"}
// Dependencies: {}
pub enum CodegenErrors { WrongFileType , EmptyVersionNumber , EncodingVersionMismatch { version_array : String , rlink_version : u32 } , RustcVersionMismatch { rustc_version : String } , CorruptFile , }
};
}
