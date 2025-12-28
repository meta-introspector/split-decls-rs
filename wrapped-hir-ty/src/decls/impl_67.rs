macro_rules! deps {
    () => {
        AutoBorrow!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl AutoBorrow { fn mutability (self) -> Mutability { match self { AutoBorrow :: Ref (mutbl) => mutbl . into () , AutoBorrow :: RawPtr (mutbl) => mutbl , } } }
    };
}

impl_67!()