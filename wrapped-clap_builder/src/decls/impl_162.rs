macro_rules! deps {
    () => {
        Result!();
        ValueRange!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl std :: fmt :: Display for ValueRange { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { ok ! (self . start_inclusive . fmt (f)) ; if self . is_fixed () { } else if self . end_inclusive == usize :: MAX { ok ! (".." . fmt (f)) ; } else { ok ! ("..=" . fmt (f)) ; ok ! (self . end_inclusive . fmt (f)) ; } Ok (()) } }
    };
}

impl_162!();