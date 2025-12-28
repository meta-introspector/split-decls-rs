macro_rules! deps {
    () => {
        FormatArgumentKind!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl FormatArgumentKind { pub fn ident (& self) -> Option < Ident > { match self { & Self :: Normal => None , & Self :: Named (id) => Some (id) , & Self :: Captured (id) => Some (id) , } } }
    };
}

impl_342!();