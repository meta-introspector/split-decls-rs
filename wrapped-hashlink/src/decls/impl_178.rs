macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < 'a , T , S > Clone for Union < 'a , T , S > { # [inline] fn clone (& self) -> Union < 'a , T , S > { Union { iter : self . iter . clone () , } } }
    };
}

impl_178!()