macro_rules! deps {
    () => {
        Const!();
        Variant!();
        Static!();
        DefWithBody!();
        Function!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl From < DefWithBodyId > for DefWithBody { fn from (def : DefWithBodyId) -> Self { match def { DefWithBodyId :: FunctionId (it) => DefWithBody :: Function (it . into ()) , DefWithBodyId :: StaticId (it) => DefWithBody :: Static (it . into ()) , DefWithBodyId :: ConstId (it) => DefWithBody :: Const (it . into ()) , DefWithBodyId :: VariantId (it) => DefWithBody :: Variant (it . into ()) , } } }
    };
}

impl_29!();