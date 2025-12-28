macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl From < FieldId > for Field { fn from (def : FieldId) -> Self { Field { parent : def . parent . into () , id : def . local_id } } }
    };
}

impl_37!()