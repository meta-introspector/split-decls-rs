macro_rules! PatternIterator {
    () => {
        struct PatternIterator { iter : SyntaxElementChildren , }
    };
}

PatternIterator!();