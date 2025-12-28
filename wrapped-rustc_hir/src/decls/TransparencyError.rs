macro_rules! TransparencyError {
    () => {
        pub enum TransparencyError { UnknownTransparency (Symbol , Span) , MultipleTransparencyAttrs (Span , Span) , }
    };
}

TransparencyError!();