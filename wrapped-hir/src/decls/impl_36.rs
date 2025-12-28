macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl From < Field > for FieldId { fn from (def : Field) -> Self { FieldId { parent : def . parent . into () , local_id : def . id } } }
    };
}

impl_36!()