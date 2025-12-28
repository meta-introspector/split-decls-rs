macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl PartialEq for Interned < str > { fn eq (& self , other : & Self) -> bool { Arc :: ptr_eq (& self . arc , & other . arc) } }
    };
}

impl_35!()