macro_rules! deps {
    () => {
        BuiltinType!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl fmt :: Display for BuiltinType { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { BuiltinType :: Char => f . write_str ("char") , BuiltinType :: Bool => f . write_str ("bool") , BuiltinType :: Str => f . write_str ("str") , BuiltinType :: Int (it) => it . fmt (f) , BuiltinType :: Uint (it) => it . fmt (f) , BuiltinType :: Float (it) => it . fmt (f) , } } }
    };
}

impl_41!();