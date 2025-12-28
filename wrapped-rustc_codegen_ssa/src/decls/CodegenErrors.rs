macro_rules! CodegenErrors {
    () => {
        pub enum CodegenErrors { WrongFileType , EmptyVersionNumber , EncodingVersionMismatch { version_array : String , rlink_version : u32 } , RustcVersionMismatch { rustc_version : String } , CorruptFile , }
    };
}

CodegenErrors!();