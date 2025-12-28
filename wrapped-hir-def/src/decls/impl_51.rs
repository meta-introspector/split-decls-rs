macro_rules! deps {
    () => {
        ImportOrGlob!();
        ImportOrExternCrate!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl From < ImportOrGlob > for ImportOrExternCrate { fn from (value : ImportOrGlob) -> Self { match value { ImportOrGlob :: Glob (it) => ImportOrExternCrate :: Glob (it) , ImportOrGlob :: Import (it) => ImportOrExternCrate :: Import (it) , } } }
    };
}

impl_51!();