macro_rules! deps {
    () => {
        Time!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl PartialEq < FileTime > for Time { fn eq (& self , other : & FileTime) -> bool { * self == Time :: from (* other) } }
    };
}

impl_97!();