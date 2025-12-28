macro_rules! deps {
    () => {
        LinesWithEnds!();
    };
}

macro_rules! lines_with_ends {
    () => {
        deps!();
        fn lines_with_ends (text : & str) -> LinesWithEnds { LinesWithEnds { text } }
    };
}

lines_with_ends!();