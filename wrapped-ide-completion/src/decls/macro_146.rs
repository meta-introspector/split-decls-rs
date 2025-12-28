macro_rules! deps {
    () => {
        CompletionItemKind!();
    };
}

macro_rules! macro_146 {
    () => {
        deps!();
        impl_from ! (SymbolKind for CompletionItemKind) ;
    };
}

macro_146!()