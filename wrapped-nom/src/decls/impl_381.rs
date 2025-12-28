macro_rules! deps {
    () => {
        NomRange!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        impl NomRange < usize > for usize { type Saturating = Range < usize > ; type Bounded = Range < usize > ; fn bounds (& self) -> (Bound < usize > , Bound < usize >) { (Bound :: Included (* self) , Bound :: Included (* self)) } fn contains (& self , item : & usize) -> bool { self == item } fn is_inverted (& self) -> bool { false } fn saturating_iter (& self) -> Self :: Saturating { 0 .. * self } fn bounded_iter (& self) -> Self :: Bounded { 0 .. * self } }
    };
}

impl_381!();