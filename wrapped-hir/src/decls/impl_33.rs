macro_rules! deps {
    () => {
        Adt!();
        Union!();
        Enum!();
        Struct!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl From < Adt > for GenericDefId { fn from (id : Adt) -> Self { match id { Adt :: Struct (it) => it . id . into () , Adt :: Union (it) => it . id . into () , Adt :: Enum (it) => it . id . into () , } } }
    };
}

impl_33!();