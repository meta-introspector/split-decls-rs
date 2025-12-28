macro_rules! deps {
    () => {
        ParamsInScope!();
        ContainerKind!();
        Field!();
        Struct!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a > Struct < 'a > { fn from_syn (node : & 'a DeriveInput , data : & 'a DataStruct) -> Result < Self > { let mut attrs = attr :: get (& node . attrs) ? ; let scope = ParamsInScope :: new (& node . generics) ; let fields = Field :: multiple_from_syn (& data . fields , & scope) ? ; if let Some (display) = & mut attrs . display { let container = ContainerKind :: from_struct (data) ; display . expand_shorthand (& fields , container) ? ; } Ok (Struct { attrs , ident : node . ident . clone () , generics : & node . generics , fields , }) } }
    };
}

impl_8!();