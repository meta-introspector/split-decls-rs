macro_rules! deps {
    () => {
        BaseContainer!();
        Interface!();
        InterfaceField!();
    };
}

macro_rules! impl_408 {
    () => {
        deps!();
        impl BaseContainer for Interface { type FieldType = InterfaceField ; # [inline] fn name (& self) -> & str { & self . name } fn graphql_type (& self) -> & str { "Interface" } # [inline] fn field (& self , name : & str) -> Option < & Self :: FieldType > { self . fields . get (name) } }
    };
}

impl_408!()