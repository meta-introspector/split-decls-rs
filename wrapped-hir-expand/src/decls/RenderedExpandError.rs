macro_rules! RenderedExpandError {
    () => {
        pub struct RenderedExpandError { pub message : String , pub error : bool , pub kind : & 'static str , }
    };
}

RenderedExpandError!();