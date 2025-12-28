macro_rules! deps {
    () => {
        AnyValueId!();
    };
}

macro_rules! impl_607 {
    () => {
        deps!();
        impl PartialEq < std :: any :: TypeId > for AnyValueId { fn eq (& self , other : & std :: any :: TypeId) -> bool { self . type_id == * other } }
    };
}

impl_607!();