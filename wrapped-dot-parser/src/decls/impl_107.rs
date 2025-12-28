macro_rules! deps {
    () => {
        Edge!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < A > Edge < A > { fn map < F , B > (self , f : F) -> Edge < B > where F : Fn (A) -> Option < B > , { Edge { from : self . from , to : self . to , attr : self . attr . filter_map_attr (& f) , } } }
    };
}

impl_107!()