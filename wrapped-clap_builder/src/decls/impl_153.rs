macro_rules! deps {
    () => {
        ValueRange!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl std :: ops :: RangeBounds < usize > for ValueRange { fn start_bound (& self) -> std :: ops :: Bound < & usize > { std :: ops :: Bound :: Included (& self . start_inclusive) } fn end_bound (& self) -> std :: ops :: Bound < & usize > { std :: ops :: Bound :: Included (& self . end_inclusive) } }
    };
}

impl_153!();