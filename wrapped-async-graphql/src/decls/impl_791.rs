macro_rules! deps {
    () => {
        MaybeUndefined!();
    };
}

macro_rules! impl_791 {
    () => {
        deps!();
        impl < T > From < MaybeUndefined < T > > for Option < Option < T > > { fn from (maybe_undefined : MaybeUndefined < T >) -> Self { match maybe_undefined { MaybeUndefined :: Undefined => None , MaybeUndefined :: Null => Some (None) , MaybeUndefined :: Value (value) => Some (Some (value)) , } } }
    };
}

impl_791!();