macro_rules! deps {
    () => {
        CompletionItemKind!();
    };
}

macro_rules! macro_57 {
    () => {
        deps!();
        impl_from ! (SymbolKind for CompletionItemKind) ;
    };
}

macro_57!()