macro_rules! deps {
    () => {
        LexError!();
    };
}

macro_rules! LexedStr {
    () => {
        deps!();
        pub struct LexedStr < 'a > { text : & 'a str , kind : Vec < SyntaxKind > , start : Vec < u32 > , error : Vec < LexError > , }
    };
}

LexedStr!()