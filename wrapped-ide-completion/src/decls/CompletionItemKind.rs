macro_rules! deps {
    () => {
        Snippet!();
    };
}

macro_rules! CompletionItemKind {
    () => {
        deps!();
        # [doc = " The type of the completion item."] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub enum CompletionItemKind { SymbolKind (SymbolKind) , Binding , BuiltinType , InferredType , Keyword , Snippet , UnresolvedReference , Expression , }
    };
}

CompletionItemKind!();