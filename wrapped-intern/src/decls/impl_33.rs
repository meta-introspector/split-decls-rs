macro_rules! deps {
    () => {
        Internable!();
        Interned!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        # [doc = " Compares interned `Ref`s using pointer equality."] impl < T : Internable > PartialEq for Interned < T > { # [inline] fn eq (& self , other : & Self) -> bool { Arc :: ptr_eq (& self . arc , & other . arc) } }
    };
}

impl_33!();