macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > PartialEq < & 'a T > for Bytes where Bytes : PartialEq < T > , { fn eq (& self , other : & & 'a T) -> bool { * self == * * other } }
    };
}

impl_104!()