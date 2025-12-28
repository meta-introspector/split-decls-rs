macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl core :: ops :: Deref for PropName { type Target = CStr ; # [inline] fn deref (& self) -> & Self :: Target { self . as_c_str () } }
    };
}

impl_303!()