macro_rules! deps {
    () => {
        Pairs!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'i , R : PartialEq > PartialEq for Pairs < 'i , R > { fn eq (& self , other : & Pairs < 'i , R >) -> bool { Rc :: ptr_eq (& self . queue , & other . queue) && ptr :: eq (self . input , other . input) && self . start == other . start && self . end == other . end } }
    };
}

impl_56!()