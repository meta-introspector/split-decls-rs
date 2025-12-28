macro_rules! deps {
    () => {
        ComPtr!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl PartialEq for ComPtr { fn eq (& self , other : & Self) -> bool { self . cast (& IID_IUnknown) . unwrap () . 0 == other . cast (& IID_IUnknown) . unwrap () . 0 } }
    };
}

impl_44!();