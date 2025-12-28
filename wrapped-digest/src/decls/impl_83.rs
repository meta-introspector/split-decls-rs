macro_rules! deps {
    () => {
        XofFixedWrapper!();
        Update!();
        ExtendableOutput!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < T : ExtendableOutput + Update , S : ArraySize > Update for XofFixedWrapper < T , S > { fn update (& mut self , data : & [u8]) { self . hash . update (data) } }
    };
}

impl_83!();