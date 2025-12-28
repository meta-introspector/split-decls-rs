macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl From < DocLinkDef > for Definition { fn from (def : DocLinkDef) -> Self { match def { DocLinkDef :: ModuleDef (it) => it . into () , DocLinkDef :: Field (it) => it . into () , DocLinkDef :: SelfType (it) => it . into () , } } }
    };
}

impl_43!();