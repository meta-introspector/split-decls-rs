macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl digest :: Update for Hasher { # [inline] fn update (& mut self , data : & [u8]) { self . update (data) ; } }
    };
}

impl_126!();