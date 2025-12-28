macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < T > Ref < T > { # [doc = " Erase the type marker"] pub (super) fn erase (self) -> Ref < () > { Ref { index : self . index , _p : PhantomData , } } pub (super) fn ref_eq (self , other : Ref < T >) -> bool { self . index == other . index } }
    };
}

impl_88!();