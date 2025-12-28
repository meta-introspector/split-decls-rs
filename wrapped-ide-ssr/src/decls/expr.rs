macro_rules! expr {
    () => {
        pub (crate) fn expr (s : & str) -> Result < SyntaxNode , () > { fragment :: < ast :: Expr > ("const _: () = {};" , s) }
    };
}

expr!();