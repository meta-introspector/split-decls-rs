macro_rules! deps {
    () => {
        IsNull!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < T > IsNull for * mut T { fn is_ptr_null (& self) -> bool { self . is_null () } }
    };
}

impl_63!()