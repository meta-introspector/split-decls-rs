macro_rules! deps {
    () => {
        PerNS!();
        Namespace!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < T > :: std :: ops :: IndexMut < Namespace > for PerNS < T > { fn index_mut (& mut self , ns : Namespace) -> & mut T { match ns { Namespace :: ValueNS => & mut self . value_ns , Namespace :: TypeNS => & mut self . type_ns , Namespace :: MacroNS => & mut self . macro_ns , } } }
    };
}

impl_71!();