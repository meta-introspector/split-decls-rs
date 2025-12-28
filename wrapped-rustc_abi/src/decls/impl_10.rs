macro_rules! deps {
    () => {
        IntegerType!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl IntegerType { pub fn is_signed (& self) -> bool { match self { IntegerType :: Pointer (b) => * b , IntegerType :: Fixed (_ , b) => * b , } } }
    };
}

impl_10!()