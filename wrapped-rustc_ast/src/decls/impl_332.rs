macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl Type { pub fn add_offset (self , add : isize) -> Self { let offset = match self . offset { - 1 => add , x => add + x , } ; Self { size : self . size , kind : self . kind , child : self . child , offset } } }
    };
}

impl_332!()