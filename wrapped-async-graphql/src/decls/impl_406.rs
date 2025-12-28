macro_rules! deps {
    () => {
        Object!();
        Field!();
        BaseContainer!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        impl BaseContainer for Object { type FieldType = Field ; # [inline] fn name (& self) -> & str { & self . name } fn graphql_type (& self) -> & str { "Object" } # [inline] fn field (& self , name : & str) -> Option < & Self :: FieldType > { self . fields . get (name) } }
    };
}

impl_406!()