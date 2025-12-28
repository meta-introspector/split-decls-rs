macro_rules! deps {
    () => {
        PatternIterator!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl Iterator for PatternIterator { type Item = SyntaxElement ; fn next (& mut self) -> Option < SyntaxElement > { self . iter . find (| element | ! element . kind () . is_trivia ()) } }
    };
}

impl_29!()