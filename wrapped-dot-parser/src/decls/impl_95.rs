macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < A > Node < A > { fn map < F , B > (self , f : F) -> Node < B > where F : Fn (A) -> Option < B > , { Node { id : self . id , port : self . port , attr : self . attr . filter_map_attr (& f) , } } }
    };
}

impl_95!()