macro_rules! deps {
    () => {
        Attributes!();
    };
}

macro_rules! impl_349 {
    () => {
        deps!();
        impl PartialEq for Attributes { fn eq (& self , other : & Attributes) -> bool { * * self == * * other } }
    };
}

impl_349!()