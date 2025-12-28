macro_rules! deps {
    () => {
        DiffMode!();
    };
}

macro_rules! impl_316 {
    () => {
        deps!();
        impl DiffMode { pub fn is_rev (& self) -> bool { matches ! (self , DiffMode :: Reverse) } pub fn is_fwd (& self) -> bool { matches ! (self , DiffMode :: Forward) } }
    };
}

impl_316!()