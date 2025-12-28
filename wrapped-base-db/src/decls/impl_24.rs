macro_rules! deps {
    () => {
        CrateOrigin!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl CrateOrigin { pub fn is_local (& self) -> bool { matches ! (self , CrateOrigin :: Local { .. }) } pub fn is_lib (& self) -> bool { matches ! (self , CrateOrigin :: Library { .. }) } pub fn is_lang (& self) -> bool { matches ! (self , CrateOrigin :: Lang { .. }) } }
    };
}

impl_24!()