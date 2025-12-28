macro_rules! deps {
    () => {
        Token!();
        SsrError!();
    };
}

macro_rules! tokenize {
    () => {
        deps!();
        fn tokenize (source : & str) -> Result < Vec < Token > , SsrError > { let lexed = parser :: LexedStr :: new (parser :: Edition :: CURRENT , source) ; if let Some ((_ , first_error)) = lexed . errors () . next () { bail ! ("Failed to parse pattern: {}" , first_error) ; } let mut tokens : Vec < Token > = Vec :: new () ; for i in 0 .. lexed . len () { tokens . push (Token { kind : lexed . kind (i) , text : lexed . text (i) . into () }) ; } Ok (tokens) }
    };
}

tokenize!()