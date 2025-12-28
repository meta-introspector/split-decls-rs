macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < 'a , K , V > Clone for Keys < 'a , K , V > { # [inline] fn clone (& self) -> Keys < 'a , K , V > { Keys { inner : self . inner . clone () , } } }
    };
}

impl_88!();