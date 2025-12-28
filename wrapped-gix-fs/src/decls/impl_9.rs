macro_rules! deps {
    () => {
        FileSnapshot!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < T : Clone + std :: fmt :: Debug > Clone for FileSnapshot < T > { fn clone (& self) -> Self { Self { value : self . value . clone () , modified : self . modified , } } }
    };
}

impl_9!();