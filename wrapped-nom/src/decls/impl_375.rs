macro_rules! deps {
    () => {
        NomRange!();
    };
}

macro_rules! impl_375 {
    () => {
        deps!();
        impl NomRange < usize > for Range < usize > { type Saturating = Range < usize > ; type Bounded = Range < usize > ; fn bounds (& self) -> (Bound < usize > , Bound < usize >) { (Bound :: Included (self . start) , Bound :: Excluded (self . end)) } fn contains (& self , item : & usize) -> bool { RangeBounds :: contains (self , item) } fn is_inverted (& self) -> bool { self . start >= self . end } fn saturating_iter (& self) -> Self :: Saturating { if self . end == 0 { Range :: default () } else { 0 .. self . end - 1 } } fn bounded_iter (& self) -> Self :: Bounded { if self . end == 0 { Range :: default () } else { 0 .. self . end - 1 } } }
    };
}

impl_375!()