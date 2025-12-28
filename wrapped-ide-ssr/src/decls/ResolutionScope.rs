macro_rules! ResolutionScope {
    () => {
        pub (crate) struct ResolutionScope < 'db > { scope : hir :: SemanticsScope < 'db > , node : SyntaxNode , }
    };
}

ResolutionScope!()