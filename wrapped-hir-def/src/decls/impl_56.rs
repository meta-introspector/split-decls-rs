macro_rules! deps {
    () => {
        ImportOrExternCrate!();
        ImportOrDef!();
        ExternCrate!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl From < ImportOrExternCrate > for ImportOrDef { fn from (value : ImportOrExternCrate) -> Self { match value { ImportOrExternCrate :: Import (it) => ImportOrDef :: Import (it) , ImportOrExternCrate :: Glob (it) => ImportOrDef :: Glob (it) , ImportOrExternCrate :: ExternCrate (it) => ImportOrDef :: ExternCrate (it) , } } }
    };
}

impl_56!()