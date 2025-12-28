macro_rules! deps {
    () => {
        ClassAsciiKind!();
    };
}

macro_rules! ascii_class_as_chars {
    () => {
        deps!();
        fn ascii_class_as_chars (kind : & ast :: ClassAsciiKind ,) -> impl Iterator < Item = (char , char) > { ascii_class (kind) . map (| (s , e) | (char :: from (s) , char :: from (e))) }
    };
}

ascii_class_as_chars!()