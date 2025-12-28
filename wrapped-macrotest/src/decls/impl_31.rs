macro_rules! deps {
    () => {
        Dependency!();
        Result!();
        Error!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Serialize for Dependency { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { Dependency :: serialize (self , serializer) } }
    };
}

impl_31!()