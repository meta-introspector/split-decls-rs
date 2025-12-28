macro_rules! MkFile {
    () => {
        # [doc = " Builder for configuring a file to copy into a container."] pub struct MkFile { path : String , contents : Vec < u8 > , header : Header , }
    };
}

MkFile!()