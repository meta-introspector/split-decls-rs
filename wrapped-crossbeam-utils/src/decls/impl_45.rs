macro_rules! deps {
    () => {
        AtomicUnit!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl AtomicUnit { # [inline] fn load (& self , _order : Ordering) { } # [inline] fn store (& self , _val : () , _order : Ordering) { } # [inline] fn swap (& self , _val : () , _order : Ordering) { } # [inline] fn compare_exchange_weak (& self , _current : () , _new : () , _success : Ordering , _failure : Ordering ,) -> Result < () , () > { Ok (()) } }
    };
}

impl_45!()