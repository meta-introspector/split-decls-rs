macro_rules! deps {
    () => {
        LexedStr!();
    };
}

macro_rules! lex {
    () => {
        deps!();
        fn lex (text : & str , edition : Edition) -> String { let lexed = LexedStr :: new (edition , text) ; let mut res = String :: new () ; for i in 0 .. lexed . len () { let kind = lexed . kind (i) ; let text = lexed . text (i) ; let error = lexed . error (i) ; let error = error . map (| err | format ! (" error: {err}")) . unwrap_or_default () ; writeln ! (res , "{kind:?} {text:?}{error}") . unwrap () ; } res }
    };
}

lex!()