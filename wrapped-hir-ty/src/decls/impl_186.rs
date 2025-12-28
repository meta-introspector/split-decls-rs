macro_rules! deps {
    () => {
        Generics!();
    };
}

macro_rules! impl_186 {
    () => {
        deps!();
        impl < T > ops :: Index < T > for Generics where GenericParams : ops :: Index < T > , { type Output = < GenericParams as ops :: Index < T > > :: Output ; fn index (& self , index : T) -> & Self :: Output { & self . params [index] } }
    };
}

impl_186!()