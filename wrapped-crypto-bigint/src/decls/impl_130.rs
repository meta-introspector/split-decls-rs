macro_rules! deps {
    () => {
        JacobiSymbol!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl PartialEq for JacobiSymbol { fn eq (& self , other : & Self) -> bool { bool :: from (self . ct_eq (other)) } }
    };
}

impl_130!()