macro_rules! deps {
    () => {
        MemberUnraw!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl MemberUnraw { pub fn span (& self) -> Span { match self { MemberUnraw :: Named (ident) => ident . 0 . span () , MemberUnraw :: Unnamed (index) => index . span , } } }
    };
}

impl_105!();