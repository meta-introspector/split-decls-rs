macro_rules! deps {
    () => {
        IndexTime!();
    };
}

macro_rules! impl_793 {
    () => {
        deps!();
        impl PartialOrd for IndexTime { fn partial_cmp (& self , other : & IndexTime) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_793!()