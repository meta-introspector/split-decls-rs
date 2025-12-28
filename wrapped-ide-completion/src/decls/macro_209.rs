macro_rules! deps {
    () => {
        CompletionItemKind!();
    };
}

macro_rules! macro_209 {
    () => {
        deps!();
        impl_from ! (SymbolKind for CompletionItemKind) ;
    };
}

macro_209!()