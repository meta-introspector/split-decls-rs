macro_rules! deps {
    () => {
        PatternIterator!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl PatternIterator { fn new (parent : & SyntaxNode) -> Self { Self { iter : parent . children_with_tokens () } } }
    };
}

impl_30!()