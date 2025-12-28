macro_rules! deps {
    () => {
        IntegerType!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl IntegerType { pub fn is_signed (& self) -> bool { match self { IntegerType :: Pointer (b) => * b , IntegerType :: Fixed (_ , b) => * b , } } }
    };
}

impl_51!()