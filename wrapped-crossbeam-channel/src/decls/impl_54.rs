macro_rules! deps {
    () => {
        Sender!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < C > PartialEq for Sender < C > { fn eq (& self , other : & Self) -> bool { self . counter == other . counter } }
    };
}

impl_54!();