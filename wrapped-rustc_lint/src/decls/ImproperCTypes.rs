macro_rules! ImproperCTypes {
    () => {
        pub (crate) struct ImproperCTypes < 'a > { pub ty : Ty < 'a > , pub desc : & 'a str , pub label : Span , pub help : Option < DiagMessage > , pub note : DiagMessage , pub span_note : Option < Span > , }
    };
}

ImproperCTypes!();