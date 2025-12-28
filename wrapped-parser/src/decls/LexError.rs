macro_rules! LexError {
    () => {
        struct LexError { msg : String , token : u32 , }
    };
}

LexError!()