macro_rules! item {
    () => {
        pub (crate) fn item (s : & str) -> Result < SyntaxNode , () > { fragment :: < ast :: Item > ("{}" , s) }
    };
}

item!()