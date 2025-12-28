macro_rules! deps {
    () => {
        Receiver!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl < C > PartialEq for Receiver < C > { fn eq (& self , other : & Self) -> bool { self . counter == other . counter } }
    };
}

impl_58!()