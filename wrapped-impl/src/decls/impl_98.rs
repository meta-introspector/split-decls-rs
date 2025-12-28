macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl PartialEq for IdentUnraw { fn eq (& self , other : & Self) -> bool { PartialEq :: eq (& self . 0 . unraw () , & other . 0 . unraw ()) } }
    };
}

impl_98!()