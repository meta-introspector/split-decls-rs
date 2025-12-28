macro_rules! ty {
    () => {
        pub (crate) fn ty (s : & str) -> Result < SyntaxNode , () > { fragment :: < ast :: Type > ("type T = {};" , s) }
    };
}

ty!();