macro_rules! deps {
    () => {
        Adt!();
        Struct!();
        Union!();
        Enum!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < Adt > for AdtId { fn from (id : Adt) -> Self { match id { Adt :: Struct (it) => AdtId :: StructId (it . id) , Adt :: Union (it) => AdtId :: UnionId (it . id) , Adt :: Enum (it) => AdtId :: EnumId (it . id) , } } }
    };
}

impl_21!();