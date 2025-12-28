macro_rules! deps {
    () => {
        AList!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < A > AList < A > { pub (crate) fn filter_map_attr < B > (self , f : & dyn Fn (A) -> Option < B >) -> AList < B > { AList { elems : self . into_iter () . filter_map (f) . collect () , } } pub (crate) fn empty () -> Self { AList { elems : Vec :: new () } } # [cfg (feature = "display")] # [doc = " Returns `true` if the list of attributes is empty"] pub (crate) fn is_empty (& self) -> bool { self . elems . is_empty () } }
    };
}

impl_50!()