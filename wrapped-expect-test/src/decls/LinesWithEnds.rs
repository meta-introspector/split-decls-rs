macro_rules! LinesWithEnds {
    () => {
        struct LinesWithEnds < 'a > { text : & 'a str , }
    };
}

LinesWithEnds!()