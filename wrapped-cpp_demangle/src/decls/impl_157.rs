macro_rules! deps {
    () => {
        BuiltinType!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl TypeHandle { fn is_void (& self) -> bool { match * self { TypeHandle :: Builtin (BuiltinType :: Standard (StandardBuiltinType :: Void)) => true , _ => false , } } }
    };
}

impl_157!()