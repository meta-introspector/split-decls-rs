macro_rules! deps {
    () => {
        DefWithBody!();
        Variant!();
        Function!();
        Const!();
        Static!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < DefWithBody > for DefWithBodyId { fn from (def : DefWithBody) -> Self { match def { DefWithBody :: Function (it) => DefWithBodyId :: FunctionId (it . id) , DefWithBody :: Static (it) => DefWithBodyId :: StaticId (it . id) , DefWithBody :: Const (it) => DefWithBodyId :: ConstId (it . id) , DefWithBody :: Variant (it) => DefWithBodyId :: VariantId (it . into ()) , } } }
    };
}

impl_28!()