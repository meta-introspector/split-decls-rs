macro_rules! deps {
    () => {
        Ptr!();
    };
}

macro_rules! impl_913 {
    () => {
        deps!();
        impl < 'b , T > PartialEq for Ptr < 'b , T > { # [doc = " Ptr compares by pointer equality, i.e if they point to the same value"] fn eq (& self , other : & Ptr < 'b , T >) -> bool { ptr_eq (self . 0 , other . 0) } }
    };
}

impl_913!();