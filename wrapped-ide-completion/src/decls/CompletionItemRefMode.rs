macro_rules! CompletionItemRefMode {
    () => {
        # [derive (Copy , Clone , Debug)] pub enum CompletionItemRefMode { Reference (Mutability) , Dereference , }
    };
}

CompletionItemRefMode!()