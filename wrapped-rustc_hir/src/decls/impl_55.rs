macro_rules! deps {
    () => {
        MacroKinds!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl From < MacroKind > for MacroKinds { fn from (kind : MacroKind) -> Self { match kind { MacroKind :: Bang => Self :: BANG , MacroKind :: Attr => Self :: ATTR , MacroKind :: Derive => Self :: DERIVE , } } }
    };
}

impl_55!()