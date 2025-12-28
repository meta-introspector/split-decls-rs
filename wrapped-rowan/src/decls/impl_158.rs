macro_rules! deps {
    () => {
        ThinArc!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < H : PartialEq , T : PartialEq > PartialEq for ThinArc < H , T > { # [inline] fn eq (& self , other : & ThinArc < H , T >) -> bool { * * self == * * other } }
    };
}

impl_158!();