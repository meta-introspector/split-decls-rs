macro_rules! deps {
    () => {
        StoreConstEmpty!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < K , V > StoreConstEmpty < K , V > for Vec < (K , V) > { const EMPTY : Vec < (K , V) > = Vec :: new () ; }
    };
}

impl_77!()