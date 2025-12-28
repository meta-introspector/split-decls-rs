macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < T : PartialEq > PartialEq for Positioned < T > { fn eq (& self , other : & Self) -> bool { self . node == other . node } }
    };
}

impl_124!();