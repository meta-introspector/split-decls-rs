macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl TypeRef { pub fn type_name (& self) -> TypeName { TypeName (self . namespace () , self . name ()) } pub fn name (& self) -> & 'static str { trim_tick (self . str (1)) } pub fn namespace (& self) -> & 'static str { self . str (2) } }
    };
}

impl_108!()