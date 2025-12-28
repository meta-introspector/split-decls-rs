macro_rules! deps {
    () => {
        ImportOrGlob!();
        ImportOrDef!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl From < ImportOrGlob > for ImportOrDef { fn from (value : ImportOrGlob) -> Self { match value { ImportOrGlob :: Import (it) => ImportOrDef :: Import (it) , ImportOrGlob :: Glob (it) => ImportOrDef :: Glob (it) , } } }
    };
}

impl_57!()