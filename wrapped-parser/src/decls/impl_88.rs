macro_rules! impl_88 {
    () => {
        impl SyntaxKind { # [inline] pub fn is_trivia (self) -> bool { matches ! (self , SyntaxKind :: WHITESPACE | SyntaxKind :: COMMENT) } # [doc = " Returns true if this is an identifier or a keyword."] # [inline] pub fn is_any_identifier (self) -> bool { self == SyntaxKind :: IDENT || self . is_keyword (Edition :: LATEST) } }
    };
}

impl_88!()