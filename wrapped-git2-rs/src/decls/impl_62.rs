macro_rules! deps {
    () => {
        IsNull!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl < T > IsNull for * const T { fn is_ptr_null (& self) -> bool { self . is_null () } }
    };
}

impl_62!();