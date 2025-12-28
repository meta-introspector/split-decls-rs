macro_rules! deps {
    () => {
        AnyValueId!();
    };
}

macro_rules! impl_604 {
    () => {
        deps!();
        impl PartialEq for AnyValueId { fn eq (& self , other : & Self) -> bool { self . type_id == other . type_id } }
    };
}

impl_604!();