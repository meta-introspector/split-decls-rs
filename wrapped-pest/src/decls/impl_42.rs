macro_rules! deps {
    () => {
        Pair!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < 'i , R : PartialEq > PartialEq for Pair < 'i , R > { fn eq (& self , other : & Pair < 'i , R >) -> bool { Rc :: ptr_eq (& self . queue , & other . queue) && ptr :: eq (self . input , other . input) && self . start == other . start } }
    };
}

impl_42!()