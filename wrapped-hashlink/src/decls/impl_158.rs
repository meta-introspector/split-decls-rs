macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < 'a , K > Clone for Iter < 'a , K > { # [inline] fn clone (& self) -> Iter < 'a , K > { Iter { iter : self . iter . clone () , } } }
    };
}

impl_158!()