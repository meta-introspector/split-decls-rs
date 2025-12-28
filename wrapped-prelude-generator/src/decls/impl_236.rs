macro_rules! deps {
    () => {
        SynFileParser!();
        RustcInfo!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl SynFileParser { pub async fn parse_expanded_code (_writer : & mut (impl tokio :: io :: AsyncWriteExt + Unpin) , _file_path : & Path , expanded_code : String , _rustc_info : & RustcInfo ,) -> Result < (File , Option < ErrorSample >) > { Ok ((syn :: parse_file (& expanded_code) ? , None)) } }
    };
}

impl_236!()