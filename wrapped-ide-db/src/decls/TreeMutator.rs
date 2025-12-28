macro_rules! TreeMutator {
    () => {
        pub struct TreeMutator { immutable : SyntaxNode , mutable_clone : SyntaxNode , }
    };
}

TreeMutator!()