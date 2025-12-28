macro_rules! deps {
    () => {
        PropertyName!();
        PropName!();
    };
}

macro_rules! impl_306 {
    () => {
        deps!();
        impl std :: borrow :: ToOwned for PropName { type Owned = PropertyName ; # [inline] fn to_owned (& self) -> Self :: Owned { PropertyName (self . 0 . to_owned ()) } # [inline] fn clone_into (& self , target : & mut Self :: Owned) { self . 0 . clone_into (& mut target . 0) ; } }
    };
}

impl_306!();