macro_rules! deps {
    () => {
        Enum!();
        Struct!();
        Adt!();
        Union!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl From < AdtId > for Adt { fn from (id : AdtId) -> Self { match id { AdtId :: StructId (it) => Adt :: Struct (it . into ()) , AdtId :: UnionId (it) => Adt :: Union (it . into ()) , AdtId :: EnumId (it) => Adt :: Enum (it . into ()) , } } }
    };
}

impl_20!()