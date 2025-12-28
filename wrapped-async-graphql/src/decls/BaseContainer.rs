macro_rules! deps {
    () => {
        BaseField!();
    };
}

macro_rules! BaseContainer {
    () => {
        deps!();
        pub (crate) trait BaseContainer { type FieldType : BaseField ; fn name (& self) -> & str ; fn graphql_type (& self) -> & str ; fn field (& self , name : & str) -> Option < & Self :: FieldType > ; }
    };
}

BaseContainer!();