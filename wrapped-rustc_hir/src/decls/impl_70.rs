macro_rules! deps {
    () => {
        PerNS!();
        Namespace!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < T > :: std :: ops :: Index < Namespace > for PerNS < T > { type Output = T ; fn index (& self , ns : Namespace) -> & T { match ns { Namespace :: ValueNS => & self . value_ns , Namespace :: TypeNS => & self . type_ns , Namespace :: MacroNS => & self . macro_ns , } } }
    };
}

impl_70!()